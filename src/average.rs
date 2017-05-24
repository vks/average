use core;

use conv::ApproxFrom;


/// Estimate the arithmetic mean of a sequence of numbers ("population").
///
/// Everything is calculated iteratively using constant memory, so the sequence
/// of numbers can be an iterator. The used algorithms try to avoid numerical
/// instabilities.
///
///
/// ## Example
///
/// ```
/// use average::Average;
///
/// let a: Average = (1..6).map(Into::into).collect();
/// println!("The average is {}.", a.mean());
/// ```
#[derive(Debug, Clone)]
pub struct Average {
    /// Average value.
    avg: f64,
    /// Sample size.
    n: u64,
}

impl Average {
    /// Create a new average estimator.
    pub fn new() -> Average {
        Average { avg: 0., n: 0 }
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
    }

    /// Determine whether the samples are empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Estimate the mean of the population.
    #[inline]
    pub fn mean(&self) -> f64 {
        self.avg
    }

    /// Return the number of samples.
    #[inline]
    pub fn len(&self) -> u64 {
        self.n
    }

    /// Merge another sample into this one.
    ///
    ///
    /// ## Example
    ///
    /// ```
    /// use average::Average;
    ///
    /// let sequence: &[f64] = &[1., 2., 3., 4., 5., 6., 7., 8., 9.];
    /// let (left, right) = sequence.split_at(3);
    /// let avg_total: Average = sequence.iter().map(|x| *x).collect();
    /// let mut avg_left: Average = left.iter().map(|x| *x).collect();
    /// let avg_right: Average = right.iter().map(|x| *x).collect();
    /// avg_left.merge(&avg_right);
    /// assert_eq!(avg_total.mean(), avg_left.mean());
    /// ```
    #[inline]
    pub fn merge(&mut self, other: &Average) {
        // This algorithm was proposed by Chan et al. in 1979.
        //
        // See https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance.
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
    }
}

impl core::default::Default for Average {
    fn default() -> Average {
        Average::new()
    }
}

impl core::iter::FromIterator<f64> for Average {
    fn from_iter<T>(iter: T) -> Average
        where T: IntoIterator<Item=f64>
    {
        let mut a = Average::new();
        for i in iter {
            a.add(i);
        }
        a
    }
}

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
    /// Estimator of average.
    avg: Average,
    /// Intermediate sum of squares for calculating the variance.
    v: f64,
}

impl AverageWithError {
    /// Create a new average estimator.
    pub fn new() -> AverageWithError {
        AverageWithError { avg: Average::new(), v: 0. }
    }

    /// Add an element sampled from the population.
    #[inline]
    pub fn add(&mut self, sample: f64) {
        // This algorithm introduced by Welford in 1962 trades numerical
        // stability for a division inside the loop.
        //
        // See https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance.
        let delta = sample - self.avg.mean();
        self.avg.add(sample);
        self.v += delta * (sample - self.avg.mean());
    }

    /// Determine whether the samples are empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.avg.is_empty()
    }

    /// Estimate the mean of the population.
    #[inline]
    pub fn mean(&self) -> f64 {
        self.avg.mean()
    }

    /// Return the number of samples.
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
        self.v / f64::approx_from(self.avg.len() - 1).unwrap()
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
        self.v / f64::approx_from(n).unwrap()
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
    #[inline]
    pub fn merge(&mut self, other: &AverageWithError) {
        // This algorithm was proposed by Chan et al. in 1979.
        //
        // See https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance.
        let len_self = f64::approx_from(self.len()).unwrap();
        let len_other = f64::approx_from(other.len()).unwrap();
        let len_total = len_self + len_other;
        let delta = other.mean() - self.mean();
        self.avg.merge(&other.avg);
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
