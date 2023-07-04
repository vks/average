/// Estimate the arithmetic mean and the variance of a sequence of numbers
/// ("population").
///
/// This can be used to estimate the standard error of the mean.
///
///
/// ## Example
///
/// ```
/// use average::Variance;
///
/// let a: Variance = (1..6).map(f64::from).collect();
/// println!("The mean is {} ± {}.", a.mean(), a.error());
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Variance {
    /// Estimator of average.
    avg: Mean,
    /// Intermediate sum of squares for calculating the variance.
    sum_2: f64,
}

impl Variance {
    /// Create a new variance estimator.
    #[inline]
    pub fn new() -> Variance {
        Variance { avg: Mean::new(), sum_2: 0. }
    }

    /// Increment the sample size.
    ///
    /// This does not update anything else.
    #[inline]
    fn increment(&mut self) {
        self.avg.increment();
    }

    /// Add an observation given an already calculated difference from the mean
    /// divided by the number of samples, assuming the inner count of the sample
    /// size was already updated.
    ///
    /// This is useful for avoiding unnecessary divisions in the inner loop.
    #[inline]
    fn add_inner(&mut self, delta_n: f64) {
        // This algorithm introduced by Welford in 1962 trades numerical
        // stability for a division inside the loop.
        //
        // See https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance.
        let n = self.avg.len().to_f64().unwrap();
        self.avg.add_inner(delta_n);
        self.sum_2 += delta_n * delta_n * n * (n - 1.);
    }

    /// Determine whether the sample is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.avg.is_empty()
    }

    /// Estimate the mean of the population.
    ///
    /// Returns NaN for an empty sample.
    #[inline]
    pub fn mean(&self) -> f64 {
        self.avg.mean()
    }

    /// Return the sample size.
    #[inline]
    pub fn len(&self) -> u64 {
        self.avg.len()
    }

    /// Calculate the sample variance.
    ///
    /// This is an unbiased estimator of the variance of the population.
    /// 
    /// Returns NaN for samples of size 1 or less.
    #[inline]
    pub fn sample_variance(&self) -> f64 {
        if self.avg.len() < 2 {
            return f64::NAN;
        }
        self.sum_2 / (self.avg.len() - 1).to_f64().unwrap()
    }

    /// Calculate the population variance of the sample.
    ///
    /// This is a biased estimator of the variance of the population.
    /// 
    /// Returns NaN for an empty sample.
    #[inline]
    pub fn population_variance(&self) -> f64 {
        let n = self.avg.len();
        if n == 0 {
            return f64::NAN;
        }
        self.sum_2 / n.to_f64().unwrap()
    }

    /// Estimate the variance of the mean of the population.
    /// 
    /// Returns NaN for an empty sample.
    #[inline]
    pub fn variance_of_mean(&self) -> f64 {
        let n = self.avg.len();
        if n == 0 {
            return f64::NAN;
        }
        if n == 1 {
            return 0.;
        }
        self.sample_variance() / n.to_f64().unwrap()
    }

    /// Estimate the standard error of the mean of the population.
    /// 
    /// Returns NaN for an empty sample.
    #[cfg(any(feature = "std", feature = "libm"))]
    #[cfg_attr(doc_cfg, doc(cfg(any(feature = "std", feature = "libm"))))]
    #[inline]
    pub fn error(&self) -> f64 {
        num_traits::Float::sqrt(self.variance_of_mean())
    }

}

impl core::default::Default for Variance {
    fn default() -> Variance {
        Variance::new()
    }
}

impl Estimate for Variance {
    #[inline]
    fn add(&mut self, sample: f64) {
        self.increment();
        let delta_n = (sample - self.avg.mean())
            / self.len().to_f64().unwrap();
        self.add_inner(delta_n);
    }

    #[inline]
    fn estimate(&self) -> f64 {
        self.population_variance()
    }
}

impl Merge for Variance {
    /// Merge another sample into this one.
    ///
    ///
    /// ## Example
    ///
    /// ```
    /// use average::{Variance, Merge};
    ///
    /// let sequence: &[f64] = &[1., 2., 3., 4., 5., 6., 7., 8., 9.];
    /// let (left, right) = sequence.split_at(3);
    /// let avg_total: Variance = sequence.iter().collect();
    /// let mut avg_left: Variance = left.iter().collect();
    /// let avg_right: Variance = right.iter().collect();
    /// avg_left.merge(&avg_right);
    /// assert_eq!(avg_total.mean(), avg_left.mean());
    /// assert_eq!(avg_total.sample_variance(), avg_left.sample_variance());
    /// ```
    #[inline]
    fn merge(&mut self, other: &Variance) {
        if other.is_empty() {
            return;
        }
        if self.is_empty() {
            *self = other.clone();
            return;
        }
        // This algorithm was proposed by Chan et al. in 1979.
        //
        // See https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance.
        let len_self = self.len().to_f64().unwrap();
        let len_other = other.len().to_f64().unwrap();
        let len_total = len_self + len_other;
        let delta = other.mean() - self.mean();
        self.avg.merge(&other.avg);
        self.sum_2 += other.sum_2 + delta*delta * len_self * len_other / len_total;
    }
}

impl_from_iterator!(Variance);
impl_from_par_iterator!(Variance);
impl_extend!(Variance);
