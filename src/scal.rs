extern crate test;

use std::ops::Mul;


pub fn scal<T>(a: T, x:&mut [T])
    where T: Copy + Clone + Mul<T, Output=T>
{
    let len = x.len();

    let xs = &mut x[..len];

    for i in 0..len {
        xs[i] = a * xs[i];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate rand;
    extern crate rblas;

    use self::rand::{Rng, StdRng, SeedableRng};

    // A constant seed that we'll use for initializing random vecs.
    const SEED: &'static [usize] = &[1, 2, 3, 4];

    #[test]
    fn sscal() {
        use self::rblas::Scal;

        use std::f32::EPSILON;
        use std::f32::consts::PI;

        let mut rng: StdRng= SeedableRng::from_seed(SEED);

        let x = rng.gen_iter::<f32>().take(3).collect::<Vec<f32>>();

        let mut fortran_result = x.clone();
        let mut rust_result = x.clone();

        Scal::scal(&PI, &mut fortran_result);
        scal(PI, &mut rust_result[..]);

        assert_eq!(fortran_result.len(), rust_result.len());

        for (f, r) in fortran_result.iter().zip(rust_result) {
            assert!((f-r).abs() <= EPSILON);
        }
    }

    #[bench]
    fn sscal_1000_rust(b: &mut test::Bencher) {
        use std::f32::consts::PI;

        let mut rng: StdRng= SeedableRng::from_seed(SEED);

        let mut x = rng.gen_iter::<f32>().take(1000).collect::<Vec<f32>>();

        b.iter(|| scal(PI, &mut x[..]));
    }

    #[bench]
    fn sscal_1000_fortran(b: &mut test::Bencher) {
        use self::rblas::Scal;
        use std::f32::consts::PI;

        let mut rng: StdRng= SeedableRng::from_seed(SEED);

        let mut x = rng.gen_iter::<f32>().take(1000).collect::<Vec<f32>>();

        b.iter(|| Scal::scal(&PI, &mut x[..]));
    }
}
