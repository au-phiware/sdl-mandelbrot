use self::Value::*;
use num_complex::Complex;
use num_traits::float::FloatCore;
use num_traits::*;
use std::ops::{AddAssign, DivAssign, MulAssign, Neg, RemAssign, SubAssign};

pub trait Projector<FromValue, ToValue> {
    fn transform(&self, v: FromValue) -> ToValue;

    fn translate(&self, v: FromValue) -> ToValue;

    fn scale(&self, v: FromValue) -> ToValue;
}

pub trait MutProjector<WithValue> {
    fn set_translate(&mut self, v: Value<WithValue>);

    fn set_transform(&mut self, v: Value<WithValue>);
}

pub enum Value<T> {
    Absolute(T),
    Relative(T),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Projection<T> {
    tx: T,
    tn: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Source<T>(pub T);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Projected<T>(pub T);

impl<T: Clone + Zero + One> Default for Projection<T> {
    fn default() -> Self {
        Projection {
            tx: T::one(),
            tn: T::zero(),
        }
    }
}

macro_rules! rewrap {
    ($v:tt : $wrapper:ident<Source<$t:ty>>) => {
        Source(($v.into(): Projected<_>).0).into(): $wrapper<Source<$t>>
    };
    ($v:tt : $wrapper:ident<Projected<$t:ty>>) => {
        Projected(($v.into(): Source<_>).0).into(): $wrapper<Projected<$t>>
    };
}

impl<T> Projector<Complex<Source<T>>, Complex<Projected<T>>> for Projection<Complex<Source<T>>>
where
    T: Clone + Float,
{
    fn transform(&self, v: Complex<Source<T>>) -> Complex<Projected<T>> {
        rewrap!(((v - self.tn) / self.tx): Complex<Projected<T>>)
    }

    fn translate(&self, v: Complex<Source<T>>) -> Complex<Projected<T>> {
        rewrap!((v - self.tn): Complex<Projected<T>>)
    }

    fn scale(&self, v: Complex<Source<T>>) -> Complex<Projected<T>> {
        rewrap!((v / self.tx.norm()): Complex<Projected<T>>)
    }
}

impl<T: Clone + Float> Projector<Complex<Projected<T>>, Complex<Source<T>>>
    for Projection<Complex<Source<T>>>
{
    fn transform(&self, v: Complex<Projected<T>>) -> Complex<Source<T>> {
        rewrap!(v: Complex<Source<T>>) * self.tx + self.tn
    }

    fn translate(&self, v: Complex<Projected<T>>) -> Complex<Source<T>> {
        rewrap!(v: Complex<Source<T>>) + self.tn
    }

    fn scale(&self, v: Complex<Projected<T>>) -> Complex<Source<T>> {
        rewrap!(v: Complex<Source<T>>) * self.tx.norm()
    }
}

impl<T: Copy + AddAssign + MulAssign> MutProjector<T> for Projection<T> {
    fn set_translate(&mut self, v: Value<T>) {
        match v {
            Absolute(v) => {
                self.tn = v;
            }
            Relative(v) => {
                self.tn += v;
            }
        }
    }

    fn set_transform(&mut self, v: Value<T>) {
        match v {
            Absolute(v) => {
                self.tx = v;
            }
            Relative(v) => {
                self.tx *= v;
            }
        }
    }
}

impl<T: Copy + Float + NumAssign> MutProjector<Complex<Projected<T>>>
    for Projection<Complex<Source<T>>>
where
    Projection<Complex<Source<T>>>: Projector<Complex<Projected<T>>, Complex<Source<T>>>,
{
    fn set_translate(&mut self, v: Value<Complex<Projected<T>>>) {
        match v {
            Absolute(v) => {
                self.tn = rewrap!(v: Complex<Source<T>>) * self.tx;
            }
            Relative(v) => {
                self.tn += rewrap!(v: Complex<Source<T>>) * self.tx;
            }
        }
    }

    fn set_transform(&mut self, v: Value<Complex<Projected<T>>>) {
        match v {
            Absolute(v) => {
                self.tx = rewrap!(v: Complex<Source<T>>) * self.tx;
            }
            Relative(v) => {
                self.tx *= rewrap!(v: Complex<Source<T>>) * self.tx;
            }
        }
    }
}

impl<T: Default> Default for Source<T> {
    fn default() -> Self {
        Source::<T>(T::default())
    }
}
impl<T: Default> Default for Projected<T> {
    fn default() -> Self {
        Projected::<T>(T::default())
    }
}

impl<T> From<Complex<Source<T>>> for Source<Complex<T>> {
    fn from(v: Complex<Source<T>>) -> Source<Complex<T>> {
        Source(Complex {
            re: v.re.0,
            im: v.im.0,
        })
    }
}
impl<T> From<Complex<Projected<T>>> for Projected<Complex<T>> {
    fn from(v: Complex<Projected<T>>) -> Projected<Complex<T>> {
        Projected(Complex {
            re: v.re.0,
            im: v.im.0,
        })
    }
}

impl<T> From<Source<Complex<T>>> for Complex<Source<T>> {
    fn from(Source(v): Source<Complex<T>>) -> Complex<Source<T>> {
        Complex {
            re: Source(v.re),
            im: Source(v.im),
        }
    }
}
impl<T> From<Projected<Complex<T>>> for Complex<Projected<T>> {
    fn from(Projected(v): Projected<Complex<T>>) -> Complex<Projected<T>> {
        Complex {
            re: Projected(v.re),
            im: Projected(v.im),
        }
    }
}

impl<T> From<Projected<T>> for Source<T> {
    fn from(Projected(v): Projected<T>) -> Source<T> {
        Source(v)
    }
}
impl<T> From<Source<T>> for Projected<T> {
    fn from(Source(v): Source<T>) -> Projected<T> {
        Projected(v)
    }
}

impl<T: Neg<Output = T>> Neg for Source<T> {
    type Output = Source<T>;
    fn neg(self) -> Self {
        Source(self.0.neg())
    }
}
impl<T: Neg<Output = T>> Neg for Projected<T> {
    type Output = Projected<T>;
    fn neg(self) -> Self {
        Projected(self.0.neg())
    }
}

impl<T: ToPrimitive> ToPrimitive for Source<T> {
    fn to_i64(&self) -> Option<i64> {
        <T as ToPrimitive>::to_i64(&self.0)
    }
    fn to_u64(&self) -> Option<u64> {
        <T as ToPrimitive>::to_u64(&self.0)
    }
    fn to_isize(&self) -> Option<isize> {
        <T as ToPrimitive>::to_isize(&self.0)
    }
    fn to_i8(&self) -> Option<i8> {
        <T as ToPrimitive>::to_i8(&self.0)
    }
    fn to_i16(&self) -> Option<i16> {
        <T as ToPrimitive>::to_i16(&self.0)
    }
    fn to_i32(&self) -> Option<i32> {
        <T as ToPrimitive>::to_i32(&self.0)
    }
    fn to_i128(&self) -> Option<i128> {
        <T as ToPrimitive>::to_i128(&self.0)
    }
    fn to_usize(&self) -> Option<usize> {
        <T as ToPrimitive>::to_usize(&self.0)
    }
    fn to_u8(&self) -> Option<u8> {
        <T as ToPrimitive>::to_u8(&self.0)
    }
    fn to_u16(&self) -> Option<u16> {
        <T as ToPrimitive>::to_u16(&self.0)
    }
    fn to_u32(&self) -> Option<u32> {
        <T as ToPrimitive>::to_u32(&self.0)
    }
    fn to_u128(&self) -> Option<u128> {
        <T as ToPrimitive>::to_u128(&self.0)
    }
    fn to_f32(&self) -> Option<f32> {
        <T as ToPrimitive>::to_f32(&self.0)
    }
    fn to_f64(&self) -> Option<f64> {
        <T as ToPrimitive>::to_f64(&self.0)
    }
}
impl<T: NumOps + PartialEq + One> One for Source<T> {
    fn one() -> Self {
        Source(<T as One>::one())
    }
    fn is_one(&self) -> bool {
        T::is_one(&self.0)
    }
}
impl<T: NumOps + Zero> Zero for Source<T> {
    fn zero() -> Self {
        Source(<T as Zero>::zero())
    }
    fn is_zero(&self) -> bool {
        T::is_zero(&self.0)
    }
}
impl<T: NumOps> ::std::ops::Add for Source<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Source(<T as ::std::ops::Add>::add(self.0, other.0))
    }
}
impl<T: NumOps> ::std::ops::Sub for Source<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Source(<T as ::std::ops::Sub>::sub(self.0, other.0))
    }
}
impl<T: NumOps> ::std::ops::Mul for Source<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Source(<T as ::std::ops::Mul>::mul(self.0, other.0))
    }
}
impl<T: NumOps> ::std::ops::Div for Source<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Source(<T as ::std::ops::Div>::div(self.0, other.0))
    }
}
impl<T: NumOps> ::std::ops::Rem for Source<T> {
    type Output = Self;
    fn rem(self, other: Self) -> Self {
        Source(<T as ::std::ops::Rem>::rem(self.0, other.0))
    }
}
impl<T: AddAssign> AddAssign for Source<T> {
    fn add_assign(&mut self, other: Self) {
        self.0.add_assign(other.0)
    }
}
impl<T: SubAssign> SubAssign for Source<T> {
    fn sub_assign(&mut self, other: Self) {
        self.0.sub_assign(other.0)
    }
}
impl<T: MulAssign> MulAssign for Source<T> {
    fn mul_assign(&mut self, other: Self) {
        self.0.mul_assign(other.0)
    }
}
impl<T: DivAssign> DivAssign for Source<T> {
    fn div_assign(&mut self, other: Self) {
        self.0.div_assign(other.0)
    }
}
impl<T: RemAssign> RemAssign for Source<T> {
    fn rem_assign(&mut self, other: Self) {
        self.0.rem_assign(other.0)
    }
}

impl<T: ToPrimitive> ToPrimitive for Projected<T> {
    fn to_i64(&self) -> Option<i64> {
        <T as ToPrimitive>::to_i64(&self.0)
    }
    fn to_u64(&self) -> Option<u64> {
        <T as ToPrimitive>::to_u64(&self.0)
    }
    fn to_isize(&self) -> Option<isize> {
        <T as ToPrimitive>::to_isize(&self.0)
    }
    fn to_i8(&self) -> Option<i8> {
        <T as ToPrimitive>::to_i8(&self.0)
    }
    fn to_i16(&self) -> Option<i16> {
        <T as ToPrimitive>::to_i16(&self.0)
    }
    fn to_i32(&self) -> Option<i32> {
        <T as ToPrimitive>::to_i32(&self.0)
    }
    fn to_i128(&self) -> Option<i128> {
        <T as ToPrimitive>::to_i128(&self.0)
    }
    fn to_usize(&self) -> Option<usize> {
        <T as ToPrimitive>::to_usize(&self.0)
    }
    fn to_u8(&self) -> Option<u8> {
        <T as ToPrimitive>::to_u8(&self.0)
    }
    fn to_u16(&self) -> Option<u16> {
        <T as ToPrimitive>::to_u16(&self.0)
    }
    fn to_u32(&self) -> Option<u32> {
        <T as ToPrimitive>::to_u32(&self.0)
    }
    fn to_u128(&self) -> Option<u128> {
        <T as ToPrimitive>::to_u128(&self.0)
    }
    fn to_f32(&self) -> Option<f32> {
        <T as ToPrimitive>::to_f32(&self.0)
    }
    fn to_f64(&self) -> Option<f64> {
        <T as ToPrimitive>::to_f64(&self.0)
    }
}
impl<T: NumOps + PartialEq + One> One for Projected<T> {
    fn one() -> Self {
        Projected(<T as One>::one())
    }
    fn is_one(&self) -> bool {
        T::is_one(&self.0)
    }
}
impl<T: NumOps + Zero> Zero for Projected<T> {
    fn zero() -> Self {
        Projected(<T as Zero>::zero())
    }
    fn is_zero(&self) -> bool {
        T::is_zero(&self.0)
    }
}

impl<T: Num> Num for Source<T> {
    type FromStrRadixErr = <T as Num>::FromStrRadixErr;
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        T::from_str_radix(s, radix).map(Source)
    }
}
impl<T: NumCast> NumCast for Source<T> {
    fn from<U: ToPrimitive>(n: U) -> Option<Self> {
        T::from(n).map(Source)
    }
}
impl<T: FromPrimitive> FromPrimitive for Source<T> {
    fn from_i64(n: i64) -> Option<Self> {
        T::from_i64(n).map(|t| Source(t))
    }
    fn from_u64(n: u64) -> Option<Self> {
        T::from_u64(n).map(|t| Source(t))
    }
    fn from_isize(n: isize) -> Option<Self> {
        T::from_isize(n).map(|t| Source(t))
    }
    fn from_i8(n: i8) -> Option<Self> {
        T::from_i8(n).map(|t| Source(t))
    }
    fn from_i16(n: i16) -> Option<Self> {
        T::from_i16(n).map(|t| Source(t))
    }
    fn from_i32(n: i32) -> Option<Self> {
        T::from_i32(n).map(|t| Source(t))
    }
    fn from_i128(n: i128) -> Option<Self> {
        T::from_i128(n).map(|t| Source(t))
    }
    fn from_usize(n: usize) -> Option<Self> {
        T::from_usize(n).map(|t| Source(t))
    }
    fn from_u8(n: u8) -> Option<Self> {
        T::from_u8(n).map(|t| Source(t))
    }
    fn from_u16(n: u16) -> Option<Self> {
        T::from_u16(n).map(|t| Source(t))
    }
    fn from_u32(n: u32) -> Option<Self> {
        T::from_u32(n).map(|t| Source(t))
    }
    fn from_u128(n: u128) -> Option<Self> {
        T::from_u128(n).map(|t| Source(t))
    }
    fn from_f32(n: f32) -> Option<Self> {
        T::from_f32(n).map(|t| Source(t))
    }
    fn from_f64(n: f64) -> Option<Self> {
        T::from_f64(n).map(|t| Source(t))
    }
}
impl<T: 'static + Copy, U: AsPrimitive<T>> AsPrimitive<T> for Source<U> {
    fn as_(self) -> T {
        self.0.as_()
    }
}
impl<T: Clone + FloatCore> FloatCore for Source<T> {
    fn nan() -> Self {
        Source(T::nan())
    }
    fn infinity() -> Self {
        Source(T::infinity())
    }
    fn neg_infinity() -> Self {
        Source(T::neg_infinity())
    }
    fn neg_zero() -> Self {
        Source(T::neg_zero())
    }
    fn min_value() -> Self {
        Source(T::min_value())
    }
    fn min_positive_value() -> Self {
        Source(T::min_positive_value())
    }
    fn max_value() -> Self {
        Source(T::max_value())
    }
    fn is_nan(self) -> bool {
        T::is_nan(self.0)
    }
    fn is_infinite(self) -> bool {
        T::is_infinite(self.0)
    }
    fn is_finite(self) -> bool {
        T::is_finite(self.0)
    }
    fn is_normal(self) -> bool {
        T::is_normal(self.0)
    }
    fn classify(self) -> ::std::num::FpCategory {
        T::classify(self.0)
    }
    fn floor(self) -> Self {
        Source(T::floor(self.0))
    }
    fn ceil(self) -> Self {
        Source(T::ceil(self.0))
    }
    fn round(self) -> Self {
        Source(T::round(self.0))
    }
    fn trunc(self) -> Self {
        Source(T::trunc(self.0))
    }
    fn fract(self) -> Self {
        Source(T::fract(self.0))
    }
    fn abs(self) -> Self {
        Source(T::abs(self.0))
    }
    fn signum(self) -> Self {
        Source(T::signum(self.0))
    }
    fn is_sign_positive(self) -> bool {
        T::is_sign_positive(self.0)
    }
    fn is_sign_negative(self) -> bool {
        T::is_sign_negative(self.0)
    }
    fn recip(self) -> Self {
        Source(T::recip(self.0))
    }
    fn powi(self, n: i32) -> Self {
        Source(T::powi(self.0, n))
    }
    fn max(self, other: Self) -> Self {
        Source(T::max(self.0, other.0))
    }
    fn min(self, other: Self) -> Self {
        Source(T::min(self.0, other.0))
    }
    fn integer_decode(self) -> (u64, i16, i8) {
        T::integer_decode(self.0)
    }
    fn epsilon() -> Self {
        Source(T::epsilon())
    }
    fn to_degrees(self) -> Self {
        Source(T::to_degrees(self.0))
    }
    fn to_radians(self) -> Self {
        Source(T::to_radians(self.0))
    }
}
impl<T: Copy + PartialOrd + Num + NumCast + Float> Float for Source<T> {
    fn nan() -> Self {
        Source(T::nan())
    }
    fn infinity() -> Self {
        Source(T::infinity())
    }
    fn neg_infinity() -> Self {
        Source(T::neg_infinity())
    }
    fn neg_zero() -> Self {
        Source(T::neg_zero())
    }
    fn min_value() -> Self {
        Source(T::min_value())
    }
    fn min_positive_value() -> Self {
        Source(T::min_positive_value())
    }
    fn max_value() -> Self {
        Source(T::max_value())
    }
    fn is_nan(self) -> bool {
        T::is_nan(self.0)
    }
    fn is_infinite(self) -> bool {
        T::is_infinite(self.0)
    }
    fn is_finite(self) -> bool {
        T::is_finite(self.0)
    }
    fn is_normal(self) -> bool {
        T::is_normal(self.0)
    }
    fn classify(self) -> ::std::num::FpCategory {
        T::classify(self.0)
    }
    fn floor(self) -> Self {
        Source(T::floor(self.0))
    }
    fn ceil(self) -> Self {
        Source(T::ceil(self.0))
    }
    fn round(self) -> Self {
        Source(T::round(self.0))
    }
    fn trunc(self) -> Self {
        Source(T::trunc(self.0))
    }
    fn fract(self) -> Self {
        Source(T::fract(self.0))
    }
    fn abs(self) -> Self {
        Source(T::abs(self.0))
    }
    fn signum(self) -> Self {
        Source(T::signum(self.0))
    }
    fn is_sign_positive(self) -> bool {
        T::is_sign_positive(self.0)
    }
    fn is_sign_negative(self) -> bool {
        T::is_sign_negative(self.0)
    }
    fn mul_add(self, a: Self, b: Self) -> Self {
        Source(T::mul_add(self.0, a.0, b.0))
    }
    fn recip(self) -> Self {
        Source(T::recip(self.0))
    }
    fn powi(self, n: i32) -> Self {
        Source(T::powi(self.0, n))
    }
    fn powf(self, n: Self) -> Self {
        Source(T::powf(self.0, n.0))
    }
    fn sqrt(self) -> Self {
        Source(T::sqrt(self.0))
    }
    fn exp(self) -> Self {
        Source(T::exp(self.0))
    }
    fn exp2(self) -> Self {
        Source(T::exp2(self.0))
    }
    fn ln(self) -> Self {
        Source(T::ln(self.0))
    }
    fn log(self, base: Self) -> Self {
        Source(T::log(self.0, base.0))
    }
    fn log2(self) -> Self {
        Source(T::log2(self.0))
    }
    fn log10(self) -> Self {
        Source(T::log10(self.0))
    }
    fn max(self, other: Self) -> Self {
        Source(T::max(self.0, other.0))
    }
    fn min(self, other: Self) -> Self {
        Source(T::min(self.0, other.0))
    }
    fn abs_sub(self, other: Self) -> Self {
        Source(T::abs_sub(self.0, other.0))
    }
    fn cbrt(self) -> Self {
        Source(T::cbrt(self.0))
    }
    fn hypot(self, other: Self) -> Self {
        Source(T::hypot(self.0, other.0))
    }
    fn sin(self) -> Self {
        Source(T::sin(self.0))
    }
    fn cos(self) -> Self {
        Source(T::cos(self.0))
    }
    fn tan(self) -> Self {
        Source(T::tan(self.0))
    }
    fn asin(self) -> Self {
        Source(T::asin(self.0))
    }
    fn acos(self) -> Self {
        Source(T::acos(self.0))
    }
    fn atan(self) -> Self {
        Source(T::atan(self.0))
    }
    fn atan2(self, other: Self) -> Self {
        Source(T::atan2(self.0, other.0))
    }
    fn sin_cos(self) -> (Self, Self) {
        let (x, y) = T::sin_cos(self.0);
        (Source(x), Source(y))
    }
    fn exp_m1(self) -> Self {
        Source(T::exp_m1(self.0))
    }
    fn ln_1p(self) -> Self {
        Source(T::ln_1p(self.0))
    }
    fn sinh(self) -> Self {
        Source(T::sinh(self.0))
    }
    fn cosh(self) -> Self {
        Source(T::cosh(self.0))
    }
    fn tanh(self) -> Self {
        Source(T::tanh(self.0))
    }
    fn asinh(self) -> Self {
        Source(T::asinh(self.0))
    }
    fn acosh(self) -> Self {
        Source(T::acosh(self.0))
    }
    fn atanh(self) -> Self {
        Source(T::atanh(self.0))
    }
    fn integer_decode(self) -> (u64, i16, i8) {
        T::integer_decode(self.0)
    }
    fn epsilon() -> Self {
        Source(T::epsilon())
    }
    fn to_degrees(self) -> Self {
        Source(T::to_degrees(self.0))
    }
    fn to_radians(self) -> Self {
        Source(T::to_radians(self.0))
    }
}

impl<T: NumOps> ::std::ops::Add for Projected<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Projected(<T as ::std::ops::Add>::add(self.0, other.0))
    }
}
impl<T: NumOps> ::std::ops::Sub for Projected<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Projected(<T as ::std::ops::Sub>::sub(self.0, other.0))
    }
}
impl<T: NumOps> ::std::ops::Mul for Projected<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Projected(<T as ::std::ops::Mul>::mul(self.0, other.0))
    }
}
impl<T: NumOps> ::std::ops::Div for Projected<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Projected(<T as ::std::ops::Div>::div(self.0, other.0))
    }
}
impl<T: NumOps> ::std::ops::Rem for Projected<T> {
    type Output = Self;
    fn rem(self, other: Self) -> Self {
        Projected(<T as ::std::ops::Rem>::rem(self.0, other.0))
    }
}
impl<T: Num> Num for Projected<T> {
    type FromStrRadixErr = <T as Num>::FromStrRadixErr;
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        T::from_str_radix(s, radix).map(Projected)
    }
}
impl<T: NumCast> NumCast for Projected<T> {
    fn from<U: ToPrimitive>(n: U) -> Option<Self> {
        T::from(n).map(Projected)
    }
}
impl<T: FromPrimitive> FromPrimitive for Projected<T> {
    fn from_i64(n: i64) -> Option<Self> {
        T::from_i64(n).map(|t| Projected(t))
    }
    fn from_u64(n: u64) -> Option<Self> {
        T::from_u64(n).map(|t| Projected(t))
    }
    fn from_isize(n: isize) -> Option<Self> {
        T::from_isize(n).map(|t| Projected(t))
    }
    fn from_i8(n: i8) -> Option<Self> {
        T::from_i8(n).map(|t| Projected(t))
    }
    fn from_i16(n: i16) -> Option<Self> {
        T::from_i16(n).map(|t| Projected(t))
    }
    fn from_i32(n: i32) -> Option<Self> {
        T::from_i32(n).map(|t| Projected(t))
    }
    fn from_i128(n: i128) -> Option<Self> {
        T::from_i128(n).map(|t| Projected(t))
    }
    fn from_usize(n: usize) -> Option<Self> {
        T::from_usize(n).map(|t| Projected(t))
    }
    fn from_u8(n: u8) -> Option<Self> {
        T::from_u8(n).map(|t| Projected(t))
    }
    fn from_u16(n: u16) -> Option<Self> {
        T::from_u16(n).map(|t| Projected(t))
    }
    fn from_u32(n: u32) -> Option<Self> {
        T::from_u32(n).map(|t| Projected(t))
    }
    fn from_u128(n: u128) -> Option<Self> {
        T::from_u128(n).map(|t| Projected(t))
    }
    fn from_f32(n: f32) -> Option<Self> {
        T::from_f32(n).map(|t| Projected(t))
    }
    fn from_f64(n: f64) -> Option<Self> {
        T::from_f64(n).map(|t| Projected(t))
    }
}
impl<T: 'static + Copy, U: AsPrimitive<T>> AsPrimitive<T> for Projected<U> {
    fn as_(self) -> T {
        self.0.as_()
    }
}
impl<T: Clone + FloatCore> FloatCore for Projected<T> {
    fn nan() -> Self {
        Projected(T::nan())
    }
    fn infinity() -> Self {
        Projected(T::infinity())
    }
    fn neg_infinity() -> Self {
        Projected(T::neg_infinity())
    }
    fn neg_zero() -> Self {
        Projected(T::neg_zero())
    }
    fn min_value() -> Self {
        Projected(T::min_value())
    }
    fn min_positive_value() -> Self {
        Projected(T::min_positive_value())
    }
    fn max_value() -> Self {
        Projected(T::max_value())
    }
    fn is_nan(self) -> bool {
        T::is_nan(self.0)
    }
    fn is_infinite(self) -> bool {
        T::is_infinite(self.0)
    }
    fn is_finite(self) -> bool {
        T::is_finite(self.0)
    }
    fn is_normal(self) -> bool {
        T::is_normal(self.0)
    }
    fn classify(self) -> ::std::num::FpCategory {
        T::classify(self.0)
    }
    fn floor(self) -> Self {
        Projected(T::floor(self.0))
    }
    fn ceil(self) -> Self {
        Projected(T::ceil(self.0))
    }
    fn round(self) -> Self {
        Projected(T::round(self.0))
    }
    fn trunc(self) -> Self {
        Projected(T::trunc(self.0))
    }
    fn fract(self) -> Self {
        Projected(T::fract(self.0))
    }
    fn abs(self) -> Self {
        Projected(T::abs(self.0))
    }
    fn signum(self) -> Self {
        Projected(T::signum(self.0))
    }
    fn is_sign_positive(self) -> bool {
        T::is_sign_positive(self.0)
    }
    fn is_sign_negative(self) -> bool {
        T::is_sign_negative(self.0)
    }
    fn recip(self) -> Self {
        Projected(T::recip(self.0))
    }
    fn powi(self, n: i32) -> Self {
        Projected(T::powi(self.0, n))
    }
    fn max(self, other: Self) -> Self {
        Projected(T::max(self.0, other.0))
    }
    fn min(self, other: Self) -> Self {
        Projected(T::min(self.0, other.0))
    }
    fn integer_decode(self) -> (u64, i16, i8) {
        T::integer_decode(self.0)
    }
    fn epsilon() -> Self {
        Projected(T::epsilon())
    }
    fn to_degrees(self) -> Self {
        Projected(T::to_degrees(self.0))
    }
    fn to_radians(self) -> Self {
        Projected(T::to_radians(self.0))
    }
}
impl<T: Copy + PartialOrd + Num + NumCast + Float> Float for Projected<T> {
    fn nan() -> Self {
        Projected(T::nan())
    }
    fn infinity() -> Self {
        Projected(T::infinity())
    }
    fn neg_infinity() -> Self {
        Projected(T::neg_infinity())
    }
    fn neg_zero() -> Self {
        Projected(T::neg_zero())
    }
    fn min_value() -> Self {
        Projected(T::min_value())
    }
    fn min_positive_value() -> Self {
        Projected(T::min_positive_value())
    }
    fn max_value() -> Self {
        Projected(T::max_value())
    }
    fn is_nan(self) -> bool {
        T::is_nan(self.0)
    }
    fn is_infinite(self) -> bool {
        T::is_infinite(self.0)
    }
    fn is_finite(self) -> bool {
        T::is_finite(self.0)
    }
    fn is_normal(self) -> bool {
        T::is_normal(self.0)
    }
    fn classify(self) -> ::std::num::FpCategory {
        T::classify(self.0)
    }
    fn floor(self) -> Self {
        Projected(T::floor(self.0))
    }
    fn ceil(self) -> Self {
        Projected(T::ceil(self.0))
    }
    fn round(self) -> Self {
        Projected(T::round(self.0))
    }
    fn trunc(self) -> Self {
        Projected(T::trunc(self.0))
    }
    fn fract(self) -> Self {
        Projected(T::fract(self.0))
    }
    fn abs(self) -> Self {
        Projected(T::abs(self.0))
    }
    fn signum(self) -> Self {
        Projected(T::signum(self.0))
    }
    fn is_sign_positive(self) -> bool {
        T::is_sign_positive(self.0)
    }
    fn is_sign_negative(self) -> bool {
        T::is_sign_negative(self.0)
    }
    fn mul_add(self, a: Self, b: Self) -> Self {
        Projected(T::mul_add(self.0, a.0, b.0))
    }
    fn recip(self) -> Self {
        Projected(T::recip(self.0))
    }
    fn powi(self, n: i32) -> Self {
        Projected(T::powi(self.0, n))
    }
    fn powf(self, n: Self) -> Self {
        Projected(T::powf(self.0, n.0))
    }
    fn sqrt(self) -> Self {
        Projected(T::sqrt(self.0))
    }
    fn exp(self) -> Self {
        Projected(T::exp(self.0))
    }
    fn exp2(self) -> Self {
        Projected(T::exp2(self.0))
    }
    fn ln(self) -> Self {
        Projected(T::ln(self.0))
    }
    fn log(self, base: Self) -> Self {
        Projected(T::log(self.0, base.0))
    }
    fn log2(self) -> Self {
        Projected(T::log2(self.0))
    }
    fn log10(self) -> Self {
        Projected(T::log10(self.0))
    }
    fn max(self, other: Self) -> Self {
        Projected(T::max(self.0, other.0))
    }
    fn min(self, other: Self) -> Self {
        Projected(T::min(self.0, other.0))
    }
    fn abs_sub(self, other: Self) -> Self {
        Projected(T::abs_sub(self.0, other.0))
    }
    fn cbrt(self) -> Self {
        Projected(T::cbrt(self.0))
    }
    fn hypot(self, other: Self) -> Self {
        Projected(T::hypot(self.0, other.0))
    }
    fn sin(self) -> Self {
        Projected(T::sin(self.0))
    }
    fn cos(self) -> Self {
        Projected(T::cos(self.0))
    }
    fn tan(self) -> Self {
        Projected(T::tan(self.0))
    }
    fn asin(self) -> Self {
        Projected(T::asin(self.0))
    }
    fn acos(self) -> Self {
        Projected(T::acos(self.0))
    }
    fn atan(self) -> Self {
        Projected(T::atan(self.0))
    }
    fn atan2(self, other: Self) -> Self {
        Projected(T::atan2(self.0, other.0))
    }
    fn sin_cos(self) -> (Self, Self) {
        let (x, y) = T::sin_cos(self.0);
        (Projected(x), Projected(y))
    }
    fn exp_m1(self) -> Self {
        Projected(T::exp_m1(self.0))
    }
    fn ln_1p(self) -> Self {
        Projected(T::ln_1p(self.0))
    }
    fn sinh(self) -> Self {
        Projected(T::sinh(self.0))
    }
    fn cosh(self) -> Self {
        Projected(T::cosh(self.0))
    }
    fn tanh(self) -> Self {
        Projected(T::tanh(self.0))
    }
    fn asinh(self) -> Self {
        Projected(T::asinh(self.0))
    }
    fn acosh(self) -> Self {
        Projected(T::acosh(self.0))
    }
    fn atanh(self) -> Self {
        Projected(T::atanh(self.0))
    }
    fn integer_decode(self) -> (u64, i16, i8) {
        T::integer_decode(self.0)
    }
    fn epsilon() -> Self {
        Projected(T::epsilon())
    }
    fn to_degrees(self) -> Self {
        Projected(T::to_degrees(self.0))
    }
    fn to_radians(self) -> Self {
        Projected(T::to_radians(self.0))
    }
}
