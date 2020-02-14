use crate::compute::compute_orbit;
use crate::projection::*;
use arr_macro::arr;
use num_complex::Complex;
use num_traits::{Float, ToPrimitive};
use palette::{rgb::Rgb, Lch, Pixel as PalettePixel, Srgb};
use sdl2::{
    pixels::{Color, Palette, PixelFormatEnum},
    rect::Point,
    render::Canvas,
    surface::Surface,
    video::Window,
};
use std::{
    convert::TryFrom,
    iter::Step,
    sync::mpsc::{channel, SendError},
    thread::{self, JoinHandle},
};

const INITIAL_RES: usize = 11;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, PartialOrd, Ord)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    ToPrimitive,
    FromPrimitive,
    NumOps,
    NumCast,
    One,
    Zero,
    Num,
)]
pub struct Pixel(i32);

pub type Real = f64;

pub type Value = Complex<Source<Real>>;

impl From<Pixel> for Real {
    fn from(p: Pixel) -> Real {
        p.0 as Real
    }
}

impl TryFrom<f64> for Pixel {
    type Error = &'static str;
    fn try_from(v: f64) -> Result<Self, Self::Error> {
        let v = v.round();
        if v.is_finite() && (std::i32::MIN as f64) < v && v < (std::i32::MAX as f64) {
            Ok(Pixel(v as i32))
        } else {
            Err("out of range float type conversion attempted")
        }
    }
}

impl Into<Point> for Projected<Complex<Pixel>> {
    fn into(self) -> Point {
        Point::new((self.0).re.0, (self.0).im.0)
    }
}

impl Step for Pixel {
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        i32::steps_between(&start.0, &end.0)
    }
    fn replace_one(&mut self) -> Self {
        Self(self.0.replace_one())
    }
    fn replace_zero(&mut self) -> Self {
        Self(self.0.replace_zero())
    }
    fn add_one(&self) -> Self {
        Self(self.0.add_one())
    }
    fn sub_one(&self) -> Self {
        Self(self.0.sub_one())
    }
    fn add_usize(&self, n: usize) -> Option<Self> {
        self.0.add_usize(n).map(|s| Self(s))
    }
}

pub struct Image {
    pub projection: Projection<Value>,
    size: Size,
    drawn_state: Option<(Projection<Value>, Size)>,

    pixels: Vec<u8>,
    res: usize,

    orbit: Vec<Value>,
    c: Option<Value>,

    palette: [Color; 256],
}

impl Image {
    pub fn new(size: Size) -> Self {
        Image {
            pixels: Vec::new(),
            size,
            res: INITIAL_RES,
            projection: Projection::<Value>::default(),
            drawn_state: None,

            orbit: Vec::<Value>::new(),
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
            let w = Pixel(self.size.w as i32);
            let h = Pixel(self.size.h as i32);
            let res = self.res;
            let mut i = 0;
            for y in (Pixel((res as i32 - 1) / 2)..h).step_by(res) {
                for x in (Pixel((res as i32 - 1) / 2)..w).step_by(res) {
                    let idx = (y * w + x)
                        .to_usize()
                        .expect("pixel coordinates cannot be negative");
                    if self.pixels[idx] == 0 {
                        let c = self.projection.transform(
                            Projected(Complex { re: x, im: y }).into(): Complex<Projected<Pixel>>,
                        );
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

    pub fn needs_draw(&self, p: Option<Value>) -> bool {
        self.res > 0 || p != self.c || self.drawn_state != Some((self.projection, self.size))
    }

    pub fn draw(&mut self, window: &mut Canvas<Window>, p: Option<Value>) -> Result<(), String> {
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
            let res = self.res as i32;
            for x in ((res - 1) / 2..w).step_by(self.res) {
                for y in ((res - 1) / 2..h).step_by(self.res) {
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

        // Increase detail; zero will skip compute altogether
        if self.res > 1 {
            self.res -= 2;
        } else if self.res == 1 {
            self.res = 0
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
                self.orbit.push(Value::default());
                self.orbit.push(p);
                compute_orbit(p, Some(&mut self.orbit));
            }
        }
        if self.c.is_some() {
            let points = self
                .orbit
                .iter()
                .map(|&c| {
                    let p: Result<Complex<Projected<Pixel>>, _> = self.projection.try_transform(c);
                    if let Ok(p) = p {
                        Some(Point::new((p.re.0).0, (p.im.0).0))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            if points[0].is_none() {
                if points.len() > 1 {
                    if let Some(p) = points[1] {
                        if let Some(Some(origin)) = (1..64)
                            .into_iter()
                            .map(|i| {
                                self.projection
                                    .try_transform(
                                        self.orbit[1] * Source(1. - 1. / ((1 << i) as f64)),
                                    )
                                    .ok():
                                    Option<Complex<Projected<Pixel>>>
                            })
                            .find(|&c| c.is_some())
                        {
                            window.draw_line(p, origin.into(): Projected<Complex<Pixel>>)?;
                        }
                    }
                }
            }
            for line in points.split(|c| c.is_none()).filter(|l| !l.is_empty()) {
                window.draw_lines::<&[Point]>(
                    line.iter()
                        .filter_map(|&p| p)
                        .collect::<Vec<_>>()
                        .as_slice(),
                )?;
            }
        }

        Result::Ok(())
    }
}
