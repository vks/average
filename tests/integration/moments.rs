use core::iter::Iterator;

use average::{assert_almost_eq, Merge, Moments4};

#[test]
fn trivial() {
    let mut a = Moments4::new();
    assert_eq!(a.len(), 0);
    assert_eq!(a.central_moment(1), 0.0);
    #[cfg(any(feature = "std", feature = "libm"))]
    assert_eq!(a.standardized_moment(2), 1.0);
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
    let mut a: Moments4 = (1..6).map(f64::from).collect();
    assert_eq!(a.len(), 5);
    assert_eq!(a.mean(), 3.0);
    assert_eq!(a.central_moment(0), 1.0);
    assert_eq!(a.central_moment(1), 0.0);
    // variance
    assert_eq!(a.central_moment(2), 2.0);
    #[cfg(any(feature = "std", feature = "libm"))]
    {
        assert_eq!(a.standardized_moment(0), 5.0);
        assert_eq!(a.standardized_moment(1), 0.0);
        assert_eq!(a.standardized_moment(2), 1.0);
        assert_almost_eq!(a.sample_skewness(), 0.0, 1e-15);
        assert_almost_eq!(a.standardized_moment(3), 0.0, 1e-15);
    }
    a.add(1.0);
    #[cfg(any(feature = "std", feature = "libm"))]
    {
        // skewness
        assert_almost_eq!(a.standardized_moment(3), 0.2795084971874741, 1e-15);
        // kurtosis
        assert_almost_eq!(a.standardized_moment(4), -1.365 + 3.0, 1e-14);
    }
}

#[test]
fn simple_extend() {
    let mut a = Moments4::new();
    a.extend((1..6).map(f64::from));
    assert_eq!(a.len(), 5);
    assert_eq!(a.mean(), 3.0);
    assert_eq!(a.central_moment(0), 1.0);
    assert_eq!(a.central_moment(1), 0.0);
    // variance
    assert_eq!(a.central_moment(2), 2.0);
    #[cfg(any(feature = "std", feature = "libm"))]
    {
        assert_eq!(a.standardized_moment(0), 5.0);
        assert_eq!(a.standardized_moment(1), 0.0);
        assert_eq!(a.standardized_moment(2), 1.0);
        assert_almost_eq!(a.sample_skewness(), 0.0, 1e-15);
        assert_almost_eq!(a.standardized_moment(3), 0.0, 1e-15);
    }
    a.add(1.0);
    #[cfg(any(feature = "std", feature = "libm"))]
    {
        // skewness
        assert_almost_eq!(a.standardized_moment(3), 0.2795084971874741, 1e-15);
        // kurtosis
        assert_almost_eq!(a.standardized_moment(4), -1.365 + 3.0, 1e-14);
    }
}

#[cfg(feature = "serde")]
#[test]
fn simple_serde() {
    let a: Moments4 = (1..6).map(f64::from).collect();
    let b = serde_json::to_string(&a).unwrap();
    assert_eq!(
        &b,
        "{\"n\":5,\"avg\":3.0,\"m\":[10.0,1.7763568394002505e-15,34.00000000000001]}"
    );
    let mut c: Moments4 = serde_json::from_str(&b).unwrap();
    assert_eq!(c.len(), 5);
    assert_eq!(c.mean(), 3.0);
    assert_eq!(c.central_moment(0), 1.0);
    assert_eq!(c.central_moment(1), 0.0);
    // variance
    assert_eq!(c.central_moment(2), 2.0);
    #[cfg(any(feature = "std", feature = "libm"))]
    {
        assert_eq!(c.standardized_moment(0), 5.0);
        assert_eq!(c.standardized_moment(1), 0.0);
        assert_eq!(c.standardized_moment(2), 1.0);
        assert_almost_eq!(c.sample_skewness(), 0.0, 1e-15);
        assert_almost_eq!(c.standardized_moment(3), 0.0, 1e-15);
    }
    c.add(1.0);
    #[cfg(any(feature = "std", feature = "libm"))]
    {
        // skewness
        assert_almost_eq!(c.standardized_moment(3), 0.2795084971874741, 1e-15);
        // kurtosis
        assert_almost_eq!(c.standardized_moment(4), -1.365 + 3.0, 1e-14);
    }
}

#[test]
fn merge() {
    let sequence: &[f64] = &[1., 2., 3., -4., 5.1, 6.3, 7.3, -8., 9., 1.];
    for mid in 0..sequence.len() {
        let (left, right) = sequence.split_at(mid);
        let avg_total: Moments4 = sequence.iter().collect();
        let mut avg_left: Moments4 = left.iter().collect();
        let avg_right: Moments4 = right.iter().collect();
        avg_left.merge(&avg_right);
        assert_eq!(avg_total.len(), avg_left.len());
        assert_almost_eq!(avg_total.mean(), avg_left.mean(), 1e-14);
        assert_almost_eq!(
            avg_total.central_moment(2),
            avg_left.central_moment(2),
            1e-14
        );
        assert_almost_eq!(
            avg_total.central_moment(3),
            avg_left.central_moment(3),
            1e-13
        );
        assert_almost_eq!(
            avg_total.central_moment(4),
            avg_left.central_moment(4),
            1e-12
        );
    }
}

#[test]
fn merge_empty() {
    let mut left = Moments4::new();
    let right = Moments4::new();
    left.merge(&right);
    assert_eq!(left.len(), 0);
    left.add(1.);
    left.add(1.);
    left.add(1.);
    left.add(1.);
    assert_eq!(left.mean(), 1.);
    assert_eq!(left.central_moment(2), 0.);
    assert_eq!(left.central_moment(3), 0.);
    assert_eq!(left.central_moment(4), 0.);
}
