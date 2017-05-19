#[macro_use] extern crate average;

extern crate core;

extern crate rand;

use core::iter::Iterator;

use average::Average;

#[test]
fn trivial() {
    let mut a = Average::new();
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
    let a: Average = (1..6).map(f64::from).collect();
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
    let a: Average = sample.iter().map(|x| *x).collect();
    assert_eq!(a.sample_variance(), 30.);
}

#[test]
fn normal_distribution() {
    use rand::distributions::{Normal, IndependentSample};
    let normal = Normal::new(2.0, 3.0);
    let mut a = Average::new();
    for _ in 0..1_000_000 {
        a.add(normal.ind_sample(&mut ::rand::thread_rng()));
    }
    assert_almost_eq!(a.mean(), 2.0, 1e-2);
    assert_almost_eq!(a.sample_variance().sqrt(), 3.0, 1e-2);
}
