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

// ── append ────────────────────────────────────────────────────────────────────

#[cfg(any(feature = "std", feature = "libm"))]
fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 1e-12,
        "expected {expected}, got {actual}, diff {}",
        (actual - expected).abs(),
    );
}

#[cfg(any(feature = "std", feature = "libm"))]
#[test]
fn mean_append_matches_sequential() {
    let alpha = 0.3;
    let samples = [1.0_f64, 4.0, 9.0, 16.0, 25.0, 36.0, 49.0];
    for split in 1..samples.len() {
        let mut full = ExpMovingMean::with_alpha(alpha);
        full.extend(samples);

        let mut a = ExpMovingMean::with_alpha(alpha);
        a.extend(&samples[..split]);
        let mut b = ExpMovingMean::with_alpha(alpha);
        b.extend(&samples[split..]);
        a.append(&b);

        assert_eq!(a.len(), full.len());
        assert_close(a.mean(), full.mean());
    }
}

#[cfg(any(feature = "std", feature = "libm"))]
#[test]
fn mean_append_three_way() {
    let alpha = 0.4;
    let samples = [10.0_f64, 20.0, 30.0, 40.0, 50.0, 60.0];
    let mut full = ExpMovingMean::with_alpha(alpha);
    full.extend(samples);

    let mut a = ExpMovingMean::with_alpha(alpha);
    a.extend(&samples[..2]);
    let mut b = ExpMovingMean::with_alpha(alpha);
    b.extend(&samples[2..4]);
    let mut c = ExpMovingMean::with_alpha(alpha);
    c.extend(&samples[4..]);

    a.append(&b);
    a.append(&c);

    assert_eq!(a.len(), full.len());
    assert_close(a.mean(), full.mean());
}

#[cfg(any(feature = "std", feature = "libm"))]
#[test]
fn mean_append_empty_other_is_noop() {
    let mut a = ExpMovingMean::with_alpha(0.5);
    a.extend([1.0_f64, 2.0, 3.0]);
    let before = a.mean();
    let len_before = a.len();
    let empty = ExpMovingMean::with_alpha(0.5);
    a.append(&empty);
    assert_eq!(a.mean(), before);
    assert_eq!(a.len(), len_before);
}

#[cfg(any(feature = "std", feature = "libm"))]
#[test]
fn mean_append_into_empty_clones_other() {
    let mut a = ExpMovingMean::with_alpha(0.5);
    let mut b = ExpMovingMean::with_alpha(0.5);
    b.extend([1.0_f64, 2.0, 3.0]);
    a.append(&b);
    assert_eq!(a.len(), b.len());
    assert_eq!(a.mean(), b.mean());
}

#[cfg(any(feature = "std", feature = "libm"))]
#[test]
fn mean_append_alpha_one() {
    // alpha = 1 makes the EWMA a last-value tracker. Appending should behave
    // accordingly: result tracks `other`'s last value.
    let mut a = ExpMovingMean::with_alpha(1.0);
    a.extend([10.0_f64, 20.0]);
    let mut b = ExpMovingMean::with_alpha(1.0);
    b.extend([100.0_f64, 200.0]);
    a.append(&b);
    assert_eq!(a.mean(), 200.0);
    assert_eq!(a.len(), 4);
}

#[cfg(any(feature = "std", feature = "libm"))]
#[test]
fn mean_append_underflow_is_stable() {
    // For α = 0.5, decay underflows to 0.0 around n ≈ 1074. With n_b = 2000
    // the older chunk's contribution is numerically zero — the merged mean
    // should equal `other.mean()` without producing NaN.
    let alpha = 0.5;
    let mut a = ExpMovingMean::with_alpha(alpha);
    a.add(1.0e10);
    let mut b = ExpMovingMean::with_alpha(alpha);
    for i in 0..2000 {
        b.add(i as f64);
    }
    let b_mean = b.mean();
    a.append(&b);
    assert!(a.mean().is_finite());
    assert_eq!(a.mean(), b_mean);
}

#[cfg(any(feature = "std", feature = "libm"))]
#[test]
fn variance_append_matches_sequential() {
    let alpha = 0.3;
    let samples = [1.0_f64, 4.0, 9.0, 16.0, 25.0, 36.0, 49.0];
    for split in 1..samples.len() {
        let mut full = ExpMovingVariance::with_alpha(alpha);
        full.extend(samples);

        let mut a = ExpMovingVariance::with_alpha(alpha);
        a.extend(&samples[..split]);
        let mut b = ExpMovingVariance::with_alpha(alpha);
        b.extend(&samples[split..]);
        a.append(&b);

        assert_eq!(a.len(), full.len());
        assert_close(a.mean(), full.mean());
        assert_close(a.population_variance(), full.population_variance());
        assert_close(a.sample_variance(), full.sample_variance());
    }
}

#[cfg(any(feature = "std", feature = "libm"))]
#[test]
fn variance_append_three_way() {
    let alpha = 0.4;
    let samples = [10.0_f64, 20.0, 30.0, 40.0, 50.0, 60.0];
    let mut full = ExpMovingVariance::with_alpha(alpha);
    full.extend(samples);

    let mut a = ExpMovingVariance::with_alpha(alpha);
    a.extend(&samples[..2]);
    let mut b = ExpMovingVariance::with_alpha(alpha);
    b.extend(&samples[2..4]);
    let mut c = ExpMovingVariance::with_alpha(alpha);
    c.extend(&samples[4..]);
    a.append(&b);
    a.append(&c);

    assert_close(a.mean(), full.mean());
    assert_close(a.population_variance(), full.population_variance());
    assert_close(a.sample_variance(), full.sample_variance());
}

#[cfg(any(feature = "std", feature = "libm"))]
#[test]
fn variance_append_empty_edges() {
    let mut a = ExpMovingVariance::with_alpha(0.5);
    a.extend([1.0_f64, 2.0, 3.0]);
    let before_mean = a.mean();
    let before_var = a.population_variance();
    let empty = ExpMovingVariance::with_alpha(0.5);
    a.append(&empty);
    assert_eq!(a.mean(), before_mean);
    assert_eq!(a.population_variance(), before_var);

    let mut a2 = ExpMovingVariance::with_alpha(0.5);
    let mut b = ExpMovingVariance::with_alpha(0.5);
    b.extend([1.0_f64, 2.0, 3.0]);
    a2.append(&b);
    assert_eq!(a2.mean(), b.mean());
    assert_eq!(a2.population_variance(), b.population_variance());
}

// ── Compile-fail: Merge is not implemented ────────────────────────────────────

/// Verify that `ExpMovingMean` and `ExpMovingVariance` intentionally do **not**
/// implement `Merge` — merging is undefined because EW statistics are
/// order-dependent. This prevents accidentally calling `merge` on them.
#[test]
fn exp_moving_no_merge_compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile-fail/exp_moving_no_merge.rs");
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
