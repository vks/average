use bencher::{benchmark_group, benchmark_main, Bencher};

/// Create a random vector of random floats in [0, 1].
fn initialize_vec() -> Vec<f64> {
    use rand::SeedableRng;
    use rand_distr::{Distribution, Uniform};
    let range = Uniform::new(0.0, 1.0);
    let n = 1_000_000;
    let mut values = Vec::with_capacity(n);
    let mut rng = rand_xoshiro::Xoshiro256StarStar::from_seed([
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32,
    ]);
    for _ in 0..n {
        values.push(range.sample(&mut rng));
    }
    values
}

fn bench_average(b: &mut Bencher) {
    let values = initialize_vec();
    b.iter(|| {
        let a: average::Min = values.iter().copied().collect();
        a
    });
}

fn bench_iter(b: &mut Bencher) {
    let values = initialize_vec();
    b.iter(|| {
        let mut it = values.iter();
        let init: f64 = *it.next().unwrap();
        it.fold(init, |a, &b| a.min(b))
    });
}

benchmark_group!(benches, bench_average, bench_iter);
benchmark_main!(benches);
