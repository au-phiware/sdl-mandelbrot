use crossbeam_channel::{bounded as channel, Receiver, SendError, Sender};
use num_complex::Complex;
use num_traits::{Float, FromPrimitive, Zero};
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

pub type Command<U> = FnOnce(&Vec<U>);

pub struct Surface<T, U> {
    fan_out: JoinHandle<_>,
    fan_from: Vec<U>,
    workers: Vec<JoinHandle<_>>,
    fan_into: Vec<U>,
    fan_in: JoinHandle<_>,

    cmd_tx: Option<Sender>,
    cmd_rx: Option<Receiver>,
}

impl<T, U> Surface<T, U>
where
    T: Copy,
    U: Copy + Zero,
{
    pub fn new(compute: Fn(T) -> U) -> Self {
        Self::with_thread_count(
            compute,
            env::var("NUM_THREADS")
                .ok()
                .and_then(|v| usize::from_str_radix(v.as_str(), 10).ok())
                .unwrap_or(32),
        )
    }

    pub fn with_thread_count(compute: Fn(T) -> U, n: usize) -> Self {
        let initial_stride = 5_usize;
        let mut workers = Vec::with_capacity(n);
        let fan_out = thread::spawn(move || {
            while let Some((w, h)) = rx.recv() {
                let size = w *h;
                fan_from.clear();
                fan_from.resize_with(size, Default::default);
                fan_into.clear();
                fan_into.resize_with(size, Default::default);
                for s in initial_stride..1 {
                    for y in (s..h).step_by(s*2 + 1) {
                        for x in (s..w).step_by(s*2+1) {
                            let index = y * w + x;
                            if index < size && fan_from[index] == U::zero() {
                                if Some() = tx.send((s, index, t)) {
                            }
                        }
                    }
                    (fan_from, fan_into) = (fan_into, fan_from);
                }
            }
        });
        {
            for _ in 0..n {
                workers.push(thread::spawn(move || {
                    while let Some((tx, rx)) = leaf() {
                        while let Ok((stride, index, c)) = rx.recv() {
                            if tx.send((stride, index, compute(c))).is_err() {
                                break;
                            }
                        }
                    }
                }));
            }
        }
        Surface {
            fan_out,
            fan_from,
            workers,
            fan_into,
            fan_in,
            cmd_tx: None,
            cmd_rx: None,
        }
    }

    pub fn clear(&mut self) {}

    pub fn resize(&mut self, size: usize) {}

    pub fn request(&mut self, cmd: Command<U>) {}

    pub fn compute(&mut self, compute: Fn(T) -> U, initial_stride: usize) {
        let (ttx, trx) = channel(n << 1);
        let (utx, urx) = channel(n << 1);
    }
}
