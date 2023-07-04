use super::{Estimate, MeanWithError, Merge};
#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

/// Estimate the weighted and unweighted arithmetic mean of a sequence of
/// numbers ("population").
///
///
/// ## Example
///
/// ```
/// use average::WeightedMean;
///
/// let a: WeightedMean = (1..6).zip(1..6)
///     .map(|(x, w)| (f64::from(x), f64::from(w))).collect();
/// println!("The weighted mean is {}.", a.mean());
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct WeightedMean {
    /// Sum of the weights.
    weight_sum: f64,
    /// Weighted mean value.
    weighted_avg: f64,
}

impl WeightedMean {
    /// Create a new weighted and unweighted mean estimator.
    pub fn new() -> WeightedMean {
        WeightedMean {
            weight_sum: 0.,
            weighted_avg: 0.,
        }
    }

    /// Add an observation sampled from the population.
    #[inline]
    pub fn add(&mut self, sample: f64, weight: f64) {
        // The algorithm for the unweighted mean was suggested by Welford in 1962.
        //
        // See
        // https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance
        // and
        // http://people.ds.cam.ac.uk/fanf2/hermes/doc/antiforgery/stats.pdf.
        self.weight_sum += weight;

        let prev_avg = self.weighted_avg;
        self.weighted_avg = prev_avg + (weight / self.weight_sum) * (sample - prev_avg);
    }

    /// Determine whether the sample is empty.
    ///
    /// Might be a false positive if the sum of weights is zero.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.weight_sum == 0.
    }

    /// Return the sum of the weights.
    ///
    /// Returns 0 for an empty sample.
    #[inline]
    pub fn sum_weights(&self) -> f64 {
        self.weight_sum
    }

    /// Estimate the weighted mean of the population.
    ///
    /// Returns NaN for an empty sample, or if the sum of weights is zero.
    #[inline]
    pub fn mean(&self) -> f64 {
        if !self.is_empty() { self.weighted_avg } else { f64::NAN }
    }
}

impl core::default::Default for WeightedMean {
    fn default() -> WeightedMean {
        WeightedMean::new()
    }
}

impl core::iter::FromIterator<(f64, f64)> for WeightedMean {
    fn from_iter<T>(iter: T) -> WeightedMean
    where
        T: IntoIterator<Item = (f64, f64)>,
    {
        let mut a = WeightedMean::new();
        for (i, w) in iter {
            a.add(i, w);
        }
        a
    }
}

impl core::iter::Extend<(f64, f64)> for WeightedMean {
    fn extend<T: IntoIterator<Item = (f64, f64)>>(&mut self, iter: T) {
        for (i, w) in iter {
            self.add(i, w);
        }
    }
}

impl<'a> core::iter::FromIterator<&'a (f64, f64)> for WeightedMean {
    fn from_iter<T>(iter: T) -> WeightedMean
    where
        T: IntoIterator<Item = &'a (f64, f64)>,
    {
        let mut a = WeightedMean::new();
        for &(i, w) in iter {
            a.add(i, w);
        }
        a
    }
}

impl<'a> core::iter::Extend<&'a (f64, f64)> for WeightedMean {
    fn extend<T: IntoIterator<Item = &'a (f64, f64)>>(&mut self, iter: T) {
        for &(i, w) in iter {
            self.add(i, w);
        }
    }
}

impl Merge for WeightedMean {
    /// Merge another sample into this one.
    ///
    ///
    /// ## Example
    ///
    /// ```
    /// use average::{WeightedMean, Merge};
    ///
    /// let weighted_sequence: &[(f64, f64)] = &[
    ///     (1., 0.1), (2., 0.2), (3., 0.3), (4., 0.4), (5., 0.5),
    ///     (6., 0.6), (7., 0.7), (8., 0.8), (9., 0.9)];
    /// let (left, right) = weighted_sequence.split_at(3);
    /// let avg_total: WeightedMean = weighted_sequence.iter().collect();
    /// let mut avg_left: WeightedMean = left.iter().collect();
    /// let avg_right: WeightedMean = right.iter().collect();
    /// avg_left.merge(&avg_right);
    /// assert!((avg_total.mean() - avg_left.mean()).abs() < 1e-15);
    /// ```
    #[inline]
    fn merge(&mut self, other: &WeightedMean) {
        if other.is_empty() {
            return;
        }
        if self.is_empty() {
            *self = other.clone();
            return;
        }
        let total_weight_sum = self.weight_sum + other.weight_sum;
        self.weighted_avg = (self.weight_sum * self.weighted_avg
            + other.weight_sum * other.weighted_avg)
            / total_weight_sum;
        self.weight_sum = total_weight_sum;
    }
}

/// Estimate the weighted and unweighted arithmetic mean and the unweighted
/// variance of a sequence of numbers ("population").
///
/// This can be used to estimate the standard error of the weighted mean.
///
///
/// ## Example
///
/// ```
/// use average::WeightedMeanWithError;
///
/// let a: WeightedMeanWithError = (1..6).zip(1..6)
///     .map(|(x, w)| (f64::from(x), f64::from(w))).collect();
/// println!("The weighted mean is {} ± {}.", a.weighted_mean(), a.error());
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct WeightedMeanWithError {
    /// Sum of the squares of the weights.
    weight_sum_sq: f64,
    /// Estimator of the weighted mean.
    weighted_avg: WeightedMean,
    /// Estimator of unweighted mean and its variance.
    unweighted_avg: MeanWithError,
}

impl WeightedMeanWithError {
    /// Create a new weighted and unweighted mean estimator.
    #[inline]
    pub fn new() -> WeightedMeanWithError {
        WeightedMeanWithError {
            weight_sum_sq: 0.,
            weighted_avg: WeightedMean::new(),
            unweighted_avg: MeanWithError::new(),
        }
    }

    /// Add an observation sampled from the population.
    #[inline]
    pub fn add(&mut self, sample: f64, weight: f64) {
        // The algorithm for the unweighted mean was suggested by Welford in 1962.
        // The algorithm for the weighted mean was suggested by West in 1979.
        //
        // See
        // https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance
        // and
        // http://people.ds.cam.ac.uk/fanf2/hermes/doc/antiforgery/stats.pdf.
        self.weight_sum_sq += weight * weight;
        self.weighted_avg.add(sample, weight);
        self.unweighted_avg.add(sample);
    }

    /// Determine whether the sample is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.unweighted_avg.is_empty()
    }

    /// Return the sum of the weights.
    ///
    /// Returns 0 for an empty sample.
    #[inline]
    pub fn sum_weights(&self) -> f64 {
        self.weighted_avg.sum_weights()
    }

    /// Return the sum of the squared weights.
    ///
    /// Returns 0 for an empty sample.
    #[inline]
    pub fn sum_weights_sq(&self) -> f64 {
        self.weight_sum_sq
    }

    /// Estimate the weighted mean of the population.
    ///
    /// Returns NaN for an empty sample, or if the sum of weights is zero.
    #[inline]
    pub fn weighted_mean(&self) -> f64 {
        self.weighted_avg.mean()
    }

    /// Estimate the unweighted mean of the population.
    ///
    /// Returns NaN for an empty sample.
    #[inline]
    pub fn unweighted_mean(&self) -> f64 {
        self.unweighted_avg.mean()
    }

    /// Return the sample size.
    #[inline]
    pub fn len(&self) -> u64 {
        self.unweighted_avg.len()
    }

    /// Calculate the effective sample size.
    #[inline]
    pub fn effective_len(&self) -> f64 {
        if self.is_empty() {
            return 0.;
        }
        let weight_sum = self.weighted_avg.sum_weights();
        weight_sum * weight_sum / self.weight_sum_sq
    }

    /// Calculate the *unweighted* population variance of the sample.
    ///
    /// This is a biased estimator of the variance of the population.
    /// 
    /// Returns NaN for an empty sample.
    #[inline]
    pub fn population_variance(&self) -> f64 {
        self.unweighted_avg.population_variance()
    }

    /// Calculate the *unweighted* sample variance.
    ///
    /// This is an unbiased estimator of the variance of the population.
    /// 
    /// Returns NaN for samples of size 1 or less.
    #[inline]
    pub fn sample_variance(&self) -> f64 {
        self.unweighted_avg.sample_variance()
    }

    /// Estimate the standard error of the *weighted* mean of the population.
    ///
    /// Returns NaN if the sample is empty, or if the sum of weights is zero.
    ///
    /// This unbiased estimator assumes that the samples were independently
    /// drawn from the same population with constant variance.
    #[inline]
    pub fn variance_of_weighted_mean(&self) -> f64 {
        // This uses the same estimate as WinCross, which should provide better
        // results than the ones used by SPSS or Mentor.
        //
        // See http://www.analyticalgroup.com/download/WEIGHTED_VARIANCE.pdf.

        let weight_sum = self.weighted_avg.sum_weights();
        if weight_sum == 0. {
            return f64::NAN;
        }
        let inv_effective_len = self.weight_sum_sq / (weight_sum * weight_sum);
        self.sample_variance() * inv_effective_len
    }

    /// Estimate the standard error of the *weighted* mean of the population.
    ///
    /// Returns NaN if the sample is empty, or if the sum of weights is zero.
    ///
    /// This unbiased estimator assumes that the samples were independently
    /// drawn from the same population with constant variance.
    #[cfg(any(feature = "std", feature = "libm"))]
    #[cfg_attr(doc_cfg, doc(cfg(any(feature = "std", feature = "libm"))))]
    #[inline]
    pub fn error(&self) -> f64 {
        num_traits::Float::sqrt(self.variance_of_weighted_mean())
    }
}

impl Merge for WeightedMeanWithError {
    /// Merge another sample into this one.
    ///
    ///
    /// ## Example
    ///
    /// ```
    /// use average::{WeightedMeanWithError, Merge};
    ///
    /// let weighted_sequence: &[(f64, f64)] = &[
    ///     (1., 0.1), (2., 0.2), (3., 0.3), (4., 0.4), (5., 0.5),
    ///     (6., 0.6), (7., 0.7), (8., 0.8), (9., 0.9)];
    /// let (left, right) = weighted_sequence.split_at(3);
    /// let avg_total: WeightedMeanWithError = weighted_sequence.iter().collect();
    /// let mut avg_left: WeightedMeanWithError = left.iter().collect();
    /// let avg_right: WeightedMeanWithError = right.iter().collect();
    /// avg_left.merge(&avg_right);
    /// assert!((avg_total.weighted_mean() - avg_left.weighted_mean()).abs() < 1e-15);
    /// assert!((avg_total.error() - avg_left.error()).abs() < 1e-15);
    /// ```
    #[inline]
    fn merge(&mut self, other: &WeightedMeanWithError) {
        self.weight_sum_sq += other.weight_sum_sq;
        self.weighted_avg.merge(&other.weighted_avg);
        self.unweighted_avg.merge(&other.unweighted_avg);
    }
}

impl core::default::Default for WeightedMeanWithError {
    fn default() -> WeightedMeanWithError {
        WeightedMeanWithError::new()
    }
}

impl core::iter::FromIterator<(f64, f64)> for WeightedMeanWithError {
    fn from_iter<T>(iter: T) -> WeightedMeanWithError
    where
        T: IntoIterator<Item = (f64, f64)>,
    {
        let mut a = WeightedMeanWithError::new();
        for (i, w) in iter {
            a.add(i, w);
        }
        a
    }
}

impl core::iter::Extend<(f64, f64)> for WeightedMeanWithError {
    fn extend<T: IntoIterator<Item = (f64, f64)>>(&mut self, iter: T) {
        for (i, w) in iter {
            self.add(i, w);
        }
    }
}

impl<'a> core::iter::FromIterator<&'a (f64, f64)> for WeightedMeanWithError {
    fn from_iter<T>(iter: T) -> WeightedMeanWithError
    where
        T: IntoIterator<Item = &'a (f64, f64)>,
    {
        let mut a = WeightedMeanWithError::new();
        for &(i, w) in iter {
            a.add(i, w);
        }
        a
    }
}

impl<'a> core::iter::Extend<&'a (f64, f64)> for WeightedMeanWithError {
    fn extend<T: IntoIterator<Item = &'a (f64, f64)>>(&mut self, iter: T) {
        for &(i, w) in iter {
            self.add(i, w);
        }
    }
}
