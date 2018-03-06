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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
        let n = f64::approx_from(self.avg.len()).unwrap();
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
    /// Returns 0 for an empty sample.
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
    #[inline]
    pub fn sample_variance(&self) -> f64 {
        if self.avg.len() < 2 {
            return 0.;
        }
        self.sum_2 / f64::approx_from(self.avg.len() - 1).unwrap()
    }

    /// Calculate the population variance of the sample.
    ///
    /// This is a biased estimator of the variance of the population.
    #[inline]
    pub fn population_variance(&self) -> f64 {
        let n = self.avg.len();
        if n < 2 {
            return 0.;
        }
        self.sum_2 / f64::approx_from(n).unwrap()
    }

    /// Estimate the standard error of the mean of the population.
    #[inline]
    pub fn error(&self) -> f64 {
        let n = self.avg.len();
        if n == 0 {
            return 0.;
        }
        (self.sample_variance() / f64::approx_from(n).unwrap()).sqrt()
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
            / f64::approx_from(self.len()).unwrap();
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
        // This algorithm was proposed by Chan et al. in 1979.
        //
        // See https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance.
        let len_self = f64::approx_from(self.len()).unwrap();
        let len_other = f64::approx_from(other.len()).unwrap();
        let len_total = len_self + len_other;
        let delta = other.mean() - self.mean();
        self.avg.merge(&other.avg);
        self.sum_2 += other.sum_2 + delta*delta * len_self * len_other / len_total;
    }
}

impl_from_iterator!(Variance);
