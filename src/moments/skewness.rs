use num_traits::Float;

/// Estimate the arithmetic mean, the variance and the skewness of a sequence of
/// numbers ("population").
///
/// This can be used to estimate the standard error of the mean.
#[cfg_attr(doc_cfg, doc(cfg(any(feature = "std", feature = "libm"))))]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Skewness {
    /// Estimator of mean and variance.
    avg: MeanWithError,
    /// Intermediate sum of cubes for calculating the skewness.
    sum_3: f64,
}

impl Skewness {
    /// Create a new skewness estimator.
    #[inline]
    pub fn new() -> Skewness {
        Skewness {
            avg: MeanWithError::new(),
            sum_3: 0.,
        }
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
    fn add_inner(&mut self, delta: f64, delta_n: f64) {
        // This algorithm was suggested by Terriberry.
        //
        // See https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance.
        let n = self.len().to_f64().unwrap();
        let term = delta * delta_n * (n - 1.);
        self.sum_3 += term * delta_n * (n - 2.)
            - 3.*delta_n * self.avg.sum_2;
        self.avg.add_inner(delta_n);
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
        self.avg.sample_variance()
    }

    /// Calculate the population variance of the sample.
    ///
    /// This is a biased estimator of the variance of the population.
    /// 
    /// Returns NaN for an empty sample.
    #[inline]
    pub fn population_variance(&self) -> f64 {
        self.avg.population_variance()
    }

    /// Estimate the standard error of the mean of the population.
    #[inline]
    pub fn error_mean(&self) -> f64 {
        self.avg.error()
    }

    /// Estimate the skewness of the population.
    /// 
    /// Returns NaN for an empty sample.
    #[inline]
    pub fn skewness(&self) -> f64 {
        if self.is_empty() {
            return f64::NAN;
        }
        if self.sum_3 == 0. {
            return 0.;
        }
        let n = self.len().to_f64().unwrap();
        let sum_2 = self.avg.sum_2;
        debug_assert_ne!(sum_2, 0.);
        Float::sqrt(n) * self.sum_3 / Float::sqrt(sum_2*sum_2*sum_2)
    }
}

impl Default for Skewness {
    fn default() -> Skewness {
        Skewness::new()
    }
}

impl Estimate for Skewness {
    #[inline]
    fn add(&mut self, x: f64) {
        let delta = x - self.avg.avg.avg;
        self.increment();
        let n = self.len().to_f64().unwrap();
        self.add_inner(delta, delta/n);
    }

    #[inline]
    fn estimate(&self) -> f64 {
        self.skewness()
    }
}

impl Merge for Skewness {
    #[inline]
    fn merge(&mut self, other: &Skewness) {
        if other.is_empty() {
            return;
        }
        if self.is_empty() {
            *self = other.clone();
            return;
        }
        let len_self = self.len().to_f64().unwrap();
        let len_other = other.len().to_f64().unwrap();
        let len_total = len_self + len_other;
        let delta = other.mean() - self.mean();
        let delta_n = delta / len_total;
        self.sum_3 += other.sum_3
            + delta*delta_n*delta_n * len_self*len_other*(len_self - len_other)
            + 3.*delta_n * (len_self * other.avg.sum_2 - len_other * self.avg.sum_2);
        self.avg.merge(&other.avg);
    }
}

impl_from_iterator!(Skewness);
impl_from_par_iterator!(Skewness);
impl_extend!(Skewness);
