use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait Sample:
    Sized
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Div<Output = Self>
    + DivAssign
    + Mul<Output = Self>
    + MulAssign
    + Neg<Output = Self>
    + Copy
    + FromSample<f32>
    + FromSample<f64>
{
    fn equilibrium() -> Self;
    fn peak() -> Self;
}

pub trait IntoSample {
    fn into_sample<S: Sample>(self) -> S;
}

pub trait FromSample<T> {
    fn from_sample(value: T) -> Self;
}

impl IntoSample for f64 {
    fn into_sample<S: Sample>(self) -> S {
        S::from_sample(self)
    }
}

impl IntoSample for f32 {
    fn into_sample<S: Sample>(self) -> S {
        S::from_sample(self)
    }
}

impl FromSample<f32> for f32 {
    #[inline]
    fn from_sample(value: f32) -> f32 {
        value
    }
}

impl FromSample<f64> for f32 {
    #[inline]
    fn from_sample(value: f64) -> f32 {
        (if value < 0. {
            value.max(core::f32::MIN as f64)
        } else {
            value.min(core::f32::MAX as f64)
        }) as f32
    }
}

impl Sample for f32 {
    fn equilibrium() -> Self {
        0.0
    }

    fn peak() -> Self {
        1.0
    }
}
