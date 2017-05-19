#[macro_use] extern crate average;

extern crate core;

use core::iter::Iterator;

use average::WeightedAverage;

#[test]
fn trivial() {
    let mut a = WeightedAverage::new();
    assert_eq!(a.sum_weights(), 0.);
    a.add(1.0, 1.0);
    assert_eq!(a.mean(), 1.0);
    assert_eq!(a.sum_weights(), 1.0);
    assert_eq!(a.population_variance(), 0.0);
    assert_eq!(a.error(), 0.0);
    a.add(1.0, 1.0);
    assert_eq!(a.mean(), 1.0);
    assert_eq!(a.sum_weights(), 2.0);
    assert_eq!(a.population_variance(), 0.0);
    assert_eq!(a.error(), 0.0);
}

#[test]
fn simple() {
    let a: WeightedAverage = (1..6).map(|x| (f64::from(x), 1.0)).collect();
    assert_eq!(a.mean(), 3.0);
    assert_eq!(a.sum_weights(), 5.0);
    assert_eq!(a.sample_variance(), 2.5);
    assert_almost_eq!(a.error(), f64::sqrt(0.5), 1e-16);
}

#[test]
fn reference() {
    // Example from http://www.analyticalgroup.com/download/WEIGHTED_MEAN.pdf.
    let values = &[5., 5., 4., 4., 3., 4., 3., 2., 2., 1.];
    let weights = &[1.23, 2.12, 1.23, 0.32, 1.53, 0.59, 0.94, 0.94, 0.84, 0.73];
    let a: WeightedAverage = values.iter().zip(weights.iter())
        .map(|(x, w)| (*x, *w)).collect();
    assert_almost_eq!(a.mean(), 3.53486, 1e-5);
    assert_almost_eq!(a.sample_variance(), 1.8210, 1e-4);
    assert_eq!(a.sum_weights(), 10.47);
    assert_almost_eq!(a.error(), f64::sqrt(0.1739), 1e-4);
}

#[test]
fn error_corner_case() {
    let values = &[1., 2.];
    let weights = &[0.5, 0.5];
    let a: WeightedAverage = values.iter().zip(weights.iter())
        .map(|(x, w)| (*x, *w)).collect();
    assert_eq!(a.error(), 0.5);
}
