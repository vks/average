use core;

/// Represent the weighted arithmetic mean and the weighted variance of a
/// sequence of numbers.
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
    /// Create a new weighted average.
    pub fn new() -> WeightedAverage {
        WeightedAverage { weight_sum: 0., avg: 0., v: 0. }
    }

    /// Add a sample to the weighted sequence of which the average is calculated.
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

    /// Determine whether the sequence is empty.
    pub fn is_empty(&self) -> bool {
        self.weight_sum == 0. && self.v == 0. && self.avg == 0.
    }

    /// Return the sum of the weights.
    pub fn sum_weights(&self) -> f64 {
        self.weight_sum
    }

    /// Estimate the weighted mean of the sequence.
    pub fn mean(&self) -> f64 {
        self.avg
    }

    /// Calculate the population variance of the weighted sequence.
    ///
    /// This assumes that the sequence consists of the entire population and the
    /// weights represent *frequency*.
    pub fn population_variance(&self) -> f64 {
        if self.is_empty() {
            0.
        } else {
            self.v / self.weight_sum
        }
    }

    /// Calculate the unbiased sample variance of the weighted sequence.
    ///
    /// This assumes that the sequence consists of samples of a larger
    /// population and the weights represent *frequency*.
    ///
    /// Note that this will return 0 if the sum of the weights is <= 1.
    pub fn sample_variance(&self) -> f64 {
        if self.weight_sum <= 1. {
            0.
        } else {
            self.v / (self.weight_sum - 1.0)
        }
    }

    /// Estimate the standard error of the weighted mean of the sequence.
    ///
    /// Note that this will return 0 if the sum of the weights is 0.
    /// For this estimator the sum of weights should be larger than 1.
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

    /// Merge the weighted average of another sequence into this one.
    ///
    /// ```
    /// use average::WeightedAverage;
    ///
    /// let weighted_sequence: &[(f64, f64)] = &[
    ///     (1., 0.1), (2., 0.2), (3., 0.3), (4., 0.4), (5., 0.5),
    ///     (6., 0.6), (7., 0.7), (8., 0.8), (9., 0.)];
    /// let (left, right) = weighted_sequence.split_at(3);
    /// let avg_total: WeightedAverage = weighted_sequence.iter().map(|&x| x).collect();
    /// let mut avg_left: WeightedAverage = left.iter().map(|&x| x).collect();
    /// let avg_right: WeightedAverage = right.iter().map(|&x| x).collect();
    /// avg_left.merge(&avg_right);
    /// assert!((avg_total.mean() - avg_left.mean()).abs() < 1e-15);
    /// assert!((avg_total.sample_variance() - avg_left.sample_variance()).abs() < 1e-15);
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

    use core::iter::Iterator;

    #[test]
    fn trivial() {
        let mut a = WeightedAverage::new();
        assert_eq!(a.sum_weights(), 0.);
        a.add(1.0, 1.0);
        assert_eq!(a.mean(), 1.0);
        assert_eq!(a.sum_weights(), 1.0);
        assert_eq!(a.population_variance(), 0.0);
        assert_eq!(a.error(), 0.0);
        a.add(1.0, 1.0);
        assert_eq!(a.mean(), 1.0);
        assert_eq!(a.sum_weights(), 2.0);
        assert_eq!(a.population_variance(), 0.0);
        assert_eq!(a.error(), 0.0);
    }

    #[test]
    fn simple() {
        let a: WeightedAverage = (1..6).map(|x| (f64::from(x), 1.0)).collect();
        assert_eq!(a.mean(), 3.0);
        assert_eq!(a.sum_weights(), 5.0);
        assert_eq!(a.sample_variance(), 2.5);
        assert_almost_eq!(a.error(), f64::sqrt(0.5), 1e-16);
    }

    #[test]
    fn reference() {
        // Example from http://www.analyticalgroup.com/download/WEIGHTED_MEAN.pdf.
        let values = &[5., 5., 4., 4., 3., 4., 3., 2., 2., 1.];
        let weights = &[1.23, 2.12, 1.23, 0.32, 1.53, 0.59, 0.94, 0.94, 0.84, 0.73];
        let a: WeightedAverage = values.iter().zip(weights.iter())
            .map(|(x, w)| (*x, *w)).collect();
        assert_almost_eq!(a.mean(), 3.53486, 1e-5);
        assert_almost_eq!(a.sample_variance(), 1.8210, 1e-4);
        assert_eq!(a.sum_weights(), 10.47);
        assert_almost_eq!(a.error(), f64::sqrt(0.1739), 1e-4);
    }

    #[test]
    fn error_corner_case() {
        let values = &[1., 2.];
        let weights = &[0.5, 0.5];
        let a: WeightedAverage = values.iter().zip(weights.iter())
            .map(|(x, w)| (*x, *w)).collect();
        assert_eq!(a.error(), 0.5);
    }

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
