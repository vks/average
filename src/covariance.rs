use num_traits::ToPrimitive;
#[cfg(feature = "serde1")]
use serde_derive::{Deserialize, Serialize};

use crate::Merge;

/// Estimate the arithmetic means and the covariance of a sequence of number pairs
/// ("population").
///
/// Because the variances are calculated as well, this can be used to calculate the Pearson
/// correlation coefficient.
///
///
/// ## Example
///
/// ```
/// use average::Covariance;
///
/// let a: Covariance = [(1., 5.), (2., 4.), (3., 3.), (4., 2.), (5., 1.)].iter().cloned().collect();
/// assert_eq!(a.mean_x(), 3.);
/// assert_eq!(a.mean_y(), 3.);
/// assert_eq!(a.population_covariance(), -2.0);
/// assert_eq!(a.sample_covariance(), -2.5);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Covariance {
    avg_x: f64,
    sum_x_2: f64,
    avg_y: f64,
    sum_y_2: f64,
    sum_prod: f64,
    n: u64,
}

impl Covariance {
    /// Create a new covariance estimator.
    #[inline]
    pub fn new() -> Covariance {
        Covariance {
            avg_x: 0.,
            sum_x_2: 0.,
            avg_y: 0.,
            sum_y_2: 0.,
            sum_prod: 0.,
            n: 0,
        }
    }

    /// Add an observation sampled from the population.
    #[inline]
    pub fn add(&mut self, x: f64, y: f64) {
        self.n += 1;
        let n = self.n.to_f64().unwrap();

        let delta_x = x - self.avg_x;
        let delta_x_n = delta_x / n;
        let delta_y_n = (y - self.avg_y) / n;

        self.avg_x += delta_x_n;
        self.sum_x_2 += delta_x_n * delta_x_n * n * (n - 1.);

        self.avg_y += delta_y_n;
        self.sum_y_2 += delta_y_n * delta_y_n * n * (n - 1.);

        self.sum_prod += delta_x * (y - self.avg_y);
    }

    /// Calculate the population covariance of the sample.
    ///
    /// This is a biased estimator of the covariance of the population.
    ///
    /// Returns NaN for an empty sample.
    #[inline]
    pub fn population_covariance(&self) -> f64 {
        if self.n < 1 {
            return f64::NAN;
        }
        self.sum_prod / self.n.to_f64().unwrap()
    }

    /// Calculate the sample covariance.
    ///
    /// This is an unbiased estimator of the covariance of the population.
    ///
    /// Returns NaN for samples of size 1 or less.
    #[inline]
    pub fn sample_covariance(&self) -> f64 {
        if self.n < 2 {
            return f64::NAN;
        }
        self.sum_prod / (self.n - 1).to_f64().unwrap()
    }

    /// Calculate the population Pearson correlation coefficient.
    ///
    /// Returns NaN for an empty sample.
    #[cfg(any(feature = "std", feature = "libm"))]
    #[cfg_attr(doc_cfg, doc(cfg(any(feature = "std", feature = "libm"))))]
    #[inline]
    pub fn pearson(&self) -> f64 {
        if self.n < 2 {
            return f64::NAN;
        }
        self.sum_prod / num_traits::Float::sqrt(self.sum_x_2 * self.sum_y_2)
    }

    /// Return the sample size.
    #[inline]
    pub fn len(&self) -> u64 {
        self.n
    }

    /// Determine whether the sample is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Estimate the mean of the `x` population.
    ///
    /// Returns NaN for an empty sample.
    #[inline]
    pub fn mean_x(&self) -> f64 {
        if self.n > 0 { self.avg_x } else { f64::NAN }
    }

    /// Estimate the mean of the `y` population.
    ///
    /// Returns NaN for an empty sample.
    #[inline]
    pub fn mean_y(&self) -> f64 {
        if self.n > 0 { self.avg_y } else { f64::NAN }
    }

    /// Calculate the sample variance of `x`.
    ///
    /// This is an unbiased estimator of the variance of the population.
    ///
    /// Returns NaN for samples of size 1 or less.
    #[inline]
    pub fn sample_variance_x(&self) -> f64 {
        if self.n < 2 {
            return f64::NAN;
        }
        self.sum_x_2 / (self.n - 1).to_f64().unwrap()
    }

    /// Calculate the population variance of the sample for `x`.
    ///
    /// This is a biased estimator of the variance of the population.
    ///
    /// Returns NaN for an empty sample.
    #[inline]
    pub fn population_variance_x(&self) -> f64 {
        if self.n == 0 {
            return f64::NAN;
        }
        self.sum_x_2 / self.n.to_f64().unwrap()
    }

    /// Calculate the sample variance of `y`.
    ///
    /// This is an unbiased estimator of the variance of the population.
    ///
    /// Returns NaN for samples of size 1 or less.
    #[inline]
    pub fn sample_variance_y(&self) -> f64 {
        if self.n < 2 {
            return f64::NAN;
        }
        self.sum_y_2 / (self.n - 1).to_f64().unwrap()
    }

    /// Calculate the population variance of the sample for `y`.
    ///
    /// This is a biased estimator of the variance of the population.
    ///
    /// Returns NaN for an empty sample.
    #[inline]
    pub fn population_variance_y(&self) -> f64 {
        if self.n == 0 {
            return f64::NAN;
        }
        self.sum_y_2 / self.n.to_f64().unwrap()
    }

    // TODO: Standard deviation and standard error
}

impl core::default::Default for Covariance {
    fn default() -> Covariance {
        Covariance::new()
    }
}

impl Merge for Covariance {
    /// Merge another sample into this one.
    ///
    /// ## Example
    ///
    /// ```
    /// use average::{Covariance, Merge};
    ///
    /// let sequence: &[(f64, f64)] = &[(1., 2.), (3., 4.), (5., 6.), (7., 8.), (9., 10.)];
    /// let (left, right) = sequence.split_at(3);
    /// let cov_total: Covariance = sequence.iter().collect();
    /// let mut cov_left: Covariance = left.iter().collect();
    /// let cov_right: Covariance = right.iter().collect();
    /// cov_left.merge(&cov_right);
    /// assert_eq!(cov_total.population_covariance(), cov_left.population_covariance());
    /// ```
    #[inline]
    fn merge(&mut self, other: &Covariance) {
        if other.n == 0 {
            return;
        }
        if self.n == 0 {
            *self = other.clone();
            return;
        }

        let delta_x = other.avg_x - self.avg_x;
        let delta_y = other.avg_y - self.avg_y;
        let len_self = self.n.to_f64().unwrap();
        let len_other = other.n.to_f64().unwrap();
        let len_total = len_self + len_other;

        self.avg_x = (len_self * self.avg_x + len_other * other.avg_x) / len_total;
        self.sum_x_2 += other.sum_x_2 + delta_x*delta_x * len_self * len_other / len_total;

        self.avg_y = (len_self * self.avg_y + len_other * other.avg_y) / len_total;
        self.sum_y_2 += other.sum_y_2 + delta_y*delta_y * len_self * len_other / len_total;

        self.sum_prod += other.sum_prod + delta_x*delta_y * len_self * len_other / len_total;

        self.n += other.n;
    }
}

impl core::iter::FromIterator<(f64, f64)> for Covariance {
    fn from_iter<T>(iter: T) -> Covariance
        where
            T: IntoIterator<Item = (f64, f64)>,
    {
        let mut cov = Covariance::new();
        for (x, y) in iter {
            cov.add(x, y);
        }
        cov
    }
}

impl core::iter::Extend<(f64, f64)> for Covariance {
    fn extend<T: IntoIterator<Item = (f64, f64)>>(&mut self, iter: T) {
        for (x, y) in iter {
            self.add(x, y);
        }
    }
}

impl<'a> core::iter::FromIterator<&'a (f64, f64)> for Covariance {
    fn from_iter<T>(iter: T) -> Covariance
        where
            T: IntoIterator<Item = &'a (f64, f64)>,
    {
        let mut cov = Covariance::new();
        for &(x, y) in iter {
            cov.add(x, y);
        }
        cov
    }
}

impl<'a> core::iter::Extend<&'a (f64, f64)> for Covariance {
    fn extend<T: IntoIterator<Item = &'a (f64, f64)>>(&mut self, iter: T) {
        for &(x, y) in iter {
            self.add(x, y);
        }
    }
}
