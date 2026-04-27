use average::{ExpMovingMean, ExpMovingVariance, Estimate};

// ── ExpMovingMean ─────────────────────────────────────────────────────────────

#[test]
fn mean_empty() {
    let a = ExpMovingMean::with_alpha(0.5);
    assert!(a.is_empty());
    assert_eq!(a.len(), 0);
    assert!(a.mean().is_nan());
    assert!(a.estimate().is_nan());
}

#[test]
fn mean_first_sample_seeds_exactly() {
    let mut a = ExpMovingMean::with_alpha(0.5);
    a.add(42.0);
    assert_eq!(a.len(), 1);
    assert_eq!(a.mean(), 42.0);
}

#[test]
fn mean_constants_converge() {
    let mut a = ExpMovingMean::with_alpha(0.5);
    for _ in 0..100 {
        a.add(7.0);
    }
    assert_eq!(a.mean(), 7.0);
}

#[test]
fn mean_reference() {
    let mut a = ExpMovingMean::with_alpha(0.5);
    for &x in &[1.0_f64, 2.0, 4.0, 8.0, 16.0] {
        a.add(x);
    }
    assert_eq!(a.len(), 5);
    assert_eq!(a.mean(), 10.6875);
}

#[test]
fn mean_extend_f64() {
    let mut a = ExpMovingMean::with_alpha(0.5);
    a.extend([1.0_f64, 2.0, 4.0, 8.0, 16.0]);
    assert_eq!(a.mean(), 10.6875);
}

#[test]
fn mean_extend_ref_f64() {
    let mut a = ExpMovingMean::with_alpha(0.5);
    let data = [1.0_f64, 2.0, 4.0, 8.0, 16.0];
    a.extend(data.iter());
    assert_eq!(a.mean(), 10.6875);
}

#[test]
fn mean_with_span() {
    // span=1 => alpha = 2/(1+1) = 1.0
    let mut a = ExpMovingMean::with_span(1.0);
    assert_eq!(a.alpha(), 1.0);
    a.add(3.0);
    a.add(7.0);
    // alpha=1 => mean always equals last value
    assert_eq!(a.mean(), 7.0);
}

// ── ExpMovingVariance ─────────────────────────────────────────────────────────

#[test]
fn variance_empty() {
    let a = ExpMovingVariance::with_alpha(0.5);
    assert!(a.is_empty());
    assert_eq!(a.len(), 0);
    assert!(a.mean().is_nan());
    assert!(a.population_variance().is_nan());
    assert!(a.sample_variance().is_nan());
    assert!(a.estimate().is_nan());
}

#[test]
fn variance_single_sample() {
    let mut a = ExpMovingVariance::with_alpha(0.5);
    a.add(5.0);
    assert_eq!(a.len(), 1);
    assert_eq!(a.mean(), 5.0);
    assert_eq!(a.population_variance(), 0.0);
    assert!(a.sample_variance().is_nan());
}

#[test]
fn variance_reference() {
    let mut a = ExpMovingVariance::with_alpha(0.5);
    for &x in &[1.0_f64, 2.0, 4.0, 8.0, 16.0] {
        a.add(x);
    }
    assert_eq!(a.len(), 5);
    assert_eq!(a.mean(), 10.6875);
    assert_eq!(a.population_variance(), 32.08984375);
    assert_eq!(a.sample_variance(), 32.08984375 / 0.6640625);
}

#[test]
fn variance_constants_converge() {
    let mut a = ExpMovingVariance::with_alpha(0.5);
    for _ in 0..100 {
        a.add(3.0);
    }
    assert_eq!(a.mean(), 3.0);
    assert_eq!(a.population_variance(), 0.0);
    assert_eq!(a.sample_variance(), 0.0);
}

#[test]
fn variance_alpha_one_sample_variance_is_nan() {
    // alpha = 1 puts all weight on the latest sample, so the unbiased
    // correction (1 - Σwᵢ²) is exactly zero.
    let mut a = ExpMovingVariance::with_alpha(1.0);
    a.add(1.0);
    a.add(2.0);
    assert_eq!(a.population_variance(), 0.0);
    assert!(a.sample_variance().is_nan());
}

#[test]
fn variance_extend_f64() {
    let mut a = ExpMovingVariance::with_alpha(0.5);
    a.extend([1.0_f64, 2.0, 4.0, 8.0, 16.0]);
    assert_eq!(a.population_variance(), 32.08984375);
}

#[test]
fn variance_extend_ref_f64() {
    let mut a = ExpMovingVariance::with_alpha(0.5);
    let data = [1.0_f64, 2.0, 4.0, 8.0, 16.0];
    a.extend(data.iter());
    assert_eq!(a.population_variance(), 32.08984375);
}

#[cfg(any(feature = "std", feature = "libm"))]
#[test]
fn variance_standard_deviation() {
    let mut a = ExpMovingVariance::with_alpha(0.5);
    for &x in &[1.0_f64, 2.0, 4.0, 8.0, 16.0] {
        a.add(x);
    }
    assert_eq!(a.standard_deviation(), a.sample_variance().sqrt());
}

#[cfg(any(feature = "std", feature = "libm"))]
#[test]
fn mean_with_half_life() {
    // After seeding with 1.0 and feeding `h` zeros, the mean should equal 0.5
    // by the definition of half-life: (1 - alpha)^h = 0.5.
    let h = 3.0_f64;
    let mut a = ExpMovingMean::with_half_life(h);
    a.add(1.0);
    for _ in 0..(h as usize) {
        a.add(0.0);
    }
    assert!((a.mean() - 0.5).abs() < 1e-15);
}

#[cfg(any(feature = "std", feature = "libm"))]
#[test]
fn variance_with_half_life() {
    // Same property check as mean_with_half_life: after h zeros following the
    // seed, the running mean should have decayed to 0.5.
    let h = 3.0_f64;
    let mut a = ExpMovingVariance::with_half_life(h);
    a.add(1.0);
    for _ in 0..(h as usize) {
        a.add(0.0);
    }
    assert!((a.mean() - 0.5).abs() < 1e-15);
}

// ── Serde round-trip ──────────────────────────────────────────────────────────

#[cfg(feature = "serde")]
#[test]
fn mean_serde_round_trip() {
    let mut a = ExpMovingMean::with_alpha(0.5);
    for &x in &[1.0_f64, 2.0, 4.0, 8.0, 16.0] {
        a.add(x);
    }
    let json = serde_json::to_string(&a).unwrap();
    let b: ExpMovingMean = serde_json::from_str(&json).unwrap();
    assert_eq!(b.len(), a.len());
    assert_eq!(b.alpha(), a.alpha());
    assert_eq!(b.mean(), a.mean());
}

#[cfg(feature = "serde")]
#[test]
fn variance_serde_round_trip() {
    let mut a = ExpMovingVariance::with_alpha(0.5);
    for &x in &[1.0_f64, 2.0, 4.0, 8.0, 16.0] {
        a.add(x);
    }
    let json = serde_json::to_string(&a).unwrap();
    let b: ExpMovingVariance = serde_json::from_str(&json).unwrap();
    assert_eq!(b.len(), a.len());
    assert_eq!(b.alpha(), a.alpha());
    assert_eq!(b.mean(), a.mean());
    assert_eq!(b.population_variance(), a.population_variance());
    assert_eq!(b.sample_variance(), a.sample_variance());
}
