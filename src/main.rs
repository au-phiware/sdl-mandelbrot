#![feature(type_ascription)]
#![feature(step_trait)]

extern crate num_complex;
extern crate num_traits;
#[macro_use]
extern crate num_derive;
extern crate palette;
extern crate sdl2;

use crate::image::*;
use crate::projection::{MutProjector, Projected, Projector, Source, Value::*};
use core::f64::consts::PI;
use exit;
use num_complex::Complex;
use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    mouse::MouseButton,
    pixels::Color,
};
use std::{
    thread,
    time::{Duration, Instant},
};

mod compute;
mod image;
#[macro_use]
mod projection;

const FRAME_SPACER: Duration = Duration::from_millis(15);

pub fn main() -> exit::Result {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let mut size = Size { w: 600, h: 600 };
    let mut image = Image::new(size);
    let mut locus_x = 0;
    let mut locus_y = 0;
    let mut trace_orbit = false;
    let mut pin_orbit = false;
    image.projection.set_transform(Absolute(
        Complex::<Source<f64>> {
            re: Source(-2. / 300.),
            im: Source(0.),
        } * Complex::<Source<f64>>::from_polar(&Source(1.), &Source(PI)),
    ));
    image
        .projection
        .set_translate(Absolute(Complex::<Source<f64>> {
            re: Source(-2.),
            im: Source(-2.),
        }));

    let window = video_subsystem
        .window("Mandelbrot", size.w, size.h)
        .position_centered()
        .resizable()
        .build()?;
    let mut canvas = window.into_canvas().accelerated().build()?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut p: Option<Complex<Source<f64>>> = None;
    'running: loop {
        let start = Instant::now();
        {
            let c = if trace_orbit { p } else { None };
            if image.needs_draw(c) {
                canvas.set_draw_color(Color::RGB(0, 0, 0));
                canvas.clear();

                image.draw(&mut canvas, c)?;

                canvas.present();
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyUp {
                    keycode: Some(Keycode::T),
                    ..
                } => trace_orbit = !trace_orbit,
                Event::KeyUp {
                    keycode: Some(Keycode::P),
                    ..
                } => pin_orbit = !pin_orbit,
                Event::Window {
                    win_event: WindowEvent::Resized(width, height),
                    ..
                } => {
                    let preserve_width = (size.w as f64) / (width as f64);
                    let preserve_height = (size.h as f64) / (height as f64);
                    let scale = if preserve_width < preserve_height {
                        size.w = (size.w as f64 * preserve_height) as u32;
                        size.h = height as u32;
                        preserve_height
                    } else {
                        size.w = width as u32;
                        size.h = (size.h as f64 * preserve_width) as u32;
                        preserve_width
                    };
                    image.projection.set_transform(Relative(
                        Source(Complex { re: scale, im: 0. }).into(): Complex<Source<_>>,
                    ));
                    let offset = Complex {
                        re: (width as f64) - (image.size().w as f64) / scale,
                        im: (height as f64) - (image.size().h as f64) / scale,
                    } / -2.;
                    image
                        .projection
                        .set_translate(Relative(Projected(offset).into(): Complex<Projected<_>>));
                    image.set_size(width as u32, height as u32);
                }
                Event::MouseMotion {
                    x,
                    y,
                    xrel,
                    yrel,
                    mousestate,
                    ..
                } => {
                    locus_x = x;
                    locus_y = y;
                    if mousestate.left() {
                        size = image.size();
                        image
                            .projection
                            .set_translate(Relative(Complex::<Projected<f64>> {
                                re: Projected(-xrel as f64),
                                im: Projected(-yrel as f64),
                            }));
                    }
                    if !pin_orbit {
                        p = Some(image.projection.transform(Complex::<Projected<f64>> {
                            re: Projected(x as f64),
                            im: Projected(y as f64),
                        }));
                    }
                }
                Event::MouseWheel { which: 0, y: n, .. } => {
                    size = image.size();
                    let focus = Complex::<Projected<f64>> {
                        re: Projected(locus_x as f64),
                        im: Projected(locus_y as f64),
                    };
                    image.projection.set_translate(Relative(focus));
                    image
                        .projection
                        .set_transform(Relative(Complex::<Source<f64>> {
                            re: Source(1.1f64.powi(n)),
                            im: Source(0.),
                        }));
                    image.projection.set_translate(Relative(-focus));
                }
                Event::MouseButtonUp { mouse_btn, .. } => match mouse_btn {
                    MouseButton::Right => {
                        p = None;
                    }
                    _ => {}
                },
                _ => {}
            }
            thread::yield_now();
        }

        if start.elapsed() < FRAME_SPACER {
            thread::yield_now();
            let space = FRAME_SPACER - start.elapsed();
            if space.as_millis() > 0 {
                thread::sleep(space);
            }
        }
    }

    exit::Result::Ok
}
