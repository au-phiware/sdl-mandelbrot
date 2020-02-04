extern crate num_complex;
extern crate num_traits;
extern crate sdl2;

use exit;
use num_complex::Complex64;
use num_traits::identities::Zero;
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

const DIV_LIMIT: f64 = 100f64;
const INITIAL_RES: i32 = 11;

struct Image {
    pixels: Vec<u8>,
    w: u32,
    h: u32,
    res: i32,
    tx: Complex64,
    tn: Complex64,
}

impl Image {
    fn new(w: u32, h: u32) -> Self {
        Image {
            pixels: Vec::new(),
            w,
            h,
            res: INITIAL_RES,
            tx: Complex64 { re: 0., im: 0. },
            tn: Complex64 { re: 0., im: 0. },
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

    fn set_transform(&mut self, scale: f64, tn: Complex64) {
        self.tx = Complex64 {
            re: scale / (if self.h < self.w { self.h } else { self.w } as f64),
            im: 0.,
        };
        self.tn = tn
            + Complex64 {
                re: -(self.w as f64) / 2.,
                im: -(self.h as f64) / 2.,
            } * self.tx;
    }

    fn compute_orbit(&self, orbit: &mut Vec<Complex64>, c: Complex64) {
        let mut z = c.clone();
        orbit.push(c);
        let mut m = z.norm_sqr();
        let mut n = 255;
        while m < DIV_LIMIT && n > 0 {
            z = z * z + c;
            m = z.norm_sqr();
            n -= 1;
            orbit.push(z.clone());
        }
    }

    fn compute(&mut self) {
        let mut c = Complex64 { re: 0., im: 0. };
        let mut z = Complex64 { re: 0., im: 0. };
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
                        z.clone_from(&c);
                        let mut m = z.norm_sqr();
                        let mut n = 255;
                        while m < DIV_LIMIT && n > 0 {
                            z = z * z + c;
                            m = z.norm_sqr();
                            n -= 1;
                        }
                        if idx < self.pixels.len() {
                            self.pixels[idx] = n;
                        }
                    }
                }
            }
        }
    }

    fn draw(&mut self, window: &mut Canvas<Window>, p: Option<Complex64>) -> Result<(), String> {
        let texture_creator = window.texture_creator();
        let mut orbit = Vec::<Complex64>::new();

        if self.res > 0 {
            self.compute();
        }
        if let Some(c) = p {
            self.compute_orbit(&mut orbit, c);
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

        window.set_draw_color(PALETTE[255]);
        window.draw_lines::<&[Point]>(
            orbit
                .iter()
                .filter_map(|&c| {
                    self.transform_inv(c)
                        .and_then(|a| Some(Point::new(a.re as i32, a.im as i32)))
                })
                .collect::<Vec<_>>()
                .as_slice(),
        )?;

        Result::Ok(())
    }
}

pub fn main() -> exit::Result {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let mut image = &mut Image::new(600, 600);
    let mut offset = Complex64 { re: 0., im: 0. };
    let mut scale = 4.;
    let mut trace_orbit = false;
    let mut pin_orbit = false;

    let window = video_subsystem
        .window("Mandelbrot", image.w, image.h)
        .position_centered()
        .resizable()
        .build()?;
    let mut canvas = window.into_canvas().accelerated().build()?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut p: Option<Complex64> = None;
    'running: loop {
        if image.res == INITIAL_RES {
            image.set_transform(scale, offset);
            image.clear();
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        image.draw(&mut canvas, if trace_orbit { p } else { None })?;

        canvas.present();
        if image.res > 0 {
            image.res -= 2;
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
                    if mousestate.left() {
                        offset += Complex64 {
                            re: -xrel as f64,
                            im: -yrel as f64,
                        } * image.tx;
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
                    scale *= 1.5f64.powi(n);
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
        }
    }

    exit::Result::Ok
}

const PALETTE: [Color; 256] = [
    /*   0 */ Color::RGBA(0xff, 0xff, 0xff, 0xFF),
    /*   1 */ Color::RGBA(0xf7, 0xf7, 0xf7, 0xFF),
    /*   2 */ Color::RGBA(0xef, 0xef, 0xef, 0xFF),
    /*   3 */ Color::RGBA(0xe7, 0xe7, 0xe7, 0xFF),
    /*   4 */ Color::RGBA(0xdf, 0xdf, 0xdf, 0xFF),
    /*   5 */ Color::RGBA(0xd7, 0xd7, 0xd7, 0xFF),
    /*   6 */ Color::RGBA(0xcf, 0xcf, 0xcf, 0xFF),
    /*   7 */ Color::RGBA(0xc7, 0xc7, 0xc7, 0xFF),
    /*   8 */ Color::RGBA(0xbf, 0xbf, 0xbf, 0xFF),
    /*   9 */ Color::RGBA(0xb7, 0xb7, 0xb7, 0xFF),
    /*  10 */ Color::RGBA(0xaf, 0xaf, 0xaf, 0xFF),
    /*  11 */ Color::RGBA(0xa7, 0xa7, 0xa7, 0xFF),
    /*  12 */ Color::RGBA(0x9f, 0x9f, 0x9f, 0xFF),
    /*  13 */ Color::RGBA(0x97, 0x97, 0x97, 0xFF),
    /*  14 */ Color::RGBA(0x8f, 0x8f, 0x8f, 0xFF),
    /*  15 */ Color::RGBA(0x87, 0x87, 0x87, 0xFF),
    /*  16 */ Color::RGBA(0x7f, 0x7f, 0x7f, 0xFF),
    /*  17 */ Color::RGBA(0x77, 0x77, 0x77, 0xFF),
    /*  18 */ Color::RGBA(0x6f, 0x6f, 0x6f, 0xFF),
    /*  19 */ Color::RGBA(0x67, 0x67, 0x67, 0xFF),
    /*  20 */ Color::RGBA(0x5f, 0x5f, 0x5f, 0xFF),
    /*  21 */ Color::RGBA(0x57, 0x57, 0x57, 0xFF),
    /*  22 */ Color::RGBA(0x4f, 0x4f, 0x4f, 0xFF),
    /*  23 */ Color::RGBA(0x47, 0x47, 0x47, 0xFF),
    /*  24 */ Color::RGBA(0x3f, 0x3f, 0x3f, 0xFF),
    /*  25 */ Color::RGBA(0x37, 0x37, 0x37, 0xFF),
    /*  26 */ Color::RGBA(0x2f, 0x2f, 0x2f, 0xFF),
    /*  27 */ Color::RGBA(0x27, 0x27, 0x27, 0xFF),
    /*  28 */ Color::RGBA(0x1f, 0x1f, 0x1f, 0xFF),
    /*  29 */ Color::RGBA(0x17, 0x17, 0x17, 0xFF),
    /*  30 */ Color::RGBA(0x0f, 0x0f, 0x0f, 0xFF),
    /*  31 */ Color::RGBA(0x07, 0x07, 0x07, 0xFF),
    /*  32 */ Color::RGBA(0x00, 0x00, 0x00, 0xFF), // black
    /*  33 */ Color::RGBA(0xbf, 0xa7, 0x8f, 0xFF),
    /*  34 */ Color::RGBA(0xb7, 0xa0, 0x88, 0xFF),
    /*  35 */ Color::RGBA(0xaf, 0x98, 0x80, 0xFF),
    /*  36 */ Color::RGBA(0xa7, 0x90, 0x78, 0xFF),
    /*  37 */ Color::RGBA(0x9f, 0x89, 0x71, 0xFF),
    /*  38 */ Color::RGBA(0x96, 0x81, 0x69, 0xFF),
    /*  39 */ Color::RGBA(0x8e, 0x79, 0x61, 0xFF),
    /*  40 */ Color::RGBA(0x86, 0x72, 0x5a, 0xFF),
    /*  41 */ Color::RGBA(0x7e, 0x6a, 0x52, 0xFF),
    /*  42 */ Color::RGBA(0x75, 0x62, 0x4a, 0xFF),
    /*  43 */ Color::RGBA(0x6d, 0x5a, 0x42, 0xFF),
    /*  44 */ Color::RGBA(0x65, 0x53, 0x3b, 0xFF),
    /*  45 */ Color::RGBA(0x5d, 0x4b, 0x33, 0xFF),
    /*  46 */ Color::RGBA(0x54, 0x43, 0x2b, 0xFF),
    /*  47 */ Color::RGBA(0x4c, 0x3c, 0x24, 0xFF),
    /*  48 */ Color::RGBA(0x43, 0x33, 0x1b, 0xFF),
    /*  49 */ Color::RGBA(0xbf, 0x7b, 0x4b, 0xFF),
    /*  50 */ Color::RGBA(0xb3, 0x73, 0x47, 0xFF),
    /*  51 */ Color::RGBA(0xab, 0x6f, 0x43, 0xFF),
    /*  52 */ Color::RGBA(0xa3, 0x6b, 0x3f, 0xFF),
    /*  53 */ Color::RGBA(0x9b, 0x63, 0x3b, 0xFF),
    /*  54 */ Color::RGBA(0x8f, 0x5f, 0x37, 0xFF),
    /*  55 */ Color::RGBA(0x87, 0x57, 0x33, 0xFF),
    /*  56 */ Color::RGBA(0x7f, 0x53, 0x2f, 0xFF),
    /*  57 */ Color::RGBA(0x77, 0x4f, 0x2b, 0xFF),
    /*  58 */ Color::RGBA(0x6b, 0x47, 0x27, 0xFF),
    /*  59 */ Color::RGBA(0x5f, 0x43, 0x23, 0xFF),
    /*  60 */ Color::RGBA(0x53, 0x3f, 0x1f, 0xFF),
    /*  61 */ Color::RGBA(0x4b, 0x37, 0x1b, 0xFF),
    /*  62 */ Color::RGBA(0x3f, 0x2f, 0x17, 0xFF),
    /*  63 */ Color::RGBA(0x33, 0x2b, 0x13, 0xFF),
    /*  64 */ Color::RGBA(0x2b, 0x23, 0x0f, 0xFF),
    /*  65 */ Color::RGBA(0xff, 0xeb, 0xdf, 0xFF),
    /*  66 */ Color::RGBA(0xff, 0xe3, 0xd3, 0xFF),
    /*  67 */ Color::RGBA(0xff, 0xdb, 0xc7, 0xFF),
    /*  68 */ Color::RGBA(0xff, 0xd3, 0xbb, 0xFF),
    /*  69 */ Color::RGBA(0xff, 0xcf, 0xb3, 0xFF),
    /*  70 */ Color::RGBA(0xff, 0xc7, 0xa7, 0xFF),
    /*  71 */ Color::RGBA(0xff, 0xbf, 0x9b, 0xFF),
    /*  72 */ Color::RGBA(0xff, 0xbb, 0x93, 0xFF),
    /*  73 */ Color::RGBA(0xff, 0xb3, 0x83, 0xFF),
    /*  74 */ Color::RGBA(0xf7, 0xab, 0x7b, 0xFF),
    /*  75 */ Color::RGBA(0xef, 0xa3, 0x73, 0xFF),
    /*  76 */ Color::RGBA(0xe7, 0x9b, 0x6b, 0xFF),
    /*  77 */ Color::RGBA(0xdf, 0x93, 0x63, 0xFF),
    /*  78 */ Color::RGBA(0xd7, 0x8b, 0x5b, 0xFF),
    /*  79 */ Color::RGBA(0xcf, 0x83, 0x53, 0xFF),
    /*  80 */ Color::RGBA(0xcb, 0x7f, 0x4f, 0xFF),
    /*  81 */ Color::RGBA(0xff, 0xee, 0xdc, 0xFF),
    /*  82 */ Color::RGBA(0xff, 0xdc, 0xb9, 0xFF),
    /*  83 */ Color::RGBA(0xff, 0xcb, 0x97, 0xFF),
    /*  84 */ Color::RGBA(0xff, 0xb9, 0x75, 0xFF),
    /*  85 */ Color::RGBA(0xff, 0xa8, 0x55, 0xFF),
    /*  86 */ Color::RGBA(0xff, 0x97, 0x36, 0xFF),
    /*  87 */ Color::RGBA(0xff, 0x86, 0x19, 0xFF),
    /*  88 */ Color::RGBA(0xff, 0x75, 0x00, 0xFF),
    /*  89 */ Color::RGBA(0xf3, 0x6d, 0x00, 0xFF),
    /*  90 */ Color::RGBA(0xe5, 0x65, 0x00, 0xFF),
    /*  91 */ Color::RGBA(0xd8, 0x5d, 0x00, 0xFF),
    /*  92 */ Color::RGBA(0xcb, 0x55, 0x00, 0xFF),
    /*  93 */ Color::RGBA(0xbe, 0x4d, 0x00, 0xFF),
    /*  94 */ Color::RGBA(0xb1, 0x45, 0x00, 0xFF),
    /*  95 */ Color::RGBA(0xa4, 0x3d, 0x00, 0xFF),
    /*  96 */ Color::RGBA(0x97, 0x36, 0x00, 0xFF),
    /*  97 */ Color::RGBA(0xff, 0xff, 0xef, 0xFF),
    /*  98 */ Color::RGBA(0xff, 0xff, 0xcf, 0xFF),
    /*  99 */ Color::RGBA(0xff, 0xff, 0xaf, 0xFF),
    /* 100 */ Color::RGBA(0xff, 0xff, 0x8f, 0xFF),
    /* 101 */ Color::RGBA(0xff, 0xff, 0x6f, 0xFF),
    /* 102 */ Color::RGBA(0xff, 0xff, 0x4f, 0xFF),
    /* 103 */ Color::RGBA(0xff, 0xff, 0x2f, 0xFF),
    /* 104 */ Color::RGBA(0xff, 0xff, 0x0f, 0xFF),
    /* 105 */ Color::RGBA(0xff, 0xff, 0x00, 0xFF),
    /* 106 */ Color::RGBA(0xcf, 0xcf, 0x00, 0xFF),
    /* 107 */ Color::RGBA(0xaf, 0xaf, 0x00, 0xFF),
    /* 108 */ Color::RGBA(0x8f, 0x8f, 0x00, 0xFF),
    /* 109 */ Color::RGBA(0x6f, 0x6f, 0x00, 0xFF),
    /* 110 */ Color::RGBA(0x4f, 0x4f, 0x00, 0xFF),
    /* 111 */ Color::RGBA(0x2f, 0x2f, 0x00, 0xFF),
    /* 112 */ Color::RGBA(0x0f, 0x0f, 0x00, 0xFF),
    /* 113 */ Color::RGBA(0xff, 0xff, 0x73, 0xFF),
    /* 114 */ Color::RGBA(0xeb, 0xdb, 0x57, 0xFF),
    /* 115 */ Color::RGBA(0xd7, 0xbb, 0x43, 0xFF),
    /* 116 */ Color::RGBA(0xc3, 0x9b, 0x2f, 0xFF),
    /* 117 */ Color::RGBA(0xaf, 0x7b, 0x1f, 0xFF),
    /* 118 */ Color::RGBA(0x9b, 0x5b, 0x13, 0xFF),
    /* 119 */ Color::RGBA(0x87, 0x43, 0x07, 0xFF),
    /* 120 */ Color::RGBA(0x73, 0x2b, 0x00, 0xFF),
    /* 121 */ Color::RGBA(0xff, 0xdf, 0xdf, 0xFF),
    /* 122 */ Color::RGBA(0xff, 0xbf, 0xbf, 0xFF),
    /* 123 */ Color::RGBA(0xff, 0x9f, 0x9f, 0xFF),
    /* 124 */ Color::RGBA(0xff, 0x7f, 0x7f, 0xFF),
    /* 125 */ Color::RGBA(0xff, 0x5f, 0x5f, 0xFF),
    /* 126 */ Color::RGBA(0xff, 0x3f, 0x3f, 0xFF),
    /* 127 */ Color::RGBA(0xff, 0x1f, 0x1f, 0xFF),
    /* 128 */ Color::RGBA(0xff, 0x00, 0x00, 0xFF),
    /* 129 */ Color::RGBA(0xef, 0x00, 0x00, 0xFF),
    /* 130 */ Color::RGBA(0xdf, 0x00, 0x00, 0xFF),
    /* 131 */ Color::RGBA(0xcf, 0x00, 0x00, 0xFF),
    /* 132 */ Color::RGBA(0xbf, 0x00, 0x00, 0xFF),
    /* 133 */ Color::RGBA(0xaf, 0x00, 0x00, 0xFF),
    /* 134 */ Color::RGBA(0x9f, 0x00, 0x00, 0xFF),
    /* 135 */ Color::RGBA(0x8f, 0x00, 0x00, 0xFF),
    /* 136 */ Color::RGBA(0x7f, 0x00, 0x00, 0xFF),
    /* 137 */ Color::RGBA(0x6f, 0x00, 0x00, 0xFF),
    /* 138 */ Color::RGBA(0x5f, 0x00, 0x00, 0xFF),
    /* 139 */ Color::RGBA(0x4f, 0x00, 0x00, 0xFF),
    /* 140 */ Color::RGBA(0x3f, 0x00, 0x00, 0xFF),
    /* 141 */ Color::RGBA(0x2f, 0x00, 0x00, 0xFF),
    /* 142 */ Color::RGBA(0x1f, 0x00, 0x00, 0xFF),
    /* 143 */ Color::RGBA(0x0f, 0x00, 0x00, 0xFF),
    /* 144 */ Color::RGBA(0xff, 0xb7, 0xb7, 0xFF),
    /* 145 */ Color::RGBA(0xf3, 0xa3, 0xa3, 0xFF),
    /* 146 */ Color::RGBA(0xe7, 0x8f, 0x8f, 0xFF),
    /* 147 */ Color::RGBA(0xdb, 0x7b, 0x7b, 0xFF),
    /* 148 */ Color::RGBA(0xcb, 0x6b, 0x6b, 0xFF),
    /* 149 */ Color::RGBA(0xbf, 0x5b, 0x5b, 0xFF),
    /* 150 */ Color::RGBA(0xb3, 0x4f, 0x4f, 0xFF),
    /* 151 */ Color::RGBA(0xa7, 0x3f, 0x3f, 0xFF),
    /* 152 */ Color::RGBA(0x8e, 0x2e, 0x00, 0xFF),
    /* 153 */ Color::RGBA(0x86, 0x27, 0x00, 0xFF),
    /* 154 */ Color::RGBA(0x7e, 0x20, 0x00, 0xFF),
    /* 155 */ Color::RGBA(0x75, 0x19, 0x00, 0xFF),
    /* 156 */ Color::RGBA(0x6d, 0x12, 0x00, 0xFF),
    /* 157 */ Color::RGBA(0x65, 0x0b, 0x00, 0xFF),
    /* 158 */ Color::RGBA(0x5d, 0x05, 0x00, 0xFF),
    /* 159 */ Color::RGBA(0x55, 0x00, 0x00, 0xFF),
    /* 160 */ Color::RGBA(0x77, 0xff, 0x4f, 0xFF),
    /* 161 */ Color::RGBA(0x70, 0xf0, 0x4b, 0xFF),
    /* 162 */ Color::RGBA(0x69, 0xe0, 0x46, 0xFF),
    /* 163 */ Color::RGBA(0x61, 0xd0, 0x41, 0xFF),
    /* 164 */ Color::RGBA(0x5a, 0xc0, 0x3c, 0xFF),
    /* 165 */ Color::RGBA(0x52, 0xb0, 0x37, 0xFF),
    /* 166 */ Color::RGBA(0x4b, 0xa0, 0x32, 0xFF),
    /* 167 */ Color::RGBA(0x43, 0x90, 0x2d, 0xFF),
    /* 168 */ Color::RGBA(0x3c, 0x80, 0x28, 0xFF),
    /* 169 */ Color::RGBA(0x35, 0x70, 0x23, 0xFF),
    /* 170 */ Color::RGBA(0x2d, 0x60, 0x1e, 0xFF),
    /* 171 */ Color::RGBA(0x26, 0x50, 0x19, 0xFF),
    /* 172 */ Color::RGBA(0x1e, 0x40, 0x14, 0xFF),
    /* 173 */ Color::RGBA(0x17, 0x30, 0x0f, 0xFF),
    /* 174 */ Color::RGBA(0x0f, 0x20, 0x0a, 0xFF),
    /* 175 */ Color::RGBA(0x07, 0x0f, 0x04, 0xFF),
    /* 176 */ Color::RGBA(0xde, 0xff, 0xa8, 0xFF),
    /* 177 */ Color::RGBA(0xc7, 0xe4, 0x94, 0xFF),
    /* 178 */ Color::RGBA(0xad, 0xc8, 0x80, 0xFF),
    /* 179 */ Color::RGBA(0x95, 0xad, 0x6b, 0xFF),
    /* 180 */ Color::RGBA(0x7c, 0x92, 0x58, 0xFF),
    /* 181 */ Color::RGBA(0x64, 0x77, 0x44, 0xFF),
    /* 182 */ Color::RGBA(0x4a, 0x5a, 0x30, 0xFF),
    /* 183 */ Color::RGBA(0x32, 0x3f, 0x1d, 0xFF),
    /* 184 */ Color::RGBA(0x00, 0xff, 0x00, 0xFF),
    /* 185 */ Color::RGBA(0x00, 0xdf, 0x00, 0xFF),
    /* 186 */ Color::RGBA(0x00, 0xbf, 0x00, 0xFF),
    /* 187 */ Color::RGBA(0x00, 0x9f, 0x00, 0xFF),
    /* 188 */ Color::RGBA(0x00, 0x7f, 0x00, 0xFF),
    /* 189 */ Color::RGBA(0x00, 0x5f, 0x00, 0xFF),
    /* 190 */ Color::RGBA(0x00, 0x3f, 0x00, 0xFF),
    /* 191 */ Color::RGBA(0x00, 0x1f, 0x00, 0xFF),
    /* 192 */ Color::RGBA(0xff, 0x6f, 0xff, 0xFF),
    /* 193 */ Color::RGBA(0xff, 0x00, 0xff, 0xFF),
    /* 194 */ Color::RGBA(0xdf, 0x00, 0xdf, 0xFF),
    /* 195 */ Color::RGBA(0xbf, 0x00, 0xbf, 0xFF),
    /* 196 */ Color::RGBA(0x9f, 0x00, 0x9f, 0xFF),
    /* 197 */ Color::RGBA(0x7f, 0x00, 0x7f, 0xFF),
    /* 198 */ Color::RGBA(0x5f, 0x00, 0x5f, 0xFF),
    /* 199 */ Color::RGBA(0x3f, 0x00, 0x3f, 0xFF),
    /* 200 */ Color::RGBA(0xe9, 0xe9, 0xf3, 0xFF),
    /* 201 */ Color::RGBA(0xc4, 0xc4, 0xe1, 0xFF),
    /* 202 */ Color::RGBA(0x9d, 0x9d, 0xce, 0xFF),
    /* 203 */ Color::RGBA(0x77, 0x77, 0xbb, 0xFF),
    /* 204 */ Color::RGBA(0x54, 0x54, 0xa7, 0xFF),
    /* 205 */ Color::RGBA(0x41, 0x41, 0x83, 0xFF),
    /* 206 */ Color::RGBA(0x2e, 0x2e, 0x5c, 0xFF),
    /* 207 */ Color::RGBA(0x1b, 0x1b, 0x34, 0xFF),
    /* 208 */ Color::RGBA(0xd5, 0xf1, 0xff, 0xFF),
    /* 209 */ Color::RGBA(0xbf, 0xeb, 0xff, 0xFF),
    /* 210 */ Color::RGBA(0xaa, 0xe3, 0xff, 0xFF),
    /* 211 */ Color::RGBA(0x95, 0xdd, 0xff, 0xFF),
    /* 212 */ Color::RGBA(0x80, 0xd6, 0xff, 0xFF),
    /* 213 */ Color::RGBA(0x6a, 0xcf, 0xff, 0xFF),
    /* 214 */ Color::RGBA(0x55, 0xc8, 0xff, 0xFF),
    /* 215 */ Color::RGBA(0x3f, 0xbf, 0xff, 0xFF),
    /* 216 */ Color::RGBA(0x37, 0x9d, 0xdf, 0xFF),
    /* 217 */ Color::RGBA(0x2f, 0x8f, 0xbf, 0xFF),
    /* 218 */ Color::RGBA(0x27, 0x77, 0x9f, 0xFF),
    /* 219 */ Color::RGBA(0x1f, 0x5f, 0x7f, 0xFF),
    /* 220 */ Color::RGBA(0x00, 0xbf, 0xbf, 0xFF),
    /* 221 */ Color::RGBA(0x00, 0x7f, 0x7f, 0xFF),
    /* 222 */ Color::RGBA(0x00, 0x5f, 0x5f, 0xFF),
    /* 223 */ Color::RGBA(0x00, 0x3f, 0x3f, 0xFF),
    /* 224 */ Color::RGBA(0xe7, 0xe7, 0xff, 0xFF),
    /* 225 */ Color::RGBA(0xc6, 0xc6, 0xff, 0xFF),
    /* 226 */ Color::RGBA(0xad, 0xad, 0xff, 0xFF),
    /* 227 */ Color::RGBA(0x8c, 0x8c, 0xff, 0xFF),
    /* 228 */ Color::RGBA(0x73, 0x73, 0xff, 0xFF),
    /* 229 */ Color::RGBA(0x52, 0x52, 0xff, 0xFF),
    /* 230 */ Color::RGBA(0x31, 0x31, 0xff, 0xFF),
    /* 231 */ Color::RGBA(0x18, 0x18, 0xff, 0xFF),
    /* 232 */ Color::RGBA(0x00, 0x00, 0xff, 0xFF),
    /* 233 */ Color::RGBA(0x00, 0x00, 0xe7, 0xFF),
    /* 234 */ Color::RGBA(0x00, 0x00, 0xce, 0xFF),
    /* 235 */ Color::RGBA(0x00, 0x00, 0xb5, 0xFF),
    /* 236 */ Color::RGBA(0x00, 0x00, 0x9c, 0xFF),
    /* 237 */ Color::RGBA(0x00, 0x00, 0x84, 0xFF),
    /* 238 */ Color::RGBA(0x00, 0x00, 0x6b, 0xFF),
    /* 239 */ Color::RGBA(0x00, 0x00, 0x52, 0xFF),
    /* 240 */ Color::RGBA(0x00, 0x00, 0x4f, 0xFF),
    /* 241 */ Color::RGBA(0x00, 0x00, 0x3f, 0xFF),
    /* 242 */ Color::RGBA(0x00, 0x00, 0x37, 0xFF),
    /* 243 */ Color::RGBA(0x00, 0x00, 0x27, 0xFF),
    /* 244 */ Color::RGBA(0x00, 0x00, 0x1f, 0xFF),
    /* 245 */ Color::RGBA(0x00, 0x00, 0x0f, 0xFF),
    /* 246 */ Color::RGBA(0x00, 0x00, 0x07, 0xFF),
    /* 247 */ Color::RGBA(0x00, 0xff, 0xff, 0xFF),
    /* 248 */ Color::RGBA(0xcf, 0x7f, 0xcf, 0xFF),
    /* 249 */ Color::RGBA(0xb7, 0x6f, 0xb7, 0xFF),
    /* 250 */ Color::RGBA(0x9f, 0x5f, 0x9f, 0xFF),
    /* 251 */ Color::RGBA(0x87, 0x4f, 0x87, 0xFF),
    /* 252 */ Color::RGBA(0x6f, 0x3f, 0x6f, 0xFF),
    /* 253 */ Color::RGBA(0x57, 0x2f, 0x57, 0xFF),
    /* 254 */ Color::RGBA(0x3f, 0x1f, 0x3f, 0xFF),
    /* 255 */ Color::RGBA(0x27, 0x0f, 0x27, 0xFF),
];
