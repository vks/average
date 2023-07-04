use core::iter::Iterator;

use average::{assert_almost_eq, Merge, WeightedMeanWithError};

#[test]
fn trivial() {
    let mut a = WeightedMeanWithError::new();
    assert_eq!(a.len(), 0);
    assert_eq!(a.sum_weights(), 0.);
    assert_eq!(a.sum_weights_sq(), 0.);
    a.add(1.0, 1.0);
    assert_eq!(a.len(), 1);
    assert_eq!(a.weighted_mean(), 1.0);
    assert_eq!(a.unweighted_mean(), 1.0);
    assert_eq!(a.sum_weights(), 1.0);
    assert_eq!(a.sum_weights_sq(), 1.0);
    assert_eq!(a.population_variance(), 0.0);
    assert_eq!(a.variance_of_weighted_mean(), 0.0);
    #[cfg(any(feature = "std", feature = "libm"))]
    assert_eq!(a.error(), 0.0);
    a.add(1.0, 1.0);
    assert_eq!(a.len(), 2);
    assert_eq!(a.weighted_mean(), 1.0);
    assert_eq!(a.unweighted_mean(), 1.0);
    assert_eq!(a.sum_weights(), 2.0);
    assert_eq!(a.sum_weights_sq(), 2.0);
    assert_eq!(a.population_variance(), 0.0);
    assert_eq!(a.variance_of_weighted_mean(), 0.0);
    #[cfg(any(feature = "std", feature = "libm"))]
    assert_eq!(a.error(), 0.0);
}

#[test]
fn simple() {
    let a: WeightedMeanWithError = (1..6).map(|x| (f64::from(x), 1.0)).collect();
    assert_eq!(a.len(), 5);
    assert_eq!(a.weighted_mean(), 3.0);
    assert_eq!(a.unweighted_mean(), 3.0);
    assert_eq!(a.sum_weights(), 5.0);
    assert_eq!(a.sample_variance(), 2.5);
    assert_eq!(a.variance_of_weighted_mean(), 0.5);
    #[cfg(any(feature = "std", feature = "libm"))]
    assert_almost_eq!(a.error(), f64::sqrt(0.5), 1e-16);
}

#[cfg(feature = "serde1")]
#[test]
fn simple_serde() {
    let a: WeightedMeanWithError = (1..6).map(|x| (f64::from(x), 1.0)).collect();
    let b = serde_json::to_string(&a).unwrap();
    assert_eq!(&b, "{\"weight_sum_sq\":5.0,\"weighted_avg\":{\"weight_sum\":5.0,\"weighted_avg\":3.0},\"unweighted_avg\":{\"avg\":{\"avg\":3.0,\"n\":5},\"sum_2\":10.0}}");
    let c: WeightedMeanWithError = serde_json::from_str(&b).unwrap();
    assert_eq!(c.len(), 5);
    assert_eq!(c.weighted_mean(), 3.0);
    assert_eq!(c.unweighted_mean(), 3.0);
    assert_eq!(c.sum_weights(), 5.0);
    assert_eq!(c.sample_variance(), 2.5);
    assert_eq!(a.variance_of_weighted_mean(), 0.5);
    #[cfg(any(feature = "std", feature = "libm"))]
    assert_almost_eq!(c.error(), f64::sqrt(0.5), 1e-16);
}

#[test]
fn reference() {
    // Example from http://www.analyticalgroup.com/download/WEIGHTED_MEAN.pdf.
    let values = &[5., 5., 4., 4., 3., 4., 3., 2., 2., 1.];
    let weights = &[1.23, 2.12, 1.23, 0.32, 1.53, 0.59, 0.94, 0.94, 0.84, 0.73];
    let a: WeightedMeanWithError = values
        .iter()
        .zip(weights.iter())
        .map(|(x, w)| (*x, *w))
        .collect();
    assert_almost_eq!(a.weighted_mean(), 3.53486, 1e-5);
    assert_almost_eq!(a.sample_variance(), 1.7889, 1e-4);
    assert_eq!(a.sum_weights(), 10.47);
    assert_eq!(a.len(), 10);
    assert_almost_eq!(a.effective_len(), 8.2315, 1e-4);
    assert_almost_eq!(a.variance_of_weighted_mean(), 0.2173, 1e-4);
    #[cfg(any(feature = "std", feature = "libm"))]
    assert_almost_eq!(a.error(), f64::sqrt(0.2173), 1e-4);
}

#[cfg(any(feature = "std", feature = "libm"))]
#[test]
fn error_corner_case() {
    let values = &[1., 2.];
    let weights = &[0.5, 0.5];
    let a: WeightedMeanWithError = values
        .iter()
        .zip(weights.iter())
        .map(|(x, w)| (*x, *w))
        .collect();
    assert_eq!(a.error(), 0.5);
}

#[test]
fn merge_unweighted() {
    let sequence: &[f64] = &[1., 2., 3., 4., 5., 6., 7., 8., 9.];
    for mid in 0..sequence.len() {
        let (left, right) = sequence.split_at(mid);
        let avg_total: WeightedMeanWithError = sequence.iter().map(|x| (*x, 1.)).collect();
        let mut avg_left: WeightedMeanWithError = left.iter().map(|x| (*x, 1.)).collect();
        let avg_right: WeightedMeanWithError = right.iter().map(|x| (*x, 1.)).collect();
        avg_left.merge(&avg_right);

        assert_eq!(avg_total.sum_weights(), avg_left.sum_weights());
        assert_eq!(avg_total.sum_weights_sq(), avg_left.sum_weights_sq());

        assert_eq!(avg_total.len(), avg_left.len());
        assert_eq!(avg_total.unweighted_mean(), avg_left.unweighted_mean());
        assert_eq!(avg_total.weighted_mean(), avg_left.weighted_mean());
        assert_eq!(avg_total.sample_variance(), avg_left.sample_variance());
    }
}

#[test]
fn merge_weighted() {
    let sequence: &[(f64, f64)] = &[
        (1., 0.1),
        (2., 0.2),
        (3., 0.3),
        (4., 0.4),
        (5., 0.5),
        (6., 0.6),
        (7., 0.7),
        (8., 0.8),
        (9., 0.),
    ];
    for mid in 0..sequence.len() {
        let (left, right) = sequence.split_at(mid);
        let avg_total: WeightedMeanWithError = sequence.iter().collect();
        let mut avg_left: WeightedMeanWithError = left.iter().collect();
        let avg_right: WeightedMeanWithError = right.iter().collect();
        avg_left.merge(&avg_right);
        assert_eq!(avg_total.len(), avg_left.len());
        assert_almost_eq!(avg_total.sum_weights(), avg_left.sum_weights(), 1e-15);
        assert_eq!(avg_total.sum_weights_sq(), avg_left.sum_weights_sq());
        assert_almost_eq!(avg_total.weighted_mean(), avg_left.weighted_mean(), 1e-15);
        assert_almost_eq!(
            avg_total.unweighted_mean(),
            avg_left.unweighted_mean(),
            1e-15
        );
        assert_almost_eq!(
            avg_total.sample_variance(),
            avg_left.sample_variance(),
            1e-14
        );
    }
}

#[test]
fn merge_empty() {
    let mut left = WeightedMeanWithError::new();
    let right = WeightedMeanWithError::new();
    left.merge(&right);
    assert_eq!(left.len(), 0);
    left.add(1., 1.);
    left.add(1., 1.);
    left.add(1., 1.);
    left.add(1., 1.);
    assert_eq!(left.weighted_mean(), 1.);
    assert_eq!(left.unweighted_mean(), 1.);
    assert_eq!(left.sample_variance(), 0.);
}
