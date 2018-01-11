#![cfg_attr(feature = "cargo-clippy", allow(float_cmp, map_clone))]

#[macro_use] extern crate average;

extern crate core;
#[cfg(feature = "serde")]
extern crate serde_json;

use core::iter::Iterator;

use average::{Moments, Merge};

#[test]
fn trivial() {
    let mut a = Moments::new();
    assert_eq!(a.len(), 0);
    a.add(1.0);
    assert_eq!(a.len(), 1);
    assert_eq!(a.mean(), 1.0);
    assert_eq!(a.central_moment(0), 1.0);
    assert_eq!(a.central_moment(1), 0.0);
    assert_eq!(a.central_moment(2), 0.0);
    assert_eq!(a.central_moment(3), 0.0);
    a.add(1.0);
    assert_eq!(a.len(), 2);
    assert_eq!(a.mean(), 1.0);
    assert_eq!(a.central_moment(0), 1.0);
    assert_eq!(a.central_moment(1), 0.0);
    assert_eq!(a.central_moment(2), 0.0);
    assert_eq!(a.central_moment(3), 0.0);
}

#[test]
fn simple() {
    let mut a: Moments = (1..6).map(f64::from).collect();
    assert_eq!(a.len(), 5);
    assert_eq!(a.mean(), 3.0);
    assert_eq!(a.central_moment(0), 1.0);
    assert_eq!(a.central_moment(1), 0.0);
    // variance
    assert_eq!(a.central_moment(2), 2.0);
    assert_eq!(a.standardized_moment(0), 5.0);
    assert_eq!(a.standardized_moment(1), 0.0);
    assert_eq!(a.standardized_moment(2), 1.0);
    assert_almost_eq!(a.sample_skewness(), 0.0, 1e-15);
    a.add(1.0);
    // skewness
    assert_almost_eq!(a.standardized_moment(3), 0.2795084971874741, 1e-15);
    // kurtosis
    assert_almost_eq!(a.standardized_moment(4), -1.365 + 3.0, 1e-14);
}

/*
#[cfg(feature = "serde")]
#[test]
fn simple_serde() {
    let a: Kurtosis = (1..6).map(f64::from).collect();
    let b = serde_json::to_string(&a).unwrap();
    assert_eq!(&b, "{\"avg\":{\"avg\":{\"avg\":{\"avg\":3.0,\"n\":5},\"sum_2\":10.0},\"sum_3\":0.0},\"sum_4\":34.0}");
    let mut c: Kurtosis = serde_json::from_str(&b).unwrap();
    assert_eq!(c.mean(), 3.0);
    assert_eq!(c.len(), 5);
    assert_eq!(c.sample_variance(), 2.5);
    assert_almost_eq!(c.error_mean(), f64::sqrt(0.5), 1e-16);
    assert_eq!(c.skewness(), 0.0);
    c.add(1.0);
    assert_almost_eq!(c.skewness(), 0.2795084971874741, 1e-15);
    assert_almost_eq!(c.kurtosis(), -1.365, 1e-15);
}
*/

#[test]
fn merge() {
    let sequence: &[f64] = &[1., 2., 3., -4., 5.1, 6.3, 7.3, -8., 9., 1.];
    for mid in 0..sequence.len() {
        let (left, right) = sequence.split_at(mid);
        let avg_total: Moments = sequence.iter().map(|x| *x).collect();
        let mut avg_left: Moments = left.iter().map(|x| *x).collect();
        let avg_right: Moments = right.iter().map(|x| *x).collect();
        avg_left.merge(&avg_right);
        assert_eq!(avg_total.len(), avg_left.len());
        assert_almost_eq!(avg_total.mean(), avg_left.mean(), 1e-14);
        assert_almost_eq!(avg_total.central_moment(2), avg_left.central_moment(2), 1e-14);
        assert_almost_eq!(avg_total.central_moment(3), avg_left.central_moment(3), 1e-13);
        assert_almost_eq!(avg_total.central_moment(4), avg_left.central_moment(4), 1e-12);
    }
}
