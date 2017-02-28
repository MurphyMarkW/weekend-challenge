#![feature(test)]

extern crate test;
extern crate rayon;

use std::cmp;

fn rayon_axpy(a: f32, x: &[f32], y: &mut [f32]) {
    if x.len() <= 500 {
        let len = cmp::min(x.len(), y.len());

        let xs = &x[..len];
        let ys = &mut y[..len];

        for i in 0..len {
            // NOTE Not using f32.mul_add because it seems to have a
            // consistent order of magnitude lower performance. Would
            // love to know why and if there's a way to fix that.
            ys[i] = ys[i] + a * xs[i];
        }
    } else {
        let mid_point = x.len() / 2;
        let (xleft, xright) = x.split_at(mid_point);
        let (yleft, yright) = y.split_at_mut(mid_point);
        rayon::join(|| rayon_axpy(a, xleft, yleft), || rayon_axpy(a, xright, yright));
    }
}

pub fn axpy(a: f32, x: &Vec<f32>, mut y: Vec<f32>) -> Vec<f32> {
    if a == 0. {
        return y
    }

    // NOTE Lexical scoping shenanigans is in place to help the rust
    // compiler with eliding bounds checks on slice accesses.
    {
        let len = cmp::min(x.len(), y.len());

        let xs = &x[..len];
        let ys = &mut y[..len];

        rayon_axpy(a, xs, ys);
    }
    y
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate rand;
    extern crate rblas;

    use self::rand::{thread_rng, Rng, StdRng, SeedableRng};

    // A constant seed that we'll use for initializing random vecs.
    const SEED: &'static [usize] = &[1, 2, 3, 4];

    #[test]
    fn sxapy() {
        use self::rblas::Axpy;

        use std::f32::EPSILON;
        use std::f32::consts::PI;

        let mut rng: StdRng= SeedableRng::from_seed(SEED);

        let x = rng.gen_iter::<f32>().take(3).collect::<Vec<f32>>();
        let y = rng.gen_iter::<f32>().take(3).collect::<Vec<f32>>();

        let mut fortran_result = y.clone();
        Axpy::axpy(&PI, &x, &mut fortran_result);

        let rust_result = axpy(PI, &x, y.clone());

        assert_eq!(fortran_result.len(), rust_result.len());

        for (f, r) in fortran_result.iter().zip(rust_result) {
            assert!((f-r).abs() <= EPSILON);
        }
    }

    #[bench]
    fn saxpy_1000_baseline(b: &mut test::Bencher) {
        let mut rng: StdRng= SeedableRng::from_seed(SEED);

        let x = rng.gen_iter::<f32>().take(1000).collect::<Vec<f32>>();
        let y = rng.gen_iter::<f32>().take(1000).collect::<Vec<f32>>();
        
        b.iter(|| (&x, y.clone()));
    }

    #[bench]
    fn saxpy_1000_rust(b: &mut test::Bencher) {
        use std::f32::consts::PI;

        let mut rng: StdRng= SeedableRng::from_seed(SEED);

        let x = rng.gen_iter::<f32>().take(1000).collect::<Vec<f32>>();
        let y = rng.gen_iter::<f32>().take(1000).collect::<Vec<f32>>();

        b.iter(|| axpy(PI, &x, y.clone()));
    }

    #[bench]
    fn saxpy_1000_fortran(b: &mut test::Bencher) {
        use self::rblas::Axpy;
        use std::f32::consts::PI;

        let mut rng: StdRng= SeedableRng::from_seed(SEED);

        let x = rng.gen_iter::<f32>().take(1000).collect::<Vec<f32>>();
        let y = rng.gen_iter::<f32>().take(1000).collect::<Vec<f32>>();

        b.iter(|| Axpy::axpy(&PI, &x, &mut y.clone()));
    }
}
