use bencher::{benchmark_group, benchmark_main, Bencher};

/// Create a random vector by sampling from a normal distribution.
fn initialize_vec() -> Vec<f64> {
    use rand::SeedableRng;
    use rand_distr::{Distribution, Normal};
    let normal = Normal::new(2.0, 3.0).unwrap();
    let n = 1_000_000;
    let mut values = Vec::with_capacity(n);
    let mut rng = rand_xoshiro::Xoshiro256StarStar::from_seed([
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32,
    ]);
    for _ in 0..n {
        values.push(normal.sample(&mut rng));
    }
    values
}

#[cfg(feature = "libm")]
fn bench_kurtosis(b: &mut Bencher) {
    let values = initialize_vec();
    b.iter(|| {
        let m: average::Kurtosis = values.iter().copied().collect();
        m
    });
}

fn bench_moments(b: &mut Bencher) {
    let values = initialize_vec();
    b.iter(|| {
        let m: average::Moments4 = values.iter().copied().collect();
        m
    });
}

#[cfg(feature = "libm")]
benchmark_group!(benches, bench_kurtosis, bench_moments);
#[cfg(not(feature = "libm"))]
benchmark_group!(benches, bench_moments);

benchmark_main!(benches);
