#![cfg_attr(feature = "cargo-clippy", allow(clippy::float_cmp, map_clone))]

use rand_distr::Distribution;

use average::{Kurtosis, Estimate, assert_almost_eq};

#[test]
fn normal_distribution() {
    let normal = rand_distr::Normal::new(2.0, 3.0).unwrap();
    let mut a = Kurtosis::new();
    for _ in 0..1_000_000 {
        a.add(normal.sample(&mut ::rand::thread_rng()));
    }
    assert_almost_eq!(a.mean(), 2.0, 1e-2);
    assert_almost_eq!(a.sample_variance().sqrt(), 3.0, 1e-2);
    assert_almost_eq!(a.population_variance().sqrt(), 3.0, 1e-2);
    assert_almost_eq!(a.error_mean(), 0.0, 1e-2);
    assert_almost_eq!(a.skewness(), 0.0, 1e-2);
    assert_almost_eq!(a.kurtosis(), 0.0, 4e-2);
}

#[test]
fn exponential_distribution() {
    let lambda = 2.0;
    let normal = rand_distr::Exp::new(lambda).unwrap();
    let mut a = Kurtosis::new();
    for _ in 0..6_000_000 {
        a.add(normal.sample(&mut ::rand::thread_rng()));
    }
    assert_almost_eq!(a.mean(), 1./lambda, 1e-2);
    assert_almost_eq!(a.sample_variance().sqrt(), 1./lambda, 1e-2);
    assert_almost_eq!(a.population_variance().sqrt(), 1./lambda, 1e-2);
    assert_almost_eq!(a.error_mean(), 0.0, 1e-2);
    assert_almost_eq!(a.skewness(), 2.0, 1e-1);
    assert_almost_eq!(a.kurtosis(), 6.0, 1e-1);
}
