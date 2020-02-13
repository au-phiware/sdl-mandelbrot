use num_complex::Complex;
use num_traits::{Float, FromPrimitive};
use std::any::type_name;

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
