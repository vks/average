#![cfg_attr(feature = "cargo-clippy", allow(float_cmp, map_clone))]

#[macro_use] extern crate average;




use stats;

/// Create a random vector by sampling from a normal distribution.
fn initialize_vec(size: usize) -> Vec<f64> {
    use rand_distr::{Normal, Distribution};
    use rand_xoshiro::Xoshiro256StarStar;
    use rand::SeedableRng;
    let normal = Normal::new(2.0, 3.0).unwrap();
    let mut values = Vec::with_capacity(size);
    let mut rng = Xoshiro256StarStar::seed_from_u64(42);
    for _ in 0..size {
        values.push(normal.sample(&mut rng));
    }
    values
}

#[test]
fn average_vs_streaming_stats_small() {
    let values = initialize_vec(100);
    let a: average::MeanWithError = values.iter().collect();
    let b: stats::OnlineStats = values.iter().map(|x| *x).collect();
    assert_almost_eq!(a.mean(), b.mean(), 1e-16);
    assert_almost_eq!(a.population_variance(), b.variance(), 1e-14);
}

#[test]
fn average_vs_streaming_stats_large() {
    let values = initialize_vec(1_000_000);
    let a: average::MeanWithError = values.iter().collect();
    let b: stats::OnlineStats = values.iter().map(|x| *x).collect();
    assert_almost_eq!(a.mean(), b.mean(), 1e-16);
    assert_almost_eq!(a.population_variance(), b.variance(), 1e-12);
}
