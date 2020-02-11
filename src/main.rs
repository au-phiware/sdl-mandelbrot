extern crate num_complex;
extern crate num_traits;
extern crate palette;
extern crate sdl2;

use crate::image::*;
use exit;
use num_complex::Complex64;
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
    image.set_scale(2. / 300., Complex64 { re: 0., im: 0. });
    image.set_translate(Complex64 { re: -2., im: -2. });

    let window = video_subsystem
        .window("Mandelbrot", size.w, size.h)
        .position_centered()
        .resizable()
        .build()?;
    let mut canvas = window.into_canvas().accelerated().build()?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut p: Option<Complex64> = None;
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
                    win_event: WindowEvent::SizeChanged(width, height),
                    ..
                }
                | Event::Window {
                    win_event: WindowEvent::Resized(width, height),
                    ..
                } => {
                    let preserve_width = (width as f64) / (size.w as f64);
                    let preserve_height = (height as f64) / (size.h as f64);
                    image.set_scale(
                        1. / if preserve_width < preserve_height {
                            size.w = width as u32;
                            size.h = (size.h as f64 * preserve_width) as u32;
                            preserve_width
                        } else {
                            size.w = (size.w as f64 * preserve_height) as u32;
                            size.h = height as u32;
                            preserve_height
                        },
                        image.scale(Complex64 {
                            re: (size.w / 2) as f64,
                            im: (size.h / 2) as f64,
                        }),
                    );
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
                        image.set_translate(image.scale(Complex64 {
                            re: -xrel as f64,
                            im: -yrel as f64,
                        }));
                    }
                    if !pin_orbit {
                        p = Some(image.transform(Complex64 {
                            re: x as f64,
                            im: y as f64,
                        }));
                    }
                }
                Event::MouseWheel { which: 0, y: n, .. } => {
                    size = image.size();
                    image.set_scale(
                        1.1f64.powi(n),
                        Complex64 {
                            re: locus_x as f64,
                            im: locus_y as f64,
                        },
                    );
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
