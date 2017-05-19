use core;

use conv::ApproxFrom;

/// Represent the weighted and unweighted arithmetic mean and the unweighted
/// variance of a sequence of numbers.
#[derive(Debug, Clone)]
pub struct WeightedAverage {
    /// Sum of the weights.
    weight_sum: f64,
    /// Sum of the squares of the weights.
    weight_sum_sq: f64,
    /// Weighted verage value.
    weighted_avg: f64,

    /// Number of samples.
    n: u64,
    /// Unweighted verage value.
    unweighted_avg: f64,
    /// Intermediate sum of squares for calculating the *unweighted* variance.
    v: f64,
}

impl WeightedAverage {
    /// Create a new weighted average.
    pub fn new() -> WeightedAverage {
        WeightedAverage {
            weight_sum: 0., weight_sum_sq: 0., weighted_avg: 0.,
            n: 0, unweighted_avg: 0., v: 0.,
        }
    }

    /// Add a sample to the weighted sequence of which the average is calculated.
    pub fn add(&mut self, sample: f64, weight: f64) {
        // The algorithm for the unweighted average was suggested by Welford in 1962.
        // The algorithm for the weighted average was suggested by West in 1979.
        //
        // See
        // https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance
        // and
        // http://people.ds.cam.ac.uk/fanf2/hermes/doc/antiforgery/stats.pdf.
        self.weight_sum += weight;
        self.weight_sum_sq += weight*weight;

        let prev_avg = self.weighted_avg;
        self.weighted_avg = prev_avg + (weight / self.weight_sum) * (sample - prev_avg);

        self.n += 1;
        let delta = sample - self.unweighted_avg;
        self.unweighted_avg += delta / f64::approx_from(self.n).unwrap();
        self.v += delta * (sample - self.unweighted_avg);
    }

    /// Determine whether the sequence is empty.
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Return the sum of the weights.
    pub fn sum_weights(&self) -> f64 {
        self.weight_sum
    }

    /// Return the sum of the squared weights.
    pub fn sum_weights_sq(&self) -> f64 {
        self.weight_sum_sq
    }

    /// Estimate the weighted mean of the sequence.
    pub fn weighted_mean(&self) -> f64 {
        self.weighted_avg
    }

    /// Estimate the unweighted mean of the sequence.
    pub fn unweighted_mean(&self) -> f64 {
        self.unweighted_avg
    }

    /// Return sample size.
    pub fn len(&self) -> u64 {
        self.n
    }

    /// Calculate the effective sample size.
    pub fn effective_len(&self) -> f64 {
        if self.is_empty() {
            return 0.
        }
        self.weight_sum * self.weight_sum / self.weight_sum_sq
    }

    /// Calculate the *unweighted* population variance of the sequence.
    ///
    /// This assumes that the sequence consists of the entire population.
    pub fn population_variance(&self) -> f64 {
        if self.n < 2 {
            return 0.;
        }
        self.v / f64::approx_from(self.n).unwrap()
    }

    /// Calculate the *unweighted*, unbiased sample variance of the sequence.
    ///
    /// This assumes that the sequence consists of samples of a larger population.
    pub fn sample_variance(&self) -> f64 {
        if self.n < 2 {
            return 0.;
        }
        self.v / f64::approx_from(self.n - 1).unwrap()
    }

    /// Estimate the standard error of the weighted mean of the sequence.
    ///
    /// Returns 0 if the sum of weights is 0.
    pub fn error(&self) -> f64 {
        // This uses the same estimate as WinCross.
        //
        // See http://www.analyticalgroup.com/download/WEIGHTED_MEAN.pdf.
        if self.weight_sum_sq == 0. || self.weight_sum == 0. {
            return 0.;
        }
        let effective_base = self.weight_sum * self.weight_sum / self.weight_sum_sq;
        (self.sample_variance() / effective_base).sqrt()
    }

    /// Merge the weighted average of another sequence into this one.
    ///
    /// ```
    /// use average::WeightedAverage2 as WeightedAverage;
    ///
    /// let weighted_sequence: &[(f64, f64)] = &[
    ///     (1., 0.1), (2., 0.2), (3., 0.3), (4., 0.4), (5., 0.5),
    ///     (6., 0.6), (7., 0.7), (8., 0.8), (9., 0.)];
    /// let (left, right) = weighted_sequence.split_at(3);
    /// let avg_total: WeightedAverage = weighted_sequence.iter().map(|&x| x).collect();
    /// let mut avg_left: WeightedAverage = left.iter().map(|&x| x).collect();
    /// let avg_right: WeightedAverage = right.iter().map(|&x| x).collect();
    /// avg_left.merge(&avg_right);
    /// assert!((avg_total.weighted_mean() - avg_left.weighted_mean()).abs() < 1e-15);
    /// assert!((avg_total.error() - avg_left.error()).abs() < 1e-15);
    /// ```
    pub fn merge(&mut self, other: &WeightedAverage) {
        // This is similar to the algorithm proposed by Chan et al. in 1979.
        //
        // See https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance.
        {
            let total_weight_sum = self.weight_sum + other.weight_sum;
            self.weighted_avg = (self.weight_sum * self.weighted_avg
                                 + other.weight_sum * other.weighted_avg)
                                / (self.weight_sum + other.weight_sum);
            self.weight_sum = total_weight_sum;
            self.weight_sum_sq += other.weight_sum_sq;
        }
        {
            let delta = other.unweighted_avg - self.unweighted_avg;
            let len_self = f64::approx_from(self.n).unwrap();
            let len_other = f64::approx_from(other.n).unwrap();
            let len_total = len_self + len_other;
            self.n += other.n;
            self.unweighted_avg = (len_self * self.unweighted_avg
                                   + len_other * other.unweighted_avg)
                                  / len_total;
            self.v += other.v + delta*delta * len_self * len_other / len_total;
        }
    }
}

impl core::default::Default for WeightedAverage {
    fn default() -> WeightedAverage {
        WeightedAverage::new()
    }
}

impl core::iter::FromIterator<(f64, f64)> for WeightedAverage {
    fn from_iter<T>(iter: T) -> WeightedAverage
        where T: IntoIterator<Item=(f64, f64)>
    {
        let mut a = WeightedAverage::new();
        for (i, w) in iter {
            a.add(i, w);
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_unweighted() {
        let sequence: &[f64] = &[1., 2., 3., 4., 5., 6., 7., 8., 9.];
        for mid in 0..sequence.len() {
            let (left, right) = sequence.split_at(mid);
            let avg_total: WeightedAverage = sequence.iter().map(|x| (*x, 1.)).collect();
            let mut avg_left: WeightedAverage = left.iter().map(|x| (*x, 1.)).collect();
            let avg_right: WeightedAverage = right.iter().map(|x| (*x, 1.)).collect();
            avg_left.merge(&avg_right);
            assert_eq!(avg_total.n, avg_left.n);
            assert_eq!(avg_total.weight_sum, avg_left.weight_sum);
            assert_eq!(avg_total.weight_sum_sq, avg_left.weight_sum_sq);
            assert_eq!(avg_total.weighted_avg, avg_left.weighted_avg);
            assert_eq!(avg_total.unweighted_avg, avg_left.unweighted_avg);
            assert_eq!(avg_total.v, avg_left.v);
        }
    }

    #[test]
    fn merge_weighted() {
        let sequence: &[(f64, f64)] = &[
            (1., 0.1), (2., 0.2), (3., 0.3), (4., 0.4), (5., 0.5),
            (6., 0.6), (7., 0.7), (8., 0.8), (9., 0.)];
        for mid in 0..sequence.len() {
            let (left, right) = sequence.split_at(mid);
            let avg_total: WeightedAverage = sequence.iter().map(|&(x, w)| (x, w)).collect();
            let mut avg_left: WeightedAverage = left.iter().map(|&(x, w)| (x, w)).collect();
            let avg_right: WeightedAverage = right.iter().map(|&(x, w)| (x, w)).collect();
            avg_left.merge(&avg_right);
            assert_eq!(avg_total.n, avg_left.n);
            assert_almost_eq!(avg_total.weight_sum, avg_left.weight_sum, 1e-15);
            assert_eq!(avg_total.weight_sum_sq, avg_left.weight_sum_sq);
            assert_almost_eq!(avg_total.weighted_avg, avg_left.weighted_avg, 1e-15);
            assert_almost_eq!(avg_total.unweighted_avg, avg_left.unweighted_avg, 1e-15);
            assert_almost_eq!(avg_total.v, avg_left.v, 1e-14);
        }
    }
}
