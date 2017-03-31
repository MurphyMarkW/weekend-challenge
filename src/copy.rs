extern crate test;

use std::cmp;


pub fn copy<T>(x: &[T], y: &mut [T])
    where T: Copy
{
    // TODO determine if we should / how best to enforce equal length
    let len = cmp::min(x.len(), y.len());

    let xs = &x[..len];
    let ys = &mut y[..len];

    for i in 0..len {
        ys[i] = xs[i];
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
    fn scopy() {
        use self::rblas::Copy;

        use std::f32::EPSILON;

        let mut rng: StdRng= SeedableRng::from_seed(SEED);

        let x = rng.gen_iter::<f32>().take(3).collect::<Vec<f32>>();
        let mut y = rng.gen_iter::<f32>().take(3).collect::<Vec<f32>>();

        let mut fortran_result = y.clone();
        let mut rust_result = y.clone();

        Copy::copy(&x, &mut fortran_result);
        copy(&x, &mut rust_result);

        assert_eq!(fortran_result.len(), rust_result.len());

        for (f, r) in fortran_result.iter().zip(rust_result) {
            assert!((f-r).abs() <= EPSILON);
        }
    }

    #[bench]
    fn scopy_1000_rust(b: &mut test::Bencher) {
        let mut rng: StdRng= SeedableRng::from_seed(SEED);

        let mut x = rng.gen_iter::<f32>().take(1000).collect::<Vec<f32>>();
        let mut y = rng.gen_iter::<f32>().take(1000).collect::<Vec<f32>>();

        b.iter(|| copy(&mut x[..], &mut y[..]));
    }

    #[bench]
    fn scopy_1000_fortran(b: &mut test::Bencher) {
        use self::rblas::Copy;

        let mut rng: StdRng= SeedableRng::from_seed(SEED);

        let mut x = rng.gen_iter::<f32>().take(1000).collect::<Vec<f32>>();
        let mut y = rng.gen_iter::<f32>().take(1000).collect::<Vec<f32>>();

        b.iter(|| Copy::copy(&mut x[..], &mut y[..]));
    }
}
