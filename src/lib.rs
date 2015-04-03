//! [Experimental] Generic programming with SIMD

#![cfg_attr(test, feature(test))]
#![cfg_attr(test, plugin(quickcheck_macros))]
#![deny(missing_docs)]
#![deny(warnings)]
#![feature(core)]
#![feature(plugin)]
#![feature(simd)]

#[cfg(test)] extern crate test as stdtest;
#[cfg(test)] extern crate approx;
#[cfg(test)] extern crate quickcheck;
#[cfg(test)] extern crate rand;

use traits::Simd;

#[cfg(test)]
mod test;
#[cfg(test)]
mod bench;

mod f32;
mod f64;

pub mod traits;

#[allow(missing_docs, non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
#[simd]
pub struct f32x4(pub f32, pub f32, pub f32, pub f32);

#[allow(missing_docs, non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
#[simd]
pub struct f64x2(pub f64, pub f64);

/// Sum the elements of a slice using SIMD ops
///
/// # Benchmarks
///
/// Size: 1 million elements
///
/// ``` ignore
/// test bench::f32::plain::sum ... bench:    962914 ns/iter (+/- 64354)
/// test bench::f32::simd::sum  ... bench:    242686 ns/iter (+/- 11847)
/// test bench::f64::plain::sum ... bench:    995390 ns/iter (+/- 27834)
/// test bench::f64::simd::sum  ... bench:    504943 ns/iter (+/- 22928)
/// ```
pub fn sum<T>(slice: &[T]) -> T where
    T: Simd,
{
    use std::ops::Add;

    use traits::Vector;

    let (head, body, tail) = T::Vector::cast(slice);
    let sum = body.iter().map(|&x| x).fold(T::Vector::zeroed(), Add::add).sum();
    let sum = head.iter().map(|&x| x).fold(sum, Add::add);
    tail.iter().map(|&x| x).fold(sum, Add::add)
}

/// "Casts" a `&[A]` into an aligned `&[B]`, the elements (both in the front and in the back) that
/// don't fit in the aligned slice, will be returned as slices.
unsafe fn cast<'a, A, B>(slice: &'a [A]) -> (&'a [A], &'a [B], &'a [A]) {
    use std::raw::Repr;
    use std::{raw, mem};

    /// Rounds down `n` to the nearest multiple of `k`
    fn round_down(n: usize, k: usize) -> usize {
        n - n % k
    }

    /// Rounds up `n` to the nearest multiple of `k`
    fn round_up(n: usize, k: usize) -> usize {
        let r = n % k;

        if r == 0 {
            n
        } else {
            n + k - r
        }
    }

    let align_of_b = mem::align_of::<B>();
    let size_of_a = mem::size_of::<A>();
    let size_of_b = mem::size_of::<B>();

    let raw::Slice { data: start, len } = slice.repr();
    let end = start.offset(len as isize);

    let (head_start, tail_end) = (start as usize, end as usize);

    let body_start = round_up(head_start, align_of_b);
    let body_end = round_down(tail_end, align_of_b);

    if body_start >= body_end {
        (slice, &[], &[])
    } else {
        let head_end = body_start;
        let head_len = (head_end - head_start) / size_of_a;

        let body_len = (body_end - body_start) / size_of_b;

        let tail_start = body_end;
        let tail_len = (tail_end - tail_start) / size_of_a;

        let head = mem::transmute(raw::Slice { data: head_start as *const A, len: head_len });
        let body = mem::transmute(raw::Slice { data: body_start as *const B, len: body_len });
        let tail = mem::transmute(raw::Slice { data: tail_start as *const A, len: tail_len });

        (head, body, tail)
    }
}
