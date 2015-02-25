const SIZE: usize = 1_000_000;

macro_rules! bench {
    ($ty:ident) => {
        mod $ty {
            mod plain {
                use std::ops::Add;
                use stdtest::Bencher;

                #[bench]
                fn sum(b: &mut Bencher) {
                    let v = ::test::vec::<$ty>(::bench::SIZE);

                    b.iter(|| v.iter().fold(0., Add::add))
                }
            }

            mod simd {
                use stdtest::Bencher;

                #[bench]
                fn sum(b: &mut Bencher) {
                    let v = ::test::vec::<$ty>(::bench::SIZE);

                    b.iter(|| ::sum(&v))
                }
            }
        }
    };
}

bench!(f32);
bench!(f64);
