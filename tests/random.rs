#![cfg_attr(feature = "cargo-clippy", allow(float_cmp, map_clone))]

#[macro_use] extern crate average;

extern crate rand;

use rand::distributions::Distribution;

use average::{Kurtosis, Estimate};

#[test]
fn normal_distribution() {
    use rand::distributions::Normal;
    let normal = Normal::new(2.0, 3.0);
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
    use rand::distributions::Exp;
    let lambda = 2.0;
    let normal = Exp::new(lambda);
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
