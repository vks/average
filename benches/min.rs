#[macro_use] extern crate bencher;
extern crate rand;

extern crate average;
extern crate stats;

use bencher::Bencher;

/// Create a random vector of random floats in [0, 1].
fn initialize_vec() -> Vec<f64> {
    use rand::distributions::{Range, IndependentSample};
    use rand::{XorShiftRng, SeedableRng};
    let range = Range::new(0.0, 1.0);
    let n = 1_000_000;
    let mut values = Vec::with_capacity(n);
    let mut rng = XorShiftRng::from_seed([1, 2, 3, 4]);
    for _ in 0..n {
        values.push(range.ind_sample(&mut rng));
    }
    values
}

fn bench_average(b: &mut Bencher) {
    let values = initialize_vec();
    b.iter(|| {
        let a: average::Min = values.iter().map(|x| *x).collect();
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
