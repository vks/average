#![no_std]

extern crate conv;
#[cfg(test)] extern crate rand;
#[cfg(test)] #[macro_use] extern crate std;

use conv::ApproxFrom;

/// Represent the arithmetic mean and the variance of a sequence of numbers.
///
/// Everything is calculated iteratively using constant memory, so the sequence
/// of numbers can be an iterator. The used algorithms try to avoid numerical
/// instabilities.
///
/// ```
/// use average::Average;
///
/// let a: Average = (1..6).map(Into::into).collect();
/// assert_eq!(a.mean(), 3.0);
/// assert_eq!(a.sample_variance(), 2.5);
/// ```
#[derive(Debug, Clone)]
pub struct Average {
    /// Average value.
    avg: f64,
    /// Number of samples.
    n: u64,
    /// Intermediate sum of squares for calculating the variance.
    v: f64,
}

impl Average {
    /// Create a new average.
    pub fn new() -> Average {
        Average { avg: 0., n: 0, v: 0. }
    }

    /// Add a number to the sequence of which the average is calculated.
    pub fn add(&mut self, x: f64) {
        // This algorithm introduced by Welford in 1962 trades numerical
        // stability for a division inside the loop.
        //
        // See https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance.
        self.n += 1;
        let delta = x - self.avg;
        self.avg += delta / f64::approx_from(self.n).unwrap();
        self.v += delta * (x - self.avg);
    }

    /// Return the mean of the sequence.
    pub fn mean(&self) -> f64 {
        self.avg
    }

    /// Return the number of elements in the sequence.
    pub fn len(&self) -> u64 {
        self.n
    }

    /// Calculate the unbiased sample variance of the sequence.
    ///
    /// This assumes that the sequence consists of samples of a larger population.
    pub fn sample_variance(&self) -> f64 {
        if self.n < 2 {
            return 0.;
        }
        self.v / f64::approx_from(self.n - 1).unwrap()
    }

    /// Calculate the population variance of the sequence.
    ///
    /// This assumes that the sequence consists of the entire population.
    pub fn population_variance(&self) -> f64 {
        if self.n < 2 {
            return 0.;
        }
        self.v / f64::approx_from(self.n).unwrap()
    }

    /// Calculate the standard error of the mean of the sequence.
    pub fn error(&self) -> f64 {
        if self.n == 0 {
            return 0.;
        }
        (self.sample_variance() / f64::approx_from(self.n).unwrap()).sqrt()
    }

    /// Merge the average of another sequence into this one.
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
    /// assert_eq!(avg_total.sample_variance(), avg_left.sample_variance());
    /// ```
    pub fn merge(&mut self, other: &Average) {
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

/// Assert that two numbers are almost equal to each other.
///
/// On panic, this macro will print the values of the expressions with their
/// debug representations.
macro_rules! assert_almost_eq {
    ($a:expr, $b:expr, $prec:expr) => (
        if ($a - $b).abs() > $prec {
            panic!(format!(
                "assertion failed: `abs(left - right) < {:e}`, \
                 (left: `{}`, right: `{}`)",
                $prec, $a, $b));
        }
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    use core::iter::Iterator;

    #[test]
    fn average_trivial() {
        let mut a = Average::new();
        assert_eq!(a.len(), 0);
        a.add(1.0);
        assert_eq!(a.mean(), 1.0);
        assert_eq!(a.len(), 1);
        assert_eq!(a.sample_variance(), 0.0);
        assert_eq!(a.error(), 0.0);
    }

    #[test]
    fn average_simple() {
        let a: Average = (1..6).map(f64::from).collect();
        assert_eq!(a.mean(), 3.0);
        assert_eq!(a.len(), 5);
        assert_eq!(a.sample_variance(), 2.5);
        assert_almost_eq!(a.error(), f64::sqrt(0.5), 1e-16);
    }

    #[test]
    fn average_numerically_unstable() {
        // The naive algorithm fails for this example due to cancelation.
        let big = 1e9;
        let sample = &[big + 4., big + 7., big + 13., big + 16.];
        let a: Average = sample.iter().map(|x| *x).collect();
        assert_eq!(a.sample_variance(), 30.);
    }

    #[test]
    fn average_normal_distribution() {
        use rand::distributions::{Normal, IndependentSample};
        let normal = Normal::new(2.0, 3.0);
        let mut a = Average::new();
        for _ in 0..1_000_000 {
            a.add(normal.ind_sample(&mut ::rand::thread_rng()));
        }
        assert_almost_eq!(a.mean(), 2.0, 1e-2);
        assert_almost_eq!(a.sample_variance().sqrt(), 3.0, 1e-2);
    }

    #[test]
    fn merge() {
        let sequence: &[f64] = &[1., 2., 3., 4., 5., 6., 7., 8., 9.];
        for mid in 0..sequence.len() {
            let (left, right) = sequence.split_at(mid);
            let avg_total: Average = sequence.iter().map(|x| *x).collect();
            let mut avg_left: Average = left.iter().map(|x| *x).collect();
            let avg_right: Average = right.iter().map(|x| *x).collect();
            avg_left.merge(&avg_right);
            assert_eq!(avg_total.n, avg_left.n);
            assert_eq!(avg_total.avg, avg_left.avg);
            assert_eq!(avg_total.v, avg_left.v);
        }
    }
}
