#![cfg_attr(feature = "cargo-clippy", allow(float_cmp, map_clone))]

extern crate average;

extern crate core;

use core::iter::Iterator;

use average::{Max, Estimate, Merge};

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

#[test]
fn merge() {
    let sequence: &[f64] = &[1., 2., 3., 4., 5., 6., 7., 8., 9.];
    for mid in 1..sequence.len() {
        let (left, right) = sequence.split_at(mid);
        let max_total: Max = sequence.iter().map(|x| *x).collect();
        assert_eq!(max_total.max(), 9.);
        let mut max_left: Max = left.iter().map(|x| *x).collect();
        assert_eq!(max_left.max(), sequence[mid - 1]);
        let max_right: Max = right.iter().map(|x| *x).collect();
        assert_eq!(max_right.max(), 9.);
        max_left.merge(&max_right);
        assert_eq!(max_total.max(), max_left.max());
    }
}
