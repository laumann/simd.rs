use std::ops::{Add, Div, Mul, Sub};

use traits::{Simd, Vector};

use f64x2;

impl Simd for f64 {
    type Vector = f64x2;
}

impl Add for f64x2 {
    type Output = f64x2;

    fn add(self, rhs: f64x2) -> f64x2 {
        self + rhs
    }
}

impl Div for f64x2 {
    type Output = f64x2;

    fn div(self, rhs: f64x2) -> f64x2 {
        self / rhs
    }
}

impl Mul for f64x2 {
    type Output = f64x2;

    fn mul(self, rhs: f64x2) -> f64x2 {
        self * rhs
    }
}

impl Sub for f64x2 {
    type Output = f64x2;

    fn sub(self, rhs: f64x2) -> f64x2 {
        self - rhs
    }
}

impl Vector for f64x2 {
    type Elem = f64;

    fn cast(slice: &[f64]) -> (&[f64], &[f64x2], &[f64]) {
        unsafe {
            ::cast(slice)
        }
    }

    fn from_elem(elem: f64) -> f64x2 {
        f64x2(elem, elem)
    }

    fn zeroed() -> f64x2 {
        f64x2(0., 0.)
    }

    fn map<F>(&self, mut f: F) -> f64x2 where F: FnMut(f64) -> f64 {
        f64x2(f(self.0), f(self.1))
    }

    fn sum(&self) -> f64 {
        self.0 + self.1
    }
}

