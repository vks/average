use core::iter::Iterator;

use average::{Kurtosis, Estimate, Merge, assert_almost_eq};

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

#[cfg(feature = "serde1")]
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

#[test]
fn merge() {
    let sequence: &[f64] = &[1., 2., 3., -4., 5.1, 6.3, 7.3, -8., 9., 1.];
    for mid in 0..sequence.len() {
        let (left, right) = sequence.split_at(mid);
        let avg_total: Kurtosis = sequence.iter().collect();
        let mut avg_left: Kurtosis = left.iter().collect();
        let avg_right: Kurtosis = right.iter().collect();
        avg_left.merge(&avg_right);
        assert_eq!(avg_total.len(), avg_left.len());
        assert_almost_eq!(avg_total.mean(), avg_left.mean(), 1e-14);
        assert_almost_eq!(avg_total.sample_variance(), avg_left.sample_variance(), 1e-14);
        assert_almost_eq!(avg_total.skewness(), avg_left.skewness(), 1e-14);
        assert_almost_eq!(avg_total.kurtosis(), avg_left.kurtosis(), 1e-14);
    }
}
