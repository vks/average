use core;

/// Represent the weighted arithmetic mean and the weighted variance of a
/// sequence of numbers.
#[derive(Debug, Clone)]
pub struct WeightedAverage {
    /// Sum of the weights.
    weight_sum: f64,
    /// Sum of the squares of the weights.
    weight_sum_sq: f64,
    /// Average value.
    avg: f64,
    /// Intermediate sum of squares for calculating the variance.
    v: f64,
}

impl WeightedAverage {
    /// Create a new weighted average.
    pub fn new() -> WeightedAverage {
        WeightedAverage { weight_sum: 0., weight_sum_sq: 0., avg: 0., v: 0. }
    }

    /// Add a sample to the weighted sequence of which the average is calculated.
    pub fn add(&mut self, sample: f64, weight: f64) {
        // This algorithm was suggested by West in 1979.
        //
        // See
        // https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance.
        self.weight_sum += weight;
        self.weight_sum_sq += weight*weight;
        let prev_avg = self.avg;
        self.avg = prev_avg + (weight / self.weight_sum) * (sample - prev_avg);
        self.v += weight * (sample - prev_avg) * (sample - self.avg);
    }

    /// Determine whether the sequence is empty.
    pub fn is_empty(&self) -> bool {
        self.weight_sum_sq == 0.
    }

    /// Return the sum of the weights.
    pub fn sum_weights(&self) -> f64 {
        self.weight_sum
    }

    /// Return the sum of the squared weights.
    pub fn sum_weights_sq(&self) -> f64 {
        self.weight_sum_sq
    }

    /// Return the weighted mean of the sequence.
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
    pub fn sample_variance(&self) -> f64 {
        if self.is_empty() {
            0.
        } else {
            self.v / (self.weight_sum - 1.0)
        }
    }

    /// Calculate the reliability variance of the weighted sequence.
    ///
    /// This assumes weights represent *reliability*.
    pub fn reliability_variance(&self) -> f64 {
        if self.is_empty() {
            0.
        } else {
            self.v / (self.weight_sum - self.weight_sum_sq / self.weight_sum)
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

    use core::iter::Iterator;

    #[test]
    fn trivial() {
        let mut a = WeightedAverage::new();
        assert_eq!(a.sum_weights(), 0.);
        assert_eq!(a.sum_weights_sq(), 0.);
        a.add(1.0, 1.0);
        assert_eq!(a.mean(), 1.0);
        assert_eq!(a.sum_weights(), 1.0);
        assert_eq!(a.sum_weights_sq(), 1.0);
        assert_eq!(a.population_variance(), 0.0);
        //assert_eq!(a.error(), 0.0);
    }

    #[test]
    fn simple() {
        let a: WeightedAverage = (1..6).map(|x| (f64::from(x), 1.0)).collect();
        assert_eq!(a.mean(), 3.0);
        assert_eq!(a.sum_weights(), 5.0);
        assert_eq!(a.sample_variance(), 2.5);
        //assert_almost_eq!(a.error(), f64::sqrt(0.5), 1e-16);
    }
}
