use crate::compute::compute_orbit;
use crate::projection::*;
use arr_macro::arr;
use num_complex::Complex;
use num_traits::{AsPrimitive, Float};
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
    pub projection: Projection<Complex<Source<f64>>>,
    size: Size,
    drawn_state: Option<(Projection<Complex<Source<f64>>>, Size)>,

    pixels: Vec<u8>,
    res: i32,

    orbit: Vec<Complex<Source<f64>>>,
    c: Option<Complex<Source<f64>>>,

    palette: [Color; 256],
}

impl Image {
    pub fn new(size: Size) -> Self {
        Image {
            pixels: Vec::new(),
            size,
            res: INITIAL_RES,
            projection: Projection::<Complex<Source<f64>>>::default(),
            drawn_state: None,

            orbit: Vec::<Complex<Source<f64>>>::new(),
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

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.size.w = width;
        self.size.h = height;
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
            let w = self.size.w as i32;
            let h = self.size.h as i32;
            let res = self.res;
            let mut i = 0;
            for y in ((res - 1) / 2..h).step_by(res as usize) {
                for x in ((res - 1) / 2..w).step_by(res as usize) {
                    let idx = (y * w + x) as usize;
                    if self.pixels[idx] == 0 {
                        let c = self.projection.transform(Complex::<Projected<f64>> {
                            re: Projected(x as f64),
                            im: Projected(y as f64),
                        });
                        if !(Source(4.) * (c + Source(1.)).norm() < Source(1.) || {
                            let (r, t) = (c - Source(0.25)).to_polar();
                            Source(2.) * r < Source(1.) - t.cos()
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

    pub fn needs_draw(&self, p: Option<Complex<Source<f64>>>) -> bool {
        self.res > 0 || p != self.c || self.drawn_state != Some((self.projection, self.size))
    }

    pub fn draw(
        &mut self,
        window: &mut Canvas<Window>,
        p: Option<Complex<Source<f64>>>,
    ) -> Result<(), String> {
        let texture_creator = window.texture_creator();

        // Check if pixels can be reused
        if self.drawn_state != Some((self.projection, self.size)) {
            self.clear();
            self.res = INITIAL_RES;
            self.drawn_state = Some((self.projection, self.size));
        }
        // Calculate pixel values
        if self.res > 0 {
            self.compute();
        }

        // Prepare pixels for texture
        let mut pixels = self.pixels.clone();
        // Pixelate over missing pixels
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

        // Increase detail
        if self.res > 0 {
            self.res -= 2;
        }

        // Prepare surface and copy to window
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

        // Draw orbit trace
        window.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        if self.c != p {
            self.c = p;
            if let Some(p) = p {
                self.orbit.clear();
                self.orbit.push(Complex::<Source<f64>>::default());
                self.orbit.push(p);
                compute_orbit(p, Some(&mut self.orbit));
            }
        }
        if self.c.is_some() {
            window.draw_lines::<&[Point]>(
                self.orbit
                    .iter()
                    .filter_map(|&c| {
                        let p: Complex<Projected<f64>> = self.projection.transform(c);
                        if p.is_finite() {
                            Some(Point::new(p.re.as_(), p.im.as_()))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                    .as_slice(),
            )?;
        }

        Result::Ok(())
    }
}
