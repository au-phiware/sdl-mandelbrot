extern crate num_complex;
extern crate num_traits;
extern crate palette;
extern crate sdl2;

use arr_macro::arr;
use exit;
use num_complex::Complex64;
use num_traits::identities::Zero;
use palette::{rgb::Rgb, Lch, Pixel, Srgb};
use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    mouse::MouseButton,
    pixels::{Color, Palette, PixelFormatEnum},
    rect::Point,
    render::Canvas,
    surface::Surface,
    video::Window,
};
use std::{
    thread,
    time::{Duration, Instant},
};

const DIV_LIMIT: f64 = 400f64;
const FIXED_THRESHOLD: u32 = 2540;
const INITIAL_RES: i32 = 11;
const FRAME_SPACER: Duration = Duration::from_millis(30);

struct Image {
    pixels: Vec<u8>,
    w: u32,
    h: u32,
    res: i32,
    tx: Complex64,
    tn: Complex64,

    orbit: Vec<Complex64>,
    c: Option<Complex64>,

    palette: [Color; 256],
}

fn compute_orbit(c: Complex64, mut orbit: Option<&mut Vec<Complex64>>) -> Option<u32> {
    let mut z = c.clone();
    let mut m = z.norm_sqr();
    let mut n = 0;
    while m < DIV_LIMIT {
        z = z * z + c;
        m = z.norm_sqr();
        n += 1;
        if let Some(ref mut orbit) = orbit {
            orbit.push(z.clone());
        }
        if n == FIXED_THRESHOLD {
            return None;
        }
    }
    Some(n)
}

impl Image {
    fn new(w: u32, h: u32) -> Self {
        Image {
            pixels: Vec::new(),
            w,
            h,
            res: INITIAL_RES,
            tx: Complex64 { re: 1., im: 0. },
            tn: Complex64 { re: 0., im: 0. },

            orbit: Vec::<Complex64>::new(),
            c: None,

            palette: {
                let mut i = 0;
                arr![{
                    let c = if 0 < i && i < 255 {
                        let lch = Lch::new(75., 100., i as f32 * 360. / 254.);
                        let rgb: Rgb = Srgb::from_linear(lch.into());
                        let parts: [u8; 3] = rgb.into_format().into_raw();
                        Color::RGB(parts[0], parts[1], parts[2])
                    } else {
                        Color::RGB(0, 0, 0)
                    };
                    i += 1;
                    c
                }; 256]
            },
        }
    }

    fn clear(&mut self) {
        self.pixels.clear();
        self.pixels
            .resize_with((self.w * self.h) as usize, Default::default);
    }

    fn transform(&self, x: Complex64) -> Complex64 {
        x * self.tx + self.tn
    }

    fn transform_inv(&self, x: Complex64) -> Option<Complex64> {
        if (&self.tx).is_zero() {
            None
        } else {
            Some((x - self.tn) / self.tx)
        }
    }

    fn translate(&mut self, tn: Complex64) {
        self.tn += tn;
    }

    fn scale(&mut self, scale: f64) {
        self.tx *= Complex64 { re: scale, im: 0. };
    }

    fn compute(&mut self) {
        let mut c = Complex64 { re: 0., im: 0. };
        let w = self.w as i32;
        let h = self.h as i32;
        let res = self.res;
        for y in ((res - 1) / 2..h).step_by(res as usize) {
            for x in ((res - 1) / 2..w).step_by(res as usize) {
                let idx = (y * w + x) as usize;
                if self.pixels[idx] == 0 {
                    c.re = x as f64;
                    c.im = y as f64;
                    c = c * self.tx + self.tn;
                    if !(4. * (c + 1.).norm() < 1. || {
                        let (r, t) = (c - 0.25).to_polar();
                        2. * r < 1. - t.cos()
                    }) {
                        if idx < self.pixels.len() {
                            let n = compute_orbit(c, None);
                            self.pixels[idx] = n.map(|n| (n % 254 + 1) as u8).unwrap_or(255);
                        }
                    }
                }
            }
        }
    }

    fn draw(&mut self, window: &mut Canvas<Window>, p: Option<Complex64>) -> Result<(), String> {
        let texture_creator = window.texture_creator();

        if self.res > 0 {
            self.compute();
        }

        let mut pixels = self.pixels.clone();
        if self.res > 1 {
            let w = self.w as i32;
            let h = self.h as i32;
            let res = self.res;
            for x in ((res - 1) / 2..w).step_by(res as usize) {
                for y in ((res - 1) / 2..h).step_by(res as usize) {
                    let n = pixels[(y * w + x) as usize];
                    for i in -((res - 1) / 2)..=((res - 1) / 2) {
                        for j in -((res - 1) / 2)..=((res - 1) / 2) {
                            let idx = (y + j) * w + x + i;
                            if idx >= 0 && idx < w * h {
                                pixels[idx as usize] = n;
                            }
                        }
                    }
                }
            }
        }
        let mut surface =
            Surface::from_data(&mut pixels, self.w, self.h, self.w, PixelFormatEnum::Index8)?;
        surface.set_palette(&Palette::with_colors(&self.palette)?)?;
        let texture = texture_creator
            .create_texture_from_surface(surface)
            .map_err(|e| e.to_string())?;
        window.copy(&texture, None, None)?;

        window.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        if self.c != p {
            self.c = p;
            if let Some(p) = p {
                self.orbit.clear();
                self.orbit.push(Complex64 { re: 0., im: 0. });
                self.orbit.push(p);
                compute_orbit(p, Some(&mut self.orbit));
            }
        }
        if self.c.is_some() {
            window.draw_lines::<&[Point]>(
                self.orbit
                    .iter()
                    .filter_map(|&c| {
                        self.transform_inv(c)
                            .and_then(|a| Some(Point::new(a.re as i32, a.im as i32)))
                    })
                    .collect::<Vec<_>>()
                    .as_slice(),
            )?;
        }

        Result::Ok(())
    }
}

pub fn main() -> exit::Result {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let (mut w, mut h) = (600, 600);
    let mut image = &mut Image::new(w, h);
    let mut locus_x = 0;
    let mut locus_y = 0;
    let mut trace_orbit = false;
    let mut pin_orbit = false;
    image.scale(2. / 300.);
    image.translate(Complex64 { re: -2., im: -2. });

    let window = video_subsystem
        .window("Mandelbrot", image.w, image.h)
        .position_centered()
        .resizable()
        .build()?;
    let mut canvas = window.into_canvas().accelerated().build()?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut p: Option<Complex64> = None;
    'running: loop {
        let start = Instant::now();
        if image.res == INITIAL_RES {
            image.clear();
        }

        if image.res > 0 || trace_orbit && p != image.c || !trace_orbit && !image.c.is_none() {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();

            image.draw(&mut canvas, if trace_orbit { p } else { None })?;

            canvas.present();
            if image.res > 0 {
                image.res -= 2;
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
                    let preserve_width = (width as f64) / (w as f64);
                    let preserve_height = (height as f64) / (h as f64);
                    image.scale(
                        1. / if preserve_width < preserve_height {
                            w = width as u32;
                            h = (h as f64 * preserve_width) as u32;
                            preserve_width
                        } else {
                            w = (w as f64 * preserve_height) as u32;
                            h = height as u32;
                            preserve_height
                        },
                    );
                    image.w = width as u32;
                    image.h = height as u32;
                    image.res = INITIAL_RES;
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
                        w = image.w;
                        h = image.h;
                        image.translate(
                            Complex64 {
                                re: -xrel as f64,
                                im: -yrel as f64,
                            } * image.tx.norm(),
                        );
                        image.res = INITIAL_RES;
                    }
                    if !pin_orbit {
                        p = Some(image.transform(Complex64 {
                            re: x as f64,
                            im: y as f64,
                        }));
                    }
                }
                Event::MouseWheel { which: 0, y: n, .. } => {
                    w = image.w;
                    h = image.h;
                    image.translate(
                        Complex64 {
                            re: locus_x as f64,
                            im: locus_y as f64,
                        } * image.tx.norm(),
                    );
                    image.scale(1.1f64.powi(n));
                    image.translate(
                        Complex64 {
                            re: -locus_x as f64,
                            im: -locus_y as f64,
                        } * image.tx.norm(),
                    );
                    image.res = INITIAL_RES;
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
