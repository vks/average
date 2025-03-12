use core::iter::Iterator;

use average::{Estimate, Max, Merge};

#[test]
fn trivial() {
    let mut m = Max::new();
    m.add(2.);
    m.add(1.);
    assert_eq!(m.max(), 2.);
    m.add(3.);
    m.add(1.);
    assert_eq!(m.max(), 3.)
}

#[cfg(feature = "serde")]
#[test]
fn trivial_serde() {
    let mut m = Max::new();
    m.add(2.);
    m.add(1.);
    m.add(3.);
    m.add(1.);
    let b = serde_json::to_string(&m).unwrap();
    assert_eq!(&b, "{\"x\":3.0}");
    let c: Max = serde_json::from_str(&b).unwrap();
    assert_eq!(c.max(), 3.)
}

#[test]
fn merge() {
    let sequence: &[f64] = &[1., 2., 3., 4., 5., 6., 7., 8., 9.];
    for mid in 1..sequence.len() {
        let (left, right) = sequence.split_at(mid);
        let max_total: Max = sequence.iter().collect();
        assert_eq!(max_total.max(), 9.);
        let mut max_left: Max = left.iter().collect();
        assert_eq!(max_left.max(), sequence[mid - 1]);
        let max_right: Max = right.iter().collect();
        assert_eq!(max_right.max(), 9.);
        max_left.merge(&max_right);
        assert_eq!(max_total.max(), max_left.max());
    }
}

#[test]
fn merge_empty() {
    let mut left = Max::new();
    let right = Max::new();
    left.merge(&right);
    assert_eq!(left.max(), f64::NEG_INFINITY);
    left.add(1.);
    assert_eq!(left.max(), 1.);
}
