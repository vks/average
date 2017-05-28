#[macro_use] extern crate average;

extern crate core;

extern crate rand;

use core::iter::Iterator;

use average::Skewness;

#[test]
fn trivial() {
    let mut a = Skewness::new();
    assert_eq!(a.len(), 0);
    a.add(1.0);
    assert_eq!(a.mean(), 1.0);
    assert_eq!(a.len(), 1);
    assert_eq!(a.sample_variance(), 0.0);
    assert_eq!(a.population_variance(), 0.0);
    assert_eq!(a.error_mean(), 0.0);
    assert_eq!(a.skewness(), 0.0);
    a.add(1.0);
    assert_eq!(a.mean(), 1.0);
    assert_eq!(a.len(), 2);
    assert_eq!(a.sample_variance(), 0.0);
    assert_eq!(a.population_variance(), 0.0);
    assert_eq!(a.error_mean(), 0.0);
    assert_eq!(a.skewness(), 0.0);
}

#[test]
fn simple() {
    let mut a: Skewness = (1..6).map(f64::from).collect();
    assert_eq!(a.mean(), 3.0);
    assert_eq!(a.len(), 5);
    assert_eq!(a.sample_variance(), 2.5);
    assert_almost_eq!(a.error_mean(), f64::sqrt(0.5), 1e-16);
    assert_eq!(a.skewness(), 0.0);
    a.add(1.0);
    assert_almost_eq!(a.skewness(), 0.2795084971874741, 1e-15);
}

#[test]
fn merge() {
    let sequence: &[f64] = &[1., 2., 3., 4., 5., 6., 7., 8., 9.];
    for mid in 0..sequence.len() {
        let (left, right) = sequence.split_at(mid);
        let avg_total: Skewness = sequence.iter().map(|x| *x).collect();
        let mut avg_left: Skewness = left.iter().map(|x| *x).collect();
        let avg_right: Skewness = right.iter().map(|x| *x).collect();
        avg_left.merge(&avg_right);
        assert_eq!(avg_total.len(), avg_left.len());
        assert_eq!(avg_total.mean(), avg_left.mean());
        assert_eq!(avg_total.sample_variance(), avg_left.sample_variance());
        assert_eq!(avg_total.skewness(), avg_left.skewness());
    }
}

#[test]
fn exponential_distribution() {
    use rand::distributions::{Exp, IndependentSample};
    let lambda = 2.0;
    let normal = Exp::new(lambda);
    let mut a = Skewness::new();
    for _ in 0..2_000_000 {
        a.add(normal.ind_sample(&mut ::rand::thread_rng()));
    }
    assert_almost_eq!(a.mean(), 1./lambda, 1e-2);
    assert_almost_eq!(a.sample_variance().sqrt(), 1./lambda, 1e-2);
    assert_almost_eq!(a.population_variance().sqrt(), 1./lambda, 1e-2);
    assert_almost_eq!(a.error_mean(), 0.0, 1e-2);
    assert_almost_eq!(a.skewness(), 2.0, 1e-2);
}
