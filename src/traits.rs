/// Estimate a statistic of a sequence of numbers ("population").
pub trait Estimate {
    /// Add an observation sampled from the population.
    fn add(&mut self, x: f64);

    /// Estimate the statistic of the population.
    fn estimate(&self) -> f64;
}

/// Merge another sample into this one.
pub trait Merge {
    fn merge(&mut self, other: &Self);
}
