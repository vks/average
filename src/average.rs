use core;

use conv::ApproxFrom;

/// Estimate the arithmetic mean and the variance of a sequence of numbers
/// ("population").
///
/// This can be used to estimate the standard error of the mean.
///
/// Everything is calculated iteratively using constant memory, so the sequence
/// of numbers can be an iterator. The used algorithms try to avoid numerical
/// instabilities.
///
///
/// ## Example
///
/// ```
/// use average::AverageWithError;
///
/// let a: AverageWithError = (1..6).map(Into::into).collect();
/// println!("The average is {} ± {}.", a.mean(), a.error());
/// ```
#[derive(Debug, Clone)]
pub struct AverageWithError {
    /// Average value.
    avg: f64,
    /// Number of samples.
    n: u64,
    /// Intermediate sum of squares for calculating the variance.
    v: f64,
}

impl AverageWithError {
    /// Create a new average estimator.
    pub fn new() -> AverageWithError {
        AverageWithError { avg: 0., n: 0, v: 0. }
    }

    /// Add an element sampled from the population.
    #[inline]
    pub fn add(&mut self, sample: f64) {
        // This algorithm introduced by Welford in 1962 trades numerical
        // stability for a division inside the loop.
        //
        // See https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance.
        self.n += 1;
        let delta = sample - self.avg;
        self.avg += delta / f64::approx_from(self.n).unwrap();
        self.v += delta * (sample - self.avg);
    }

    /// Determine whether the samples are empty.
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Estimate the mean of the population.
    pub fn mean(&self) -> f64 {
        self.avg
    }

    /// Return the number of samples.
    pub fn len(&self) -> u64 {
        self.n
    }

    /// Calculate the sample variance.
    ///
    /// This is an unbiased estimator of the variance of the population.
    pub fn sample_variance(&self) -> f64 {
        if self.n < 2 {
            return 0.;
        }
        self.v / f64::approx_from(self.n - 1).unwrap()
    }

    /// Calculate the population variance of the sample.
    ///
    /// This is a biased estimator of the variance of the population.
    pub fn population_variance(&self) -> f64 {
        if self.n < 2 {
            return 0.;
        }
        self.v / f64::approx_from(self.n).unwrap()
    }

    /// Estimate the standard error of the mean of the population.
    pub fn error(&self) -> f64 {
        if self.n == 0 {
            return 0.;
        }
        (self.sample_variance() / f64::approx_from(self.n).unwrap()).sqrt()
    }

    /// Merge another sample into this one.
    ///
    ///
    /// ## Example
    ///
    /// ```
    /// use average::AverageWithError;
    ///
    /// let sequence: &[f64] = &[1., 2., 3., 4., 5., 6., 7., 8., 9.];
    /// let (left, right) = sequence.split_at(3);
    /// let avg_total: AverageWithError = sequence.iter().map(|x| *x).collect();
    /// let mut avg_left: AverageWithError = left.iter().map(|x| *x).collect();
    /// let avg_right: AverageWithError = right.iter().map(|x| *x).collect();
    /// avg_left.merge(&avg_right);
    /// assert_eq!(avg_total.mean(), avg_left.mean());
    /// assert_eq!(avg_total.sample_variance(), avg_left.sample_variance());
    /// ```
    pub fn merge(&mut self, other: &AverageWithError) {
        // This algorithm was proposed by Chan et al. in 1979.
        //
        // See https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance.
        let delta = other.avg - self.avg;
        let len_self = f64::approx_from(self.n).unwrap();
        let len_other = f64::approx_from(other.n).unwrap();
        let len_total = len_self + len_other;
        self.n += other.n;
        self.avg = (len_self * self.avg + len_other * other.avg) / len_total;
        // Chan et al. use
        //
        //     self.avg += delta * len_other / len_total;
        //
        // instead but this results in cancelation if the number of samples are similar.
        self.v += other.v + delta*delta * len_self * len_other / len_total;
    }
}

impl core::default::Default for AverageWithError {
    fn default() -> AverageWithError {
        AverageWithError::new()
    }
}

impl core::iter::FromIterator<f64> for AverageWithError {
    fn from_iter<T>(iter: T) -> AverageWithError
        where T: IntoIterator<Item=f64>
    {
        let mut a = AverageWithError::new();
        for i in iter {
            a.add(i);
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge() {
        let sequence: &[f64] = &[1., 2., 3., 4., 5., 6., 7., 8., 9.];
        for mid in 0..sequence.len() {
            let (left, right) = sequence.split_at(mid);
            let avg_total: AverageWithError = sequence.iter().map(|x| *x).collect();
            let mut avg_left: AverageWithError = left.iter().map(|x| *x).collect();
            let avg_right: AverageWithError = right.iter().map(|x| *x).collect();
            avg_left.merge(&avg_right);
            assert_eq!(avg_total.n, avg_left.n);
            assert_eq!(avg_total.avg, avg_left.avg);
            assert_eq!(avg_total.v, avg_left.v);
        }
    }
}
