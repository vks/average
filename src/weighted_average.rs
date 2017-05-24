use core;

use super::AverageWithError;

/// Estimate the weighted and unweighted arithmetic mean and the unweighted
/// variance of a sequence of numbers ("population").
///
/// This can be used to estimate the standard error of the weighted mean.
///
///
/// ## Example
///
/// ```
/// use average::WeightedAverageWithError;
///
/// let a: WeightedAverageWithError = (1..6).zip(1..6)
///     .map(|(x, w)| (f64::from(x), f64::from(w))).collect();
/// println!("The weighted average is {} ± {}.", a.weighted_mean(), a.error());
/// ```
#[derive(Debug, Clone)]
pub struct WeightedAverageWithError {
    /// Sum of the weights.
    weight_sum: f64,
    /// Sum of the squares of the weights.
    weight_sum_sq: f64,
    /// Weighted average value.
    weighted_avg: f64,

    /// Estimator of unweighted average and its variance.
    unweighted_avg: AverageWithError,
}

impl WeightedAverageWithError {
    /// Create a new weighted and unweighted average estimator.
    pub fn new() -> WeightedAverageWithError {
        WeightedAverageWithError {
            weight_sum: 0., weight_sum_sq: 0., weighted_avg: 0.,
            unweighted_avg: AverageWithError::new(),
        }
    }

    /// Add a weighted element sampled from the population.
    #[inline]
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

        self.unweighted_avg.add(sample);
    }

    /// Determine whether the sample is empty.
    pub fn is_empty(&self) -> bool {
        self.unweighted_avg.is_empty()
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
        self.unweighted_avg.mean()
    }

    /// Return sample size.
    pub fn len(&self) -> u64 {
        self.unweighted_avg.len()
    }

    /// Calculate the effective sample size.
    pub fn effective_len(&self) -> f64 {
        if self.is_empty() {
            return 0.
        }
        self.weight_sum * self.weight_sum / self.weight_sum_sq
    }

    /// Calculate the *unweighted* population variance of the sample.
    ///
    /// This is a biased estimator of the variance of the population.
    pub fn population_variance(&self) -> f64 {
        self.unweighted_avg.population_variance()
    }

    /// Calculate the *unweighted* sample variance.
    ///
    /// This is an unbiased estimator of the variance of the population.
    pub fn sample_variance(&self) -> f64 {
        self.unweighted_avg.sample_variance()
    }

    /// Estimate the standard error of the *weighted* mean of the sequence.
    ///
    /// Returns 0 if the sum of weights is 0.
    ///
    /// This unbiased estimator assumes that the samples were independently
    /// drawn from the same population with constant variance.
    pub fn error(&self) -> f64 {
        // This uses the same estimate as WinCross, which should provide better
        // results than the ones used by SPSS or Mentor.
        //
        // See http://www.analyticalgroup.com/download/WEIGHTED_VARIANCE.pdf.
        if self.weight_sum == 0. {
            return 0.;
        }
        let inv_effective_len = self.weight_sum_sq / (self.weight_sum * self.weight_sum);
        (self.sample_variance() * inv_effective_len).sqrt()
    }

    /// Merge another sample into this one.
    ///
    ///
    /// ## Example
    ///
    /// ```
    /// use average::WeightedAverageWithError;
    ///
    /// let weighted_sequence: &[(f64, f64)] = &[
    ///     (1., 0.1), (2., 0.2), (3., 0.3), (4., 0.4), (5., 0.5),
    ///     (6., 0.6), (7., 0.7), (8., 0.8), (9., 0.9)];
    /// let (left, right) = weighted_sequence.split_at(3);
    /// let avg_total: WeightedAverageWithError = weighted_sequence.iter().map(|&x| x).collect();
    /// let mut avg_left: WeightedAverageWithError = left.iter().map(|&x| x).collect();
    /// let avg_right: WeightedAverageWithError = right.iter().map(|&x| x).collect();
    /// avg_left.merge(&avg_right);
    /// assert!((avg_total.weighted_mean() - avg_left.weighted_mean()).abs() < 1e-15);
    /// assert!((avg_total.error() - avg_left.error()).abs() < 1e-15);
    /// ```
    pub fn merge(&mut self, other: &WeightedAverageWithError) {
        let total_weight_sum = self.weight_sum + other.weight_sum;
        self.weighted_avg = (self.weight_sum * self.weighted_avg
                             + other.weight_sum * other.weighted_avg)
                            / total_weight_sum;
        self.weight_sum = total_weight_sum;
        self.weight_sum_sq += other.weight_sum_sq;

        self.unweighted_avg.merge(&other.unweighted_avg);
    }
}

impl core::default::Default for WeightedAverageWithError {
    fn default() -> WeightedAverageWithError {
        WeightedAverageWithError::new()
    }
}

impl core::iter::FromIterator<(f64, f64)> for WeightedAverageWithError {
    fn from_iter<T>(iter: T) -> WeightedAverageWithError
        where T: IntoIterator<Item=(f64, f64)>
    {
        let mut a = WeightedAverageWithError::new();
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
            let avg_total: WeightedAverageWithError = sequence.iter().map(|x| (*x, 1.)).collect();
            let mut avg_left: WeightedAverageWithError = left.iter().map(|x| (*x, 1.)).collect();
            let avg_right: WeightedAverageWithError = right.iter().map(|x| (*x, 1.)).collect();
            avg_left.merge(&avg_right);

            assert_eq!(avg_total.weight_sum, avg_left.weight_sum);
            assert_eq!(avg_total.weight_sum_sq, avg_left.weight_sum_sq);
            assert_eq!(avg_total.weighted_avg, avg_left.weighted_avg);

            assert_eq!(avg_total.unweighted_avg.len(), avg_left.unweighted_avg.len());
            assert_eq!(avg_total.unweighted_avg.mean(), avg_left.unweighted_avg.mean());
            assert_eq!(avg_total.unweighted_avg.sample_variance(),
                       avg_left.unweighted_avg.sample_variance());
        }
    }

    #[test]
    fn merge_weighted() {
        let sequence: &[(f64, f64)] = &[
            (1., 0.1), (2., 0.2), (3., 0.3), (4., 0.4), (5., 0.5),
            (6., 0.6), (7., 0.7), (8., 0.8), (9., 0.)];
        for mid in 0..sequence.len() {
            let (left, right) = sequence.split_at(mid);
            let avg_total: WeightedAverageWithError = sequence.iter().map(|&(x, w)| (x, w)).collect();
            let mut avg_left: WeightedAverageWithError = left.iter().map(|&(x, w)| (x, w)).collect();
            let avg_right: WeightedAverageWithError = right.iter().map(|&(x, w)| (x, w)).collect();
            avg_left.merge(&avg_right);
            assert_eq!(avg_total.unweighted_avg.len(), avg_left.unweighted_avg.len());
            assert_almost_eq!(avg_total.weight_sum, avg_left.weight_sum, 1e-15);
            assert_eq!(avg_total.weight_sum_sq, avg_left.weight_sum_sq);
            assert_almost_eq!(avg_total.weighted_avg, avg_left.weighted_avg, 1e-15);
            assert_almost_eq!(avg_total.unweighted_avg.mean(),
                              avg_left.unweighted_avg.mean(), 1e-15);
            assert_almost_eq!(avg_total.unweighted_avg.sample_variance(),
                              avg_left.unweighted_avg.sample_variance(), 1e-14);
        }
    }
}
