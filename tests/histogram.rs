#[macro_use] extern crate average;

extern crate core;
extern crate rand;

use core::iter::Iterator;
use rand::distributions::IndependentSample;

use average::Histogram;

define_histogram!(Histogram10, 10);

#[test]
fn with_const_width() {
    let mut h = Histogram10::with_const_width(-30., 70.);
    for i in -30..70 {
        h.add(f64::from(i)).unwrap();
    }
    assert_eq!(h.bins(), &[10, 10, 10, 10, 10, 10, 10, 10, 10, 10]);
}

#[test]
fn from_ranges() {
    let mut h = Histogram10::from_ranges(
        [0., 0.1, 0.2, 0.3, 0.4, 0.5, 0.7, 0.8, 0.9, 1.0, 2.0].iter().cloned()).unwrap();
    for &i in &[0.05, 0.7, 1.0, 1.5] {
        h.add(i).unwrap();
    }
    assert_eq!(h.bins(), &[1, 0, 0, 0, 0, 0, 1, 0, 0, 2]);
}

#[test]
fn iter() {
    let mut h = Histogram10::from_ranges(
        [0., 0.1, 0.2, 0.3, 0.4, 0.5, 0.7, 0.8, 0.9, 1.0, 2.0].iter().cloned()).unwrap();
    for &i in &[0.05, 0.7, 1.0, 1.5] {
        h.add(i).unwrap();
    }
    let iterated: Vec<((f64, f64), u64)> = h.iter().collect();
    assert_eq!(&iterated, &[
        ((0., 0.1), 1), ((0.1, 0.2), 0), ((0.2, 0.3), 0), ((0.3, 0.4), 0),
        ((0.4, 0.5), 0), ((0.5, 0.7), 0), ((0.7, 0.8), 1), ((0.8, 0.9), 0),
        ((0.9, 1.0), 0), ((1.0, 2.0), 2)
    ]);
}

#[test]
fn normalized_bins() {
    let inf = std::f64::INFINITY;
    let mut h = Histogram10::from_ranges(
        [-inf, 0.1, 0.2, 0.3, 0.4, 0.4, 0.7, 0.8, 0.9, 1.0, inf].iter().cloned()).unwrap();
    for &i in &[0.05, 0.1, 0.7, 1.0, 1.5] {
        h.add(i).unwrap();
    }
    let normalized: Vec<f64> = h.normalized_bins().collect();
    let expected = [0., 10., 0., 0., 0., 0., 10., 0., 0., 0.];
    for (a, b) in normalized.iter().zip(expected.iter()) {
        assert_almost_eq!(a, b, 1e-14);
    }
}

#[test]
fn widths() {
    let inf = std::f64::INFINITY;
    let h = Histogram10::from_ranges(
        [-inf, 0.1, 0.2, 0.3, 0.4, 0.4, 0.7, 0.8, 0.9, 1.0, inf].iter().cloned()).unwrap();
    let widths: Vec<f64> = h.widths().collect();
    let expected = [inf, 0.1, 0.1, 0.1, 0., 0.3, 0.1, 0.1, 0.1, inf];
    for (a, b) in widths.iter().zip(expected.iter()) {
        assert_almost_eq!(a, b, 1e-14);
    }
}

#[test]
fn centers() {
    let inf = std::f64::INFINITY;
    let h = Histogram10::from_ranges(
        [-inf, 0.1, 0.2, 0.3, 0.4, 0.4, 0.7, 0.8, 0.9, 1.0, inf].iter().cloned()).unwrap();
    let centers: Vec<f64> = h.centers().collect();
    let expected = [-inf, 0.15, 0.25, 0.35, 0.4, 0.55, 0.75, 0.85, 0.95, inf];
    for (a, b) in centers.iter().zip(expected.iter()) {
        assert_almost_eq!(a, b, 1e-14);
    }
}

#[test]
fn from_ranges_infinity() {
    let inf = std::f64::INFINITY;
    let mut h = Histogram10::from_ranges(
        [-inf, -0.4, -0.3, -0.2, -0.1, 0.0, 0.1, 0.2, 0.3, 0.4, inf].iter().cloned()).unwrap();
    for &i in &[-100., -0.45, 0., 0.25, 0.4, 100.] {
        h.add(i).unwrap();
    }
    assert_eq!(h.bins(), &[2, 0, 0, 0, 0, 1, 0, 1, 0, 2]);
}

#[test]
fn from_ranges_invalid() {
    assert!(Histogram10::from_ranges([].iter().cloned()).is_err());
    let valid = vec![0., 0.1, 0.2, 0.3, 0.4, 0.5, 0.7, 0.8, 0.9, 1.0, 2.0];
    assert!(Histogram10::from_ranges(valid.iter().cloned()).is_ok());
    let mut invalid_nan = valid.clone();
    invalid_nan[3] = std::f64::NAN;
    assert!(Histogram10::from_ranges(invalid_nan.iter().cloned()).is_err());
    let mut invalid_order = valid.clone();
    invalid_order[10] = 0.9;
    assert!(Histogram10::from_ranges(invalid_order.iter().cloned()).is_err());
    let mut valid_empty_ranges = valid.clone();
    valid_empty_ranges[1] = 0.;
    valid_empty_ranges[10] = 1.;
}

#[test]
fn from_ranges_empty() {
    let mut h = Histogram10::from_ranges(
        [0., 0., 0.2, 0.3, 0.4, 0.5, 0.5, 0.8, 0.9, 2.0, 2.0].iter().cloned()).unwrap();
    for &i in &[0.05, 0.7, 1.0, 1.5] {
        h.add(i).unwrap();
    }
    assert_eq!(h.bins(), &[0, 1, 0, 0, 0, 0, 1, 0, 2, 0]);
}

#[test]
fn out_of_range() {
    let mut h = Histogram10::with_const_width(0., 100.);
    assert_eq!(h.add(-0.1), Err(()));
    assert_eq!(h.add(0.0), Ok(()));
    assert_eq!(h.add(1.0), Ok(()));
    assert_eq!(h.add(100.0), Err(()));
    assert_eq!(h.add(100.1), Err(()));
}


#[test]
fn reset() {
    let mut h = Histogram10::with_const_width(0., 100.);
    for i in 0..100 {
        h.add(f64::from(i)).unwrap();
    }
    assert_eq!(h.bins(), &[10, 10, 10, 10, 10, 10, 10, 10, 10, 10]);
    h.reset();
    assert_eq!(h.bins(), &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn range_minmax() {
    let h = Histogram10::with_const_width(0., 100.);
    assert_eq!(h.range_min(), 0.);
    assert_eq!(h.range_max(), 100.);
}

#[test]
fn add() {
    let mut h1 = Histogram10::with_const_width(0., 100.);
    let mut h2 = h1.clone();
    let mut expected = h1.clone();

    for i in 0..50 {
        h1.add(f64::from(i)).unwrap();
        expected.add(f64::from(i)).unwrap();
    }
    for i in 50..100 {
        h2.add(f64::from(i)).unwrap();
        expected.add(f64::from(i)).unwrap();
    }
    h1 += &h2;

    assert_eq!(h1.bins(), expected.bins());
}

#[test]
fn mul() {
    let mut h = Histogram10::with_const_width(0., 100.);
    let mut expected = h.clone();

    for i in 0..100 {
        h.add(f64::from(i)).unwrap();
        expected.add(f64::from(i)).unwrap();
        expected.add(f64::from(i)).unwrap();
    }

    h *= 2;

    assert_eq!(h.bins(), expected.bins());
}

#[test]
fn variance() {
    let mut h = Histogram10::with_const_width(-3., 3.);
    let normal = rand::distributions::Normal::new(0., 1.);
    let mut rng = rand::weak_rng();
    for _ in 0..1000000 {
        let _ = h.add(normal.ind_sample(&mut rng));
    }
    println!("{:?}", h);
    let sum: u64 = h.bins().iter().sum();
    let sum = sum as f64;
    for (i, v) in h.variances().enumerate() {
        assert_almost_eq!(v, h.variance(i), 1e-14);
        let poissonian_variance = h.bins()[i] as f64;
        assert_almost_eq!(v.sqrt() / sum, poissonian_variance.sqrt() / sum, 1e-4);
    }
}
