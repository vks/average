#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Estimate;

/// Exponentially weighted moving average (EWMA / EMA) of a sequence of numbers.
///
/// This estimator assigns exponentially decaying weights to observations, so
/// that recent samples contribute more to the estimate than older ones.
///
/// The smoothing factor `alpha` (in `(0, 1]`) controls the decay rate:
/// `alpha = 1` gives a plain last-value tracker; `alpha → 0` makes old samples
/// dominate longer. See [`Self::with_span`] and [`Self::with_half_life`] for
/// intuitive ways to interpret the decay rate.
///
/// Note: merging two EW estimators is not meaningful because exponential
/// weighting is order-dependent. Therefore, [`crate::Merge`] is **not** implemented.
///
///
/// ## Example
///
/// ```
/// use average::{ExpMovingMean, Estimate};
///
/// let mut a = ExpMovingMean::with_alpha(0.5);
/// for &x in &[2.0_f64, 4.0, 8.0] {
///     a.add(x);
/// }
/// assert_eq!(a.mean(), 5.5);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExpMovingMean {
    /// Smoothing factor in (0, 1].
    alpha: f64,
    /// Current exponentially weighted mean.
    mean: f64,
    /// Number of observations added so far.
    n: u64,
}

impl ExpMovingMean {
    /// Create a new estimator with the given smoothing factor `alpha`.
    ///
    /// # Panics (debug)
    ///
    /// Panics in debug mode if `alpha` is not in `(0, 1]`.
    #[inline]
    pub fn with_alpha(alpha: f64) -> Self {
        debug_assert!(alpha > 0.0 && alpha <= 1.0, "alpha must be in (0, 1]");
        Self { alpha, mean: 0.0, n: 0 }
    }

    /// Create a new estimator using the span convention.
    ///
    /// Sets `alpha = 2 / (span + 1)`. The `span` is the approximate number
    /// of observations over which the exponential decay spreads.
    #[inline]
    pub fn with_span(span: f64) -> Self {
        Self::with_alpha(2.0 / (span + 1.0))
    }

    /// Create a new estimator with the given half-life `h`.
    ///
    /// Sets `alpha = 1 - 0.5^(1/h)`. A half-life of `h` means that the weight
    /// of an observation decays to half its original value after `h` steps.
    #[cfg(any(feature = "std", feature = "libm"))]
    #[inline]
    pub fn with_half_life(h: f64) -> Self {
        Self::with_alpha(1.0 - <f64 as num_traits::Float>::powf(0.5, 1.0 / h))
    }

    /// Return the smoothing factor `alpha`.
    #[inline]
    pub fn alpha(&self) -> f64 {
        self.alpha
    }

    /// Determine whether the sample is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Return the sample size.
    #[inline]
    pub fn len(&self) -> u64 {
        self.n
    }

    /// Return the current exponentially weighted mean.
    ///
    /// Returns NaN for an empty sample.
    #[inline]
    pub fn mean(&self) -> f64 {
        if self.n > 0 { self.mean } else { f64::NAN }
    }
}

impl Estimate for ExpMovingMean {
    /// Add a new observation.
    ///
    /// The first observation seeds the mean exactly (no bias-toward-zero
    /// transient). Subsequent observations update via:
    ///
    /// ```text
    /// mean += alpha * (x - mean)
    /// ```
    #[inline]
    fn add(&mut self, x: f64) {
        if self.n == 0 {
            self.mean = x;
        } else {
            let delta = x - self.mean;
            self.mean += self.alpha * delta;
        }
        self.n += 1;
    }

    #[inline]
    fn estimate(&self) -> f64 {
        self.mean()
    }
}

impl core::iter::Extend<f64> for ExpMovingMean {
    #[inline]
    fn extend<T: IntoIterator<Item = f64>>(&mut self, iter: T) {
        for x in iter {
            self.add(x);
        }
    }
}

impl<'a> core::iter::Extend<&'a f64> for ExpMovingMean {
    #[inline]
    fn extend<T: IntoIterator<Item = &'a f64>>(&mut self, iter: T) {
        for &x in iter {
            self.add(x);
        }
    }
}

/// Exponentially weighted moving variance (EWMVar / EMVar / EWVar) of a
/// sequence of numbers.
///
/// Tracks both the exponentially weighted mean (EWMA / EMA) and variance using
/// the recurrence from Welford (1962) adapted to exponential weighting.
///
/// The smoothing factor `alpha` (in `(0, 1]`) controls the decay rate:
/// `alpha = 1` gives a plain last-value tracker; `alpha → 0` makes old samples
/// dominate longer. See [`Self::with_span`] and [`Self::with_half_life`] for
/// intuitive ways to interpret the decay rate.
///
/// Note: merging two EW estimators is not meaningful because exponential
/// weighting is order-dependent. Therefore, [`crate::Merge`] is **not** implemented.
///
///
/// ## Example
///
/// ```
/// use average::{ExpMovingVariance, Estimate};
///
/// let mut a = ExpMovingVariance::with_alpha(0.5);
/// for &x in &[2.0_f64, 4.0, 8.0] {
///     a.add(x);
/// }
/// assert_eq!(a.mean(), 5.5);
/// assert_eq!(a.population_variance(), 6.75);
/// assert_eq!(a.sample_variance(), 10.8);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExpMovingVariance {
    /// Inner mean estimator.
    avg: ExpMovingMean,
    /// Exponentially weighted population variance estimate.
    s: f64,
    /// Sum of squared sample weights. Equals `Σ wᵢ²` where the `wᵢ` sum to 1
    /// by construction, so it lies in `(0, 1]`. Used for the bias correction
    /// in `sample_variance()`.
    weight_sum_sq: f64,
}

impl ExpMovingVariance {
    /// Create a new estimator with the given smoothing factor `alpha`.
    ///
    /// # Panics (debug)
    ///
    /// Panics in debug mode if `alpha` is not in `(0, 1]`.
    #[inline]
    pub fn with_alpha(alpha: f64) -> Self {
        Self {
            avg: ExpMovingMean::with_alpha(alpha),
            s: 0.0,
            weight_sum_sq: 0.0,
        }
    }

    /// Create a new estimator using the span convention.
    ///
    /// Sets `alpha = 2 / (span + 1)`. The `span` is the approximate number
    /// of observations over which the exponential decay spreads.
    #[inline]
    pub fn with_span(span: f64) -> Self {
        Self::with_alpha(2.0 / (span + 1.0))
    }

    /// Create a new estimator with the given half-life `h`.
    ///
    /// Sets `alpha = 1 - 0.5^(1/h)`. A half-life of `h` means that the weight
    /// of an observation decays to half its original value after `h` steps.
    #[cfg(any(feature = "std", feature = "libm"))]
    #[inline]
    pub fn with_half_life(h: f64) -> Self {
        Self::with_alpha(1.0 - <f64 as num_traits::Float>::powf(0.5, 1.0 / h))
    }

    /// Return the smoothing factor `alpha`.
    #[inline]
    pub fn alpha(&self) -> f64 {
        self.avg.alpha()
    }

    /// Determine whether the sample is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.avg.is_empty()
    }

    /// Return the sample size.
    #[inline]
    pub fn len(&self) -> u64 {
        self.avg.len()
    }

    /// Return the current exponentially weighted mean.
    ///
    /// Returns NaN for an empty sample.
    #[inline]
    pub fn mean(&self) -> f64 {
        self.avg.mean()
    }

    /// Return the population (biased) exponentially weighted variance.
    ///
    /// This is the maximum-likelihood estimate. It is biased downward because
    /// the same data is used to estimate the mean.
    ///
    /// Returns NaN for an empty sample. Returns 0.0 for a sample of size 1.
    #[inline]
    pub fn population_variance(&self) -> f64 {
        if self.avg.is_empty() { f64::NAN } else { self.s }
    }

    /// Return the sample (unbiased) exponentially weighted variance.
    ///
    /// Applies the reliability correction `1 / (1 − Σwᵢ²)` to the population
    /// variance, where the `wᵢ` are the implicit exponentially decaying
    /// weights. This is analogous to Bessel's correction for the unweighted
    /// case.
    ///
    /// Returns NaN for samples of size 1 or less, or if `alpha == 1` (in
    /// which case the bias correction is undefined because all weight sits on
    /// the last sample).
    #[inline]
    pub fn sample_variance(&self) -> f64 {
        let denom = 1.0 - self.weight_sum_sq;
        if self.avg.len() <= 1 || denom <= 0.0 {
            f64::NAN
        } else {
            self.s / denom
        }
    }

    /// Return the standard deviation, defined as `sqrt(sample_variance())`.
    ///
    /// Returns NaN under the same conditions as [`Self::sample_variance`].
    #[cfg(any(feature = "std", feature = "libm"))]
    #[inline]
    pub fn standard_deviation(&self) -> f64 {
        <f64 as num_traits::Float>::sqrt(self.sample_variance())
    }
}

impl Estimate for ExpMovingVariance {
    #[inline]
    fn add(&mut self, x: f64) {
        let alpha = self.avg.alpha;
        if self.avg.n == 0 {
            // First sample: seed the mean (variance stays 0). The seeded
            // sample carries the full weight, so Σwᵢ² starts at 1.
            self.avg.add(x);
            self.weight_sum_sq = 1.0;
        } else {
            let prev_mean = self.avg.mean;
            self.avg.add(x);
            let delta = x - prev_mean;
            self.s = (1.0 - alpha) * (self.s + alpha * delta * delta);
            // Each pre-existing weight is scaled by (1 - alpha); the new
            // sample contributes alpha.
            let one_minus_alpha = 1.0 - alpha;
            self.weight_sum_sq =
                one_minus_alpha * one_minus_alpha * self.weight_sum_sq + alpha * alpha;
        }
    }

    #[inline]
    fn estimate(&self) -> f64 {
        self.population_variance()
    }
}

impl core::iter::Extend<f64> for ExpMovingVariance {
    #[inline]
    fn extend<T: IntoIterator<Item = f64>>(&mut self, iter: T) {
        for x in iter {
            self.add(x);
        }
    }
}

impl<'a> core::iter::Extend<&'a f64> for ExpMovingVariance {
    #[inline]
    fn extend<T: IntoIterator<Item = &'a f64>>(&mut self, iter: T) {
        for &x in iter {
            self.add(x);
        }
    }
}
