use std::ops::{Add, Div, Mul, Sub};

use traits::{Simd, Vector};

use f32x4;

impl Simd for f32 {
    type Vector = f32x4;
}

impl Add for f32x4 {
    type Output = f32x4;

    fn add(self, rhs: f32x4) -> f32x4 {
        self + rhs
    }
}

impl Div for f32x4 {
    type Output = f32x4;

    fn div(self, rhs: f32x4) -> f32x4 {
        self / rhs
    }
}

impl Mul for f32x4 {
    type Output = f32x4;

    fn mul(self, rhs: f32x4) -> f32x4 {
        self * rhs
    }
}

impl Sub for f32x4 {
    type Output = f32x4;

    fn sub(self, rhs: f32x4) -> f32x4 {
        self - rhs
    }
}

impl Vector for f32x4 {
    type Elem = f32;

    fn cast(slice: &[f32]) -> (&[f32], &[f32x4], &[f32]) {
        unsafe {
            ::cast(slice)
        }
    }

    fn from_elem(elem: f32) -> f32x4 {
        f32x4(elem, elem, elem, elem)
    }

    fn zeroed() -> f32x4 {
        f32x4(0., 0., 0., 0.)
    }

    fn map<F>(&self, mut f: F) -> f32x4 where F: FnMut(f32) -> f32 {
        f32x4(f(self.0), f(self.1), f(self.2), f(self.3))
    }

    fn sum(&self) -> f32 {
        self.0 + self.1 + self.2 + self.3
    }
}

