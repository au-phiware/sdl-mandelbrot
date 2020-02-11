use crate::compute::compute_orbit;
use arr_macro::arr;
use num_complex::Complex64;
use num_traits::identities::Zero;
use palette::{rgb::Rgb, Lch, Pixel, Srgb};
use sdl2::{
    pixels::{Color, Palette, PixelFormatEnum},
    rect::Point,
    render::Canvas,
    surface::Surface,
    video::Window,
};
use std::{
    sync::mpsc::{channel, SendError},
    thread::{self, JoinHandle},
};

const INITIAL_RES: i32 = 11;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, PartialOrd, Ord)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}

pub struct Image {
    pixels: Vec<u8>,
    size: Size,
    res: i32,
    tx: Complex64,
    tn: Complex64,

    orbit: Vec<Complex64>,
    c: Option<Complex64>,

    palette: [Color; 256],
}

impl Image {
    pub fn new(size: Size) -> Self {
        Image {
            pixels: Vec::new(),
            size,
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

    pub fn clear(&mut self) {
        self.pixels.clear();
        self.pixels
            .resize_with((self.size.w * self.size.h) as usize, Default::default);
    }

    pub fn transform(&self, x: Complex64) -> Complex64 {
        x * self.tx + self.tn
    }

    pub fn transform_inv(&self, x: Complex64) -> Option<Complex64> {
        if (&self.tx).is_zero() {
            None
        } else {
            Some((x - self.tn) / self.tx)
        }
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.size.w = width;
        self.size.h = height;
        self.res = INITIAL_RES;
    }

    pub fn set_translate(&mut self, tn: Complex64) {
        self.tn += tn;
        self.res = INITIAL_RES;
    }

    pub fn scale(&self, c: Complex64) -> Complex64 {
        c * self.tx.norm()
    }

    pub fn set_scale(&mut self, scale: f64, focus: Complex64) {
        self.set_translate(-focus);
        self.tx *= Complex64 { re: scale, im: 0. };
        self.set_translate(focus);
        self.res = INITIAL_RES;
    }

    pub fn compute(&mut self) {
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
            let w = self.size.w as i32;
            let h = self.size.h as i32;
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

    pub fn needs_draw(&self, p: Option<Complex64>) -> bool {
        self.res > 0 || p != self.c
    }

    pub fn draw(
        &mut self,
        window: &mut Canvas<Window>,
        p: Option<Complex64>,
    ) -> Result<(), String> {
        let texture_creator = window.texture_creator();

        if self.res == INITIAL_RES {
            self.clear();
        } else if self.res > 0 {
            self.compute();
        }

        let mut pixels = self.pixels.clone();
        if self.res > 1 {
            let w = self.size.w as i32;
            let h = self.size.h as i32;
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
        let mut surface = Surface::from_data(
            &mut pixels,
            self.size.w,
            self.size.h,
            self.size.w,
            PixelFormatEnum::Index8,
        )?;
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

        if self.res > 0 {
            self.res -= 2;
        }

        Result::Ok(())
    }
}
