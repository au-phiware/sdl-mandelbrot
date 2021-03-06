extern crate num_complex;
extern crate num_traits;
extern crate palette;
extern crate sdl2;

use exit;
use num_complex::Complex64;
use num_traits::identities::Zero;
use palette::{rgb::Rgb, Lch};
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
    sync::mpsc::{channel, SendError},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

const DIV_LIMIT: f64 = 400f64;
const FIXED_THRESHOLD: u32 = 20000;
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
        const N: usize = 32;
        let (results_tx, results_rx) = channel();
        let (mut workers, mut worker_handles) = (Vec::with_capacity(N), Vec::with_capacity(N));
        for _ in 0..N {
            let results = results_tx.clone();
            let (worker, port) = channel();
            let handle: JoinHandle<Result<_, SendError<_>>> = thread::spawn(move || {
                while let Ok((idx, c)) = port.recv() {
                    let n = compute_orbit(c, None);
                    results.send((idx, n))?;
                }
                Ok(())
            });
            workers.push(worker);
            worker_handles.push(handle);
        }
        drop(results_tx);

        {
            let mut c = Complex64 { re: 0., im: 0. };
            let w = self.w as i32;
            let h = self.h as i32;
            let res = self.res;
            let mut i = 0;
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
                                workers[i % N].send((idx, c)).unwrap();
                                i = i.wrapping_add(1);
                            }
                        } else {
                            self.pixels[idx] = 255;
                        }
                    }
                }
            }
        }
        drop(workers);

        while let Ok((idx, n)) = results_rx.recv() {
            self.pixels[idx] = n.map(|n| (n % 254 + 1) as u8).unwrap_or(255);
        }
        for w in worker_handles.into_iter() {
            w.join().unwrap().unwrap();
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
        surface.set_palette(&Palette::with_colors(&PALETTE)?)?;
        let texture = texture_creator
            .create_texture_from_surface(surface)
            .map_err(|e| e.to_string())?;
        window.copy(&texture, None, None)?;

        window.set_draw_color(PALETTE[0]);
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
    PALETTE[0] = Color::RGBA(0, 0, 0, 0xFF);
    for i in 1..255 {
        PALETTE[i] = {
            let rgb: Rgb = Lch::new(75., 100., (i - 1) as f32 * 360. / 254.).into();
            let (r, g, b) = rgb.into();
            Color::RGBA(
                (0xff as f32 * r) as u8,
                (0xff as f32 * g) as u8,
                (0xff as f32 * b) as u8,
                0xFF,
            )
        }
    }
    PALETTE[255] = Color::RGBA(0xff, 0xff, 0xff, 0xFF);
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

const PALETTE: [Color; 256] = [Color {
    r: 0,
    g: 0,
    b: 0,
    a: 0,
}; 256];
