#[macro_use] extern crate average;

extern crate core;

use core::iter::Iterator;

define_histogram!(Histogram, 10);

#[test]
fn with_const_width() {
    let mut h = Histogram::with_const_width(0., 100.);
    for i in 0..100 {
        h.add(i as f64).unwrap();
    }
    assert_eq!(h.bins(), &[10, 10, 10, 10, 10, 10, 10, 10, 10, 10]);
}

#[test]
fn from_ranges() {
    let mut h = Histogram::from_ranges(
        [0., 0.1, 0.2, 0.3, 0.4, 0.5, 0.7, 0.8, 0.9, 1.0, 2.0].iter().cloned()).unwrap();
    for &i in &[0.05, 0.7, 1.0, 1.5] {
        h.add(i).unwrap();
    }
    assert_eq!(h.bins(), &[1, 0, 0, 0, 0, 0, 1, 0, 0, 2]);
}

#[test]
fn from_ranges_infinity() {
    let inf = std::f64::INFINITY;
    let mut h = Histogram::from_ranges(
        [-inf, -0.4, -0.3, -0.2, -0.1, 0.0, 0.1, 0.2, 0.3, 0.4, inf].iter().cloned()).unwrap();
    for &i in &[-100., -0.45, 0., 0.25, 0.4, 100.] {
        h.add(i).unwrap();
    }
    assert_eq!(h.bins(), &[2, 0, 0, 0, 0, 1, 0, 1, 0, 2]);
}

#[test]
fn from_ranges_invalid() {
    assert!(Histogram::from_ranges([].iter().cloned()).is_err());
    let valid = vec![0., 0.1, 0.2, 0.3, 0.4, 0.5, 0.7, 0.8, 0.9, 1.0, 2.0];
    assert!(Histogram::from_ranges(valid.iter().cloned()).is_ok());
    let mut invalid_nan = valid.clone();
    invalid_nan[3] = std::f64::NAN;
    assert!(Histogram::from_ranges(invalid_nan.iter().cloned()).is_err());
    let mut invalid_order = valid.clone();
    invalid_order[10] = 0.9;
    assert!(Histogram::from_ranges(invalid_order.iter().cloned()).is_err());
    let mut valid_empty_ranges = valid.clone();
    valid_empty_ranges[1] = 0.;
    valid_empty_ranges[10] = 1.;
}

#[test]
fn from_ranges_empty() {
    let mut h = Histogram::from_ranges(
        [0., 0., 0.2, 0.3, 0.4, 0.5, 0.5, 0.8, 0.9, 2.0, 2.0].iter().cloned()).unwrap();
    for &i in &[0.05, 0.7, 1.0, 1.5] {
        h.add(i).unwrap();
    }
    assert_eq!(h.bins(), &[0, 1, 0, 0, 0, 0, 1, 0, 2, 0]);
}

#[test]
fn out_of_range() {
    let mut h = Histogram::with_const_width(0., 100.);
    assert_eq!(h.add(-0.1), Err(()));
    assert_eq!(h.add(0.0), Ok(()));
    assert_eq!(h.add(1.0), Ok(()));
    assert_eq!(h.add(100.0), Err(()));
    assert_eq!(h.add(100.1), Err(()));
}
