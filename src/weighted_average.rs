use core;

/// Estimate the weighted arithmetic mean and the weighted variance of a
/// sequence of numbers ("population").
///
/// This can be used to estimate the standard error of the weighted mean.
///
///
/// ## Example
///
/// ```
/// use average::WeightedAverage;
///
/// let a: WeightedAverage = (1..6).zip(1..6)
///     .map(|(x, w)| (f64::from(x), f64::from(w))).collect();
/// println!("The weighted average is {} ± {}.", a.mean(), a.error());
/// ```
#[derive(Debug, Clone)]
pub struct WeightedAverage {
    /// Sum of the weights.
    weight_sum: f64,
    /// Average value.
    avg: f64,
    /// Intermediate sum of squares for calculating the variance.
    v: f64,
}

impl WeightedAverage {
    /// Create a new weighted average estimator.
    pub fn new() -> WeightedAverage {
        WeightedAverage { weight_sum: 0., avg: 0., v: 0. }
    }

    /// Add a weighted element sampled from the population.
    pub fn add(&mut self, sample: f64, weight: f64) {
        // This algorithm was suggested by West in 1979.
        //
        // See
        // https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance
        // and
        // http://people.ds.cam.ac.uk/fanf2/hermes/doc/antiforgery/stats.pdf.
        self.weight_sum += weight;
        let prev_avg = self.avg;
        self.avg = prev_avg + (weight / self.weight_sum) * (sample - prev_avg);
        self.v += weight * (sample - prev_avg) * (sample - self.avg);
    }

    /// Determine whether the samples are empty.
    pub fn is_empty(&self) -> bool {
        self.weight_sum == 0. && self.v == 0. && self.avg == 0.
    }

    /// Return the sum of the weights.
    pub fn sum_weights(&self) -> f64 {
        self.weight_sum
    }

    /// Estimate the weighted mean of the population.
    pub fn mean(&self) -> f64 {
        self.avg
    }

    /// Calculate the weighted population variance of the sample.
    ///
    /// This is a biased estimator of the weighted variance of the population.
    pub fn population_variance(&self) -> f64 {
        if self.is_empty() {
            0.
        } else {
            self.v / self.weight_sum
        }
    }

    /// Calculate the weighted sample variance.
    ///
    /// This is an unbiased estimator of the weighted variance of the population.
    ///
    /// Note that this will return 0 if the sum of the weights is <= 1.
    pub fn sample_variance(&self) -> f64 {
        if self.weight_sum <= 1. {
            0.
        } else {
            self.v / (self.weight_sum - 1.0)
        }
    }

    /// Estimate the standard error of the weighted mean of the population.
    ///
    /// Note that this will return 0 if the sum of the weights is 0.
    /// For this estimator, the sum of weights should be larger than 1.
    ///
    /// This biased estimator uses the weighted variance and the sum of weights.
    /// It considers the weights as (noninteger) counts of how often the sample
    /// has been observed, applying the standard formulas to calculate mean,
    /// variance and sample size across all "repeats".
    pub fn error(&self) -> f64 {
        // This uses the same estimate as SPSS.
        //
        // See http://www.analyticalgroup.com/download/WEIGHTED_MEAN.pdf.
        if self.weight_sum == 0. {
            return 0.;
        }
        let variance = if self.weight_sum <= 1. {
            self.population_variance()
        } else {
            self.sample_variance()
        };
        (variance / self.weight_sum).sqrt()
    }

    /// Merge another sample into this one.
    ///
    ///
    /// ## Example
    ///
    /// ```
    /// use average::WeightedAverage;
    ///
    /// let weighted_sequence: &[(f64, f64)] = &[
    ///     (1., 0.1), (2., 0.2), (3., 0.3), (4., 0.4), (5., 0.5),
    ///     (6., 0.6), (7., 0.7), (8., 0.8), (9., 0.9)];
    /// let (left, right) = weighted_sequence.split_at(3);
    /// let avg_total: WeightedAverage = weighted_sequence.iter().map(|&x| x).collect();
    /// let mut avg_left: WeightedAverage = left.iter().map(|&x| x).collect();
    /// let avg_right: WeightedAverage = right.iter().map(|&x| x).collect();
    /// avg_left.merge(&avg_right);
    /// assert!((avg_total.mean() - avg_left.mean()).abs() < 1e-15);
    /// assert!((avg_total.error() - avg_left.error()).abs() < 1e-15);
    /// ```
    pub fn merge(&mut self, other: &WeightedAverage) {
        // This is similar to the algorithm proposed by Chan et al. in 1979.
        //
        // See https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance.
        let delta = other.avg - self.avg;
        let total_weight_sum = self.weight_sum + other.weight_sum;
        self.avg = (self.weight_sum * self.avg + other.weight_sum * other.avg)
                   / (self.weight_sum + other.weight_sum);
        self.v += other.v + delta*delta * self.weight_sum * other.weight_sum
                            / total_weight_sum;
        self.weight_sum = total_weight_sum;
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
            assert_eq!(avg_total.weight_sum, avg_left.weight_sum);
            assert_eq!(avg_total.avg, avg_left.avg);
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
            assert_almost_eq!(avg_total.weight_sum, avg_left.weight_sum, 1e-15);
            assert_almost_eq!(avg_total.avg, avg_left.avg, 1e-15);
            assert_almost_eq!(avg_total.v, avg_left.v, 1e-14);
        }
    }
}
