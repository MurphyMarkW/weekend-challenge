extern crate test;

use std::cmp;
use std::ops::{Add, Mul};


pub fn dot<T>(x: &[T], y: &[T]) -> T
    where T: Copy + Add<T, Output=T> + Mul<T, Output=T>
{
    let len = cmp::min(x.len(), y.len());

    let xs = &x[..len];
    let ys = &y[..len];

    let mut res: T = xs[0] * ys[0];

    for i in 1..len {
        res = res + xs[i] * ys[i];
    }

    res
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
    fn sdot() {
        use self::rblas::Dot;

        use std::f32::EPSILON;

        let mut rng: StdRng= SeedableRng::from_seed(SEED);

        let x = rng.gen_iter::<f32>().take(3).collect::<Vec<f32>>();
        let y = rng.gen_iter::<f32>().take(3).collect::<Vec<f32>>();

        let f_res = Dot::dot(&x, &y);
        let r_res = dot(&x, &y);

        assert!((f_res - r_res).abs() <= EPSILON);
    }

    #[bench]
    fn sdot_1000_rust(b: &mut test::Bencher) {
        let mut rng: StdRng= SeedableRng::from_seed(SEED);

        let x = rng.gen_iter::<f32>().take(1000).collect::<Vec<f32>>();
        let y = rng.gen_iter::<f32>().take(1000).collect::<Vec<f32>>();

        b.iter(|| dot(&x, &y));
    }

    #[bench]
    fn sdot_1000_fortran(b: &mut test::Bencher) {
        use self::rblas::Dot;

        let mut rng: StdRng= SeedableRng::from_seed(SEED);

        let x = rng.gen_iter::<f32>().take(1000).collect::<Vec<f32>>();
        let y = rng.gen_iter::<f32>().take(1000).collect::<Vec<f32>>();

        b.iter(|| Dot::dot(&x, &y));
    }
}
