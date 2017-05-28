use core;

use conv::ApproxFrom;


/// Estimate the arithmetic mean of a sequence of numbers ("population").
///
///
/// ## Example
///
/// ```
/// use average::Mean;
///
/// let a: Mean = (1..6).map(Into::into).collect();
/// println!("The mean is {}.", a.mean());
/// ```
#[derive(Debug, Clone)]
pub struct Mean {
    /// Mean value.
    avg: f64,
    /// Sample size.
    n: u64,
}

impl Mean {
    /// Create a new mean estimator.
    #[inline]
    pub fn new() -> Mean {
        Mean { avg: 0., n: 0 }
    }

    /// Add an observation sampled from the population.
    #[inline]
    pub fn add(&mut self, sample: f64) {
        self.increment();
        let delta_n = (sample - self.avg)
            / f64::approx_from(self.n).unwrap();
        self.add_inner(delta_n);
    }

    /// Increment the sample size.
    ///
    /// This does not update anything else.
    #[inline]
    fn increment(&mut self) {
        self.n += 1;
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
        self.avg += delta_n;
    }

    /// Determine whether the sample is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Estimate the mean of the population.
    ///
    /// Returns 0 for an empty sample.
    #[inline]
    pub fn mean(&self) -> f64 {
        self.avg
    }

    /// Return the sample size.
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
    /// use average::Mean;
    ///
    /// let sequence: &[f64] = &[1., 2., 3., 4., 5., 6., 7., 8., 9.];
    /// let (left, right) = sequence.split_at(3);
    /// let avg_total: Mean = sequence.iter().map(|x| *x).collect();
    /// let mut avg_left: Mean = left.iter().map(|x| *x).collect();
    /// let avg_right: Mean = right.iter().map(|x| *x).collect();
    /// avg_left.merge(&avg_right);
    /// assert_eq!(avg_total.mean(), avg_left.mean());
    /// ```
    #[inline]
    pub fn merge(&mut self, other: &Mean) {
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

impl core::default::Default for Mean {
    fn default() -> Mean {
        Mean::new()
    }
}

impl core::iter::FromIterator<f64> for Mean {
    fn from_iter<T>(iter: T) -> Mean
        where T: IntoIterator<Item=f64>
    {
        let mut a = Mean::new();
        for i in iter {
            a.add(i);
        }
        a
    }
}
