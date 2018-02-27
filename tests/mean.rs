#![cfg_attr(feature = "cargo-clippy", allow(float_cmp, map_clone))]

#[macro_use] extern crate average;

extern crate core;
#[cfg(feature = "serde")]
extern crate serde_json;

use core::iter::Iterator;

use average::{MeanWithError, Estimate, Merge};

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

#[cfg(feature = "serde")]
#[test]
fn simple_serde() {
    let a: MeanWithError = (1..6).map(f64::from).collect();
    let b = serde_json::to_string(&a).unwrap();
    assert_eq!(&b, "{\"avg\":{\"avg\":3.0,\"n\":5},\"sum_2\":10.0}");
    let c: MeanWithError = serde_json::from_str(&b).unwrap();
    assert_eq!(c.mean(), 3.0);
    assert_eq!(c.len(), 5);
    assert_eq!(c.sample_variance(), 2.5);
    assert_almost_eq!(c.error(), f64::sqrt(0.5), 1e-16);
}

#[test]
fn numerically_unstable() {
    // The naive algorithm fails for this example due to cancelation.
    let big = 1e9;
    let sample = &[big + 4., big + 7., big + 13., big + 16.];
    let a: MeanWithError = sample.iter().collect();
    assert_eq!(a.sample_variance(), 30.);
}

#[test]
fn merge() {
    let sequence: &[f64] = &[1., 2., 3., 4., 5., 6., 7., 8., 9.];
    for mid in 0..sequence.len() {
        let (left, right) = sequence.split_at(mid);
        let avg_total: MeanWithError = sequence.iter().collect();
        let mut avg_left: MeanWithError = left.iter().collect();
        let avg_right: MeanWithError = right.iter().collect();
        avg_left.merge(&avg_right);
        assert_eq!(avg_total.len(), avg_left.len());
        assert_eq!(avg_total.mean(), avg_left.mean());
        assert_eq!(avg_total.sample_variance(), avg_left.sample_variance());
    }
}
