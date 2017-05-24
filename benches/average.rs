#[macro_use] extern crate bencher;
extern crate rand;

extern crate average;
extern crate stats;

use bencher::Bencher;

/// Create a random vector by sampling from a normal distribution.
fn initialize_vec() -> Vec<f64> {
    use rand::distributions::{Normal, IndependentSample};
    use rand::{XorShiftRng, SeedableRng};
    let normal = Normal::new(2.0, 3.0);
    let n = 1_000_000;
    let mut values = Vec::with_capacity(n);
    let mut rng = XorShiftRng::from_seed([1, 2, 3, 4]);
    for _ in 0..n {
        values.push(normal.ind_sample(&mut rng));
    }
    values
}

fn bench_average(b: &mut Bencher) {
    let values = initialize_vec();
    b.iter(|| {
        let a: average::AverageWithError = values.iter().map(|x| *x).collect();
        a
    });
}

fn bench_stats(b: &mut Bencher) {
    let values = initialize_vec();
    b.iter(|| {
        let a: stats::OnlineStats = values.iter().map(|x| *x).collect();
        a
    });
}

benchmark_group!(benches, bench_average, bench_stats);
benchmark_main!(benches);
