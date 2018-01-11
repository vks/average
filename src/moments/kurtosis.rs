/// Estimate the arithmetic mean, the variance, the skewness and the kurtosis of
/// a sequence of numbers ("population").
///
/// This can be used to estimate the standard error of the mean.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Kurtosis {
    /// Estimator of mean, variance and skewness.
    avg: Skewness,
    /// Intermediate sum of terms to the fourth for calculating the skewness.
    sum_4: f64,
}

impl Kurtosis {
    /// Create a new kurtosis estimator.
    #[inline]
    pub fn new() -> Kurtosis {
        Kurtosis {
            avg: Skewness::new(),
            sum_4: 0.,
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
        let n = f64::approx_from(self.len()).unwrap();
        let term = delta * delta_n * (n - 1.);
        let delta_n_sq = delta_n*delta_n;
        self.sum_4 += term * delta_n_sq * (n*n - 3.*n + 3.)
            + 6. * delta_n_sq * self.avg.avg.sum_2
            - 4. * delta_n * self.avg.sum_3;
        self.avg.add_inner(delta, delta_n);
        println!("skewness={} kurtosis={}", self.skewness(), self.kurtosis());
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
        self.avg.sample_variance()
    }

    /// Calculate the population variance of the sample.
    ///
    /// This is a biased estimator of the variance of the population.
    #[inline]
    pub fn population_variance(&self) -> f64 {
        self.avg.population_variance()
    }

    /// Estimate the standard error of the mean of the population.
    #[inline]
    pub fn error_mean(&self) -> f64 {
        self.avg.error_mean()
    }

    /// Estimate the skewness of the population.
    #[inline]
    pub fn skewness(&self) -> f64 {
        self.avg.skewness()
    }

    /// Estimate the excess kurtosis of the population.
    #[inline]
    pub fn kurtosis(&self) -> f64 {
        if self.sum_4 == 0. {
            return 0.;
        }
        let n = f64::approx_from(self.len()).unwrap();
        n * self.sum_4 / (self.avg.avg.sum_2 * self.avg.avg.sum_2) - 3.
    }

}

impl core::default::Default for Kurtosis {
    fn default() -> Kurtosis {
        Kurtosis::new()
    }
}

impl Estimate for Kurtosis {
    #[inline]
    fn add(&mut self, x: f64) {
        let delta = x - self.mean();
        self.increment();
        let n = f64::approx_from(self.len()).unwrap();
        self.add_inner(delta, delta/n);
    }

    #[inline]
    fn estimate(&self) -> f64 {
        self.kurtosis()
    }
}

impl Merge for Kurtosis {
    #[inline]
    fn merge(&mut self, other: &Kurtosis) {
        let len_self = f64::approx_from(self.len()).unwrap();
        let len_other = f64::approx_from(other.len()).unwrap();
        let len_total = len_self + len_other;
        let delta = other.mean() - self.mean();
        let delta_n = delta / len_total;
        let delta_n_sq = delta_n * delta_n;
        self.sum_4 += other.sum_4
            + delta * delta_n*delta_n_sq * len_self*len_other
              * (len_self*len_self - len_self*len_other + len_other*len_other)
            + 6.*delta_n_sq * (len_self*len_self * other.avg.avg.sum_2 + len_other*len_other * self.avg.avg.sum_2)
            + 4.*delta_n * (len_self * other.avg.sum_3 - len_other * self.avg.sum_3);
        self.avg.merge(&other.avg);
    }
}

impl_from_iterator!(Kurtosis);
