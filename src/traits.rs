//! Traits

use std::ops::{Add, Div, Mul, Sub};

/// Any type that can be SIMD accelerated
pub trait Simd: Add<Output=Self> + Copy + Div<Output=Self> + Mul<Output=Self> + Sub<Output=Self> {
    type Vector: Vector<Elem=Self>;
}

/// SIMD vector
pub trait Vector: Add<Output=Self> + Copy + Div<Output=Self> + Mul<Output=Self> + Sub<Output=Self> {
    type Elem: Simd<Vector=Self>;

    /// Casts e.g. `&[f32]` into an *aligned* `&[f32x4]`, the elements that don't fit in the
    /// aligned slice will be returned as a `&[f32]` slices.
    ///
    /// # Examples
    ///
    /// ```
    /// use simd::f32x4;
    /// use simd::traits::Vector;
    ///
    /// let v: Vec<_> = (0..10).map(|x| x as f32).collect();
    /// let (head, body, tail) = <f32x4 as Vector>::cast(&v);
    ///
    /// assert_eq!(format!("{:?}", head), "[]");
    /// assert_eq!(format!("{:?}", body), "[f32x4(0, 1, 2, 3), f32x4(4, 5, 6, 7)]");
    /// assert_eq!(format!("{:?}", tail), "[8, 9]");
    ///
    /// let slice = &v[1..5];
    /// let (head, body, tail) = <f32x4 as Vector>::cast(slice);
    ///
    /// assert_eq!(format!("{:?}", head), "[1, 2, 3, 4]");
    /// assert_eq!(format!("{:?}", body), "[]");
    /// assert_eq!(format!("{:?}", tail), "[]");
    ///
    /// let slice = &v[2..];
    /// let (head, body, tail) = <f32x4 as Vector>::cast(slice);
    ///
    /// assert_eq!(format!("{:?}", head), "[2, 3]");
    /// assert_eq!(format!("{:?}", body), "[f32x4(4, 5, 6, 7)]");
    /// assert_eq!(format!("{:?}", tail), "[8, 9]");
    /// ```
    fn cast<'a>(&'a [Self::Elem]) -> (&'a [Self::Elem], &'a [Self], &'a [Self::Elem]);

    /// Creates an SIMD vector that contains several copies of `elem`
    fn from_elem(Self::Elem) -> Self;

    /// Creates an SIMD vector with all its elements set to zero
    fn zeroed() -> Self;

    /// Maps each element of the SIMD vector
    ///
    /// NOTE: Slow! This maps `f` on each element *serially*.
    fn map<F>(&self, f: F) -> Self where F: FnMut(Self::Elem) -> Self::Elem;

    /// Returns the sum of all its elements
    fn sum(&self) -> Self::Elem;
}

