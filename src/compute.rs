use num_complex::Complex64;

const DIV_LIMIT: f64 = 400f64;
const FIXED_THRESHOLD: u32 = 20000;

pub fn compute_orbit(c: Complex64, mut orbit: Option<&mut Vec<Complex64>>) -> Option<u32> {
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
