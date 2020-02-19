use crossbeam_channel::{bounded as channel, Receiver, SendError, Sender};
use num_complex::Complex;
use num_traits::{Float, FromPrimitive};
use std::{
    any::type_name,
    env,
    thread::{self, JoinHandle},
};

const DIV_LIMIT: i32 = 400;
const FIXED_THRESHOLD: u32 = 20000;

pub fn compute_orbit<T: Float + FromPrimitive>(
    c: Complex<T>,
    mut orbit: Option<&mut Vec<Complex<T>>>,
) -> Option<u32> {
    let mut z = c.clone();
    let mut m = z.norm_sqr();
    let mut n = 0;
    if let Some(div_limit) = T::from_i32(DIV_LIMIT) {
        while m < div_limit {
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
    } else {
        panic!(
            "{} cannot be represented by {}",
            DIV_LIMIT,
            type_name::<T>()
        );
    }
}

pub enum Command<U, F: FnOnce(usize, &Vec<U>)> {
    Immediate(F),
    Deferred(F),
}

pub struct Surface<T, U, F: FnOnce(usize, &Vec<U>)> {
    fan_out: JoinHandle<_>,
    workers: Vec<JoinHandle<_>>,
    fan_in: JoinHandle<_>,

    cmd_tx: Option<Sender<Command<U, F>>>,
    cmd_rx: Option<Receiver<Command<U, F>>>,
}

impl<T, U, F: FnOnce(usize, &Vec<U>)> Surface<T, U, F>
where
    T: Copy,
    U: Copy + Default,
{
    pub fn new<G: Fn(T) -> U>(compute: G) -> Self {
        Self::with_thread_count(
            compute,
            env::var("NUM_THREADS")
                .ok()
                .and_then(|v| usize::from_str_radix(v.as_str(), 10).ok())
                .unwrap_or(32),
        )
    }

    pub fn with_thread_count<G: Fn(T) -> U, H: Fn(usize, usize) -> T>(compute: G, t: H, n: usize) -> Self {
        let mut workers = Vec::with_capacity(n);
        let fan_from: Vec<U> = Vec::new();
        let fan_into: Vec<U> = Vec::new();
        let fan_out = thread::spawn(move || {
            let default = Default::<U>::default();
            while let Ok((clear, stride, w, h)) = rx.recv() {
                let size = w * h;
                if clear {
                    fan_from.clear();
                }
                fan_from.resize_with(size, Default::default);
                cmd_tx.send(Command::Deferred(|_, fan_into| {
                    fan_into.truncate(size);
                    fan_from.clone_from_slice(fan_into.as_slice());
                })?;
                'all_pixel: for y in (stride..h).step_by(stride * 2 + 1) {
                    for x in (stride..w).step_by(stride * 2 + 1) {
                        let index = y * w + x;
                        if index < size && fan_from[index] == default {
                            if tx.send((index, t(x, y))).is_err() {
                                break 'all_pixel;
                            }
                        }
                    }
                }
            }
        });
        let fan_in = thread::spawn(move || {
            while let Some(stride) = rx.recv() {
                let defers = Vec::new();
                loop {
                    select! {
                        recv(rx) -> data => if let Ok((index, u)) = data {
                            fan_into.resize_with(index + 1, Default::default);
                            fan_into[index] = u
                        } else {
                            break
                        },
                        recv(cmd_rx) -> Ok(cmd) => match cmd {
                            Command::Immediate(cmd) => cmd(stride, fan_into),
                            Command::Deferred(cmd) => defers.push(cmd),
                        },
                    }
                }
                for cmd in defers {
                    cmd(stride, fan_into);
                }
            }
        });
        {
            for _ in 0..n {
                workers.push(thread::spawn(move || {
                    while let Some((tx, rx)) = leaf() {
                        while let Ok((index, c)) = rx.recv() {
                            if tx.send((index, compute(c))).is_err() {
                                break;
                            }
                        }
                    }
                }));
            }
        }
        Surface {
            fan_out,
            workers,
            fan_in,
            cmd_tx: None,
            cmd_rx: None,
        }
    }

    pub fn clear(&mut self) {}

    pub fn resize(&mut self, size: usize) {}

    pub fn request(&mut self, cmd: Command<U>) {}

    pub fn compute<G: Fn(T) -> U>(&mut self, compute: G, initial_stride: usize) {
        let (ttx, trx) = channel(n << 1);
        let (utx, urx) = channel(n << 1);
        let s = initial_stride;
        tx.send((true, s, w, h))?;
        for s in s..1 {
            tx.send((false, s, w, h))?;
        }
    }
}
