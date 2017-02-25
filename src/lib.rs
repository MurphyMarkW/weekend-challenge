#![feature(test)]

extern crate test;

fn axpy(a: f32, x: &Vec<f32>, y: Vec<f32>) -> Vec<f32> {
    x.into_iter().zip(y).map(|(m, n)| m.mul_add(a, n)).collect()
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
    fn it_works() {
    }

    #[test]
    fn fortran_saxpy() {
        use self::rblas::Axpy;

        let a = 3.14;
        let x = vec![1.0, 2.0, 3.0];
        let mut y = vec![2.0, 4.0, 6.0];

        Axpy::axpy(&a, &x, &mut y);
    }

    #[bench]
    fn benchmark_saxpy_1000_baseline(b: &mut test::Bencher) {
        let mut rng: StdRng= SeedableRng::from_seed(SEED);

        let x = rng.gen_iter::<f32>().take(1000).collect::<Vec<f32>>();
        let y = rng.gen_iter::<f32>().take(1000).collect::<Vec<f32>>();
        
        b.iter(|| (&x, y.clone()));
    }

    #[bench]
    fn benchmark_saxpy_1000_rust(b: &mut test::Bencher) {
        use std::f32::consts::PI;

        let mut rng: StdRng= SeedableRng::from_seed(SEED);

        let x = rng.gen_iter::<f32>().take(1000).collect::<Vec<f32>>();
        let y = rng.gen_iter::<f32>().take(1000).collect::<Vec<f32>>();

        b.iter(|| axpy(PI, &x, y.clone()));
    }

    #[bench]
    fn benchmark_saxpy_1000_fortran(b: &mut test::Bencher) {
        use self::rblas::Axpy;
        use std::f32::consts::PI;

        let mut rng: StdRng= SeedableRng::from_seed(SEED);

        let x = rng.gen_iter::<f32>().take(1000).collect::<Vec<f32>>();
        let y = rng.gen_iter::<f32>().take(1000).collect::<Vec<f32>>();

        b.iter(|| Axpy::axpy(&PI, &x, &mut y.clone()));
    }
}
