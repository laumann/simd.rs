use rand::{Rand, Rng, XorShiftRng, self};

pub fn vec<T>(n: usize) -> Vec<T> where T: Rand {
    let mut rng: XorShiftRng = rand::thread_rng().gen();

    (0..n).map(|_| rng.gen()).collect()
}

macro_rules! approx_eq {
    ($lhs:expr, $rhs:expr) => ({
        let ref lhs = $lhs;
        let ref rhs = $rhs;

        ::approx::eq(lhs, rhs, ::approx::Abs::tol(1e-5)) ||
        ::approx::eq(lhs, rhs, ::approx::Rel::tol(1e-5))
    });
}

macro_rules! test {
    ($ty:ident) => {
        mod $ty {
            use std::ops::Add;

            use quickcheck::TestResult;

            #[quickcheck]
            fn sum(n: usize, start: usize) -> TestResult {
                if start < n {
                    let v = ::test::vec::<$ty>(n);
                    let slice = &v[start..];

                    let lhs = slice.iter().fold(0., Add::add);
                    let rhs = ::sum(slice);

                    TestResult::from_bool(approx_eq!(lhs, rhs))
                } else {
                    TestResult::discard()
                }
            }
        }
    };
}

test!(f32);
test!(f64);
