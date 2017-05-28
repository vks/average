#[macro_use] extern crate average;

extern crate core;

extern crate rand;

use core::iter::Iterator;

use average::MeanWithError;

#[test]
fn trivial() {
    let mut a = MeanWithError::new();
    assert_eq!(a.len(), 0);
    a.add(1.0);
    assert_eq!(a.mean(), 1.0);
    assert_eq!(a.len(), 1);
    assert_eq!(a.sample_variance(), 0.0);
    assert_eq!(a.population_variance(), 0.0);
    assert_eq!(a.error(), 0.0);
    a.add(1.0);
    assert_eq!(a.mean(), 1.0);
    assert_eq!(a.len(), 2);
    assert_eq!(a.sample_variance(), 0.0);
    assert_eq!(a.population_variance(), 0.0);
    assert_eq!(a.error(), 0.0);
}

#[test]
fn simple() {
    let a: MeanWithError = (1..6).map(f64::from).collect();
    assert_eq!(a.mean(), 3.0);
    assert_eq!(a.len(), 5);
    assert_eq!(a.sample_variance(), 2.5);
    assert_almost_eq!(a.error(), f64::sqrt(0.5), 1e-16);
}

#[test]
fn numerically_unstable() {
    // The naive algorithm fails for this example due to cancelation.
    let big = 1e9;
    let sample = &[big + 4., big + 7., big + 13., big + 16.];
    let a: MeanWithError = sample.iter().map(|x| *x).collect();
    assert_eq!(a.sample_variance(), 30.);
}

#[test]
fn merge() {
    let sequence: &[f64] = &[1., 2., 3., 4., 5., 6., 7., 8., 9.];
    for mid in 0..sequence.len() {
        let (left, right) = sequence.split_at(mid);
        let avg_total: MeanWithError = sequence.iter().map(|x| *x).collect();
        let mut avg_left: MeanWithError = left.iter().map(|x| *x).collect();
        let avg_right: MeanWithError = right.iter().map(|x| *x).collect();
        avg_left.merge(&avg_right);
        assert_eq!(avg_total.len(), avg_left.len());
        assert_eq!(avg_total.mean(), avg_left.mean());
        assert_eq!(avg_total.sample_variance(), avg_left.sample_variance());
    }
}

#[test]
fn normal_distribution() {
    use rand::distributions::{Normal, IndependentSample};
    let normal = Normal::new(2.0, 3.0);
    let mut a = MeanWithError::new();
    for _ in 0..1_000_000 {
        a.add(normal.ind_sample(&mut ::rand::thread_rng()));
    }
    assert_almost_eq!(a.mean(), 2.0, 1e-2);
    assert_almost_eq!(a.sample_variance().sqrt(), 3.0, 1e-2);
    assert_almost_eq!(a.population_variance().sqrt(), 3.0, 1e-2);
    assert_almost_eq!(a.error(), 0.0, 1e-2);
}
