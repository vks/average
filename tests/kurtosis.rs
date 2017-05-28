#[macro_use] extern crate average;

extern crate core;

use core::iter::Iterator;

use average::Kurtosis;

#[test]
fn trivial() {
    let mut a = Kurtosis::new();
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
    let mut a: Kurtosis = (1..6).map(f64::from).collect();
    assert_eq!(a.mean(), 3.0);
    assert_eq!(a.len(), 5);
    assert_eq!(a.sample_variance(), 2.5);
    assert_almost_eq!(a.error_mean(), f64::sqrt(0.5), 1e-16);
    assert_eq!(a.skewness(), 0.0);
    a.add(1.0);
    assert_almost_eq!(a.skewness(), 0.2795084971874741, 1e-15);
    assert_almost_eq!(a.kurtosis(), -1.365, 1e-15);
}

#[test]
fn merge() {
    let sequence: &[f64] = &[1., 2., 3., -4., 5.1, 6.3, 7.3, -8., 9., 1.];
    for mid in 0..sequence.len() {
        let (left, right) = sequence.split_at(mid);
        let avg_total: Kurtosis = sequence.iter().map(|x| *x).collect();
        let mut avg_left: Kurtosis = left.iter().map(|x| *x).collect();
        let avg_right: Kurtosis = right.iter().map(|x| *x).collect();
        avg_left.merge(&avg_right);
        assert_eq!(avg_total.len(), avg_left.len());
        assert_almost_eq!(avg_total.mean(), avg_left.mean(), 1e-14);
        assert_almost_eq!(avg_total.sample_variance(), avg_left.sample_variance(), 1e-14);
        assert_almost_eq!(avg_total.skewness(), avg_left.skewness(), 1e-14);
        assert_almost_eq!(avg_total.kurtosis(), avg_left.kurtosis(), 1e-14);
    }
}
