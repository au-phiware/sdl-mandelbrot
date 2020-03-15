use crossbeam_channel::{bounded as channel, Sender};
use num_complex::Complex;
use num_traits::{Float, FromPrimitive};
use std::{
    any::type_name,
    env,
    sync::{Arc, Condvar, Mutex},
    thread::{self, JoinHandle},
};

const DIV_LIMIT: i32 = 400;
const FIXED_THRESHOLD: u32 = 20000;

pub fn compute_orbit<T: Float + FromPrimitive>(
    value: Complex<T>,
    mut orbit: Option<&mut Vec<Complex<T>>>,
) -> Option<u32> {
    let mut z = value.clone();
    let mut m = z.norm_sqr();
    let mut n = 0;
    if let Some(div_limit) = T::from_i32(DIV_LIMIT) {
        while m < div_limit {
            z = z * z + value;
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
    Update(usize, U),
}

pub struct Surface<T, U, F: FnOnce(usize, &Vec<U>)> {
    fan_out: JoinHandle<()>,
    workers: Vec<JoinHandle<()>>,
    fan_in: JoinHandle<()>,
    job_tx: Sender<ComputeJob<T>>,

    width: usize,
    height: usize,

    cmd_tx: Option<Sender<Command<U, F>>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct ComputeKernel<T> {
    index: usize,
    value: T,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct ComputeJob<T> {
    clear: bool,
    stride: usize,
    width: usize,
    height: usize,
}

impl<T, U, F: FnOnce(usize, &Vec<U>)> Surface<T, U, F>
where
    T: Copy,
    U: Copy + Default,
{
    pub fn new<G: Fn(T) -> U, H: Fn(usize, usize) -> T>(
        compute: G,
        from_cartesian: H,
        width: usize,
        height: usize,
    ) -> Self {
        Self::with_thread_count(
            compute,
            from_cartesian,
            width,
            height,
            env::var("NUM_THREADS")
                .ok()
                .and_then(|v| usize::from_str_radix(v.as_str(), 10).ok())
                .unwrap_or(32),
        )
    }

    pub fn with_thread_count<G: Fn(T) -> U, H: Fn(usize, usize) -> T>(
        compute: G,
        from_cartesian: H,
        width: usize,
        height: usize,
        n: usize,
    ) -> Self {
        let mut workers = Vec::with_capacity(n);
        let fan_from: Vec<U> = Vec::new();
        let fan_into: Vec<U> = Vec::new();
        let (cmd_tx, cmd_rx) = channel(0);
        let (job_tx, job_rx) = channel(0);
        let (pln_tx, pln_rx) = channel(0);
        let pln = Arc::new(Mutex::new(None), Condvar::new());

        let fan_out = {
            let pln = pln.clone();
            thread::spawn(move || {
                let default = Default::<U>::default();
                while let Ok(ComputeJob {
                    clear,
                    stride,
                    width,
                    height,
                }) = job_rx.recv()
                {
                    let (ktx, krx) = channel(n << 1);
                    let (cmd_tx, cmd_rx) = channel(0);
                    let size = width * height;
                    if clear {
                        fan_from.clear();
                    }
                    fan_from.resize_with(size, Default::default);
                    let (pln, cond) = &*pln;
                    {
                        *pln.lock()? = Some((cmd_tx, krx));
                    }
                    cond.notify_all();
                    cmd_tx.send(Command::Deferred(|_, fan_into| {
                        fan_into.truncate(size);
                        fan_from.clone_from_slice(fan_into.as_slice());
                    }))?;
                    if pln_tx.send((stride, cmd_rx)).is_err() {
                        break;
                    }
                    'forall: for y in (stride..height).step_by(stride * 2 + 1) {
                        for x in (stride..width).step_by(stride * 2 + 1) {
                            let index = y * width + x;
                            if index < size && fan_from[index] == default {
                                if ktx
                                    .send(ComputeKernel {
                                        index,
                                        value: from_cartesian(x, y),
                                    })
                                    .is_err()
                                {
                                    break 'forall;
                                }
                            }
                        }
                    }
                }
            })
        };
        let leaf = || {
            let (pln, cond) = &*pln;
            let (cmd_tx, krx) = *cond.wait_while(pln.lock()?, |pln| *pln.is_some())?.unwrap();
            return (cmd_tx.clone(), krx.clone());
        };
        let fan_in = thread::spawn(move || {
            let defers = Vec::new();
            while let Some((stride, rx)) = pln_rx.recv() {
                loop {
                    match select! {
                        recv(rx) -> data => if data.is_err() {
                            break
                        } else {
                            data
                        },
                        recv(cmd_rx) -> data => data,
                    } {
                        Ok(Command::Immediate(cmd)) => cmd(stride, fan_into),
                        Ok(Command::Deferred(cmd)) => defers.push(cmd),
                        Ok(Command::Update(index, u)) => {
                            fan_into.resize_with(index + 1, Default::default);
                            fan_into[index] = u
                        }
                        Err(_) => {}
                    };
                }
                for cmd in defers {
                    cmd(stride, fan_into);
                }
                defers.clear();
            }
        });
        {
            for _ in 0..n {
                workers.push(thread::spawn(move || {
                    while let Some((cmd_tx, krx)) = leaf()? {
                        while let Ok(ComputeKernel { index, value }) = krx.recv() {
                            if cmd_tx.send(Command::Update(index, compute(value))).is_err() {
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
            job_tx,
            width,
            height,
            cmd_tx,
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }

    pub fn request(&mut self, cmd: Command<U, F>) {}

    pub fn compute<G: Fn(T) -> U>(&mut self, compute: G, initial_stride: usize) {
        let mut job = ComputeJob {
            clear: true,
            stride: initial_stride,
            width: self.width,
            height: self.height,
        };
        self.job_tx.send(job)?;
        job.clear = false;
        while job.stride > 1 {
            job.stride -= 1;
            self.job_tx.send(job)?;
        }
    }
}
