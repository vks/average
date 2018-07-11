#![cfg_attr(feature = "cargo-clippy", allow(float_cmp, map_clone))]

#[macro_use] extern crate bencher;
extern crate rand;

extern crate average;

use bencher::Bencher;

/// Create a random vector by sampling from a normal distribution.
fn initialize_vec() -> Vec<f64> {
    use rand::distributions::{Normal, Distribution};
    use rand::{XorShiftRng, SeedableRng};
    let normal = Normal::new(2.0, 3.0);
    let n = 1_000_000;
    let mut values = Vec::with_capacity(n);
    let mut rng = XorShiftRng::from_seed(
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    for _ in 0..n {
        values.push(normal.sample(&mut rng));
    }
    values
}

fn bench_kurtosis(b: &mut Bencher) {
    let values = initialize_vec();
    b.iter(|| {
        let m: average::Kurtosis = values.iter().map(|x| *x).collect();
        m
    });
}

fn bench_moments(b: &mut Bencher) {
    let values = initialize_vec();
    b.iter(|| {
        let m: average::Moments4 = values.iter().map(|x| *x).collect();
        m
    });
}

benchmark_group!(benches, bench_kurtosis, bench_moments);
benchmark_main!(benches);
