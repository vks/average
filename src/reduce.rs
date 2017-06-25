use super::{Estimate, Merge};

/// Estimate the reduction of a sequence of numbers ("population").
///
/// The reduction is a given function `Fn(f64, f64) -> f64`.
///
/// Everything is calculated iteratively using constant memory, so the sequence
/// of numbers can be an iterator.
#[derive(Debug, Clone)]
pub struct Reduce<F> {
    x: f64,
    reduce: F,
}

impl<F> Reduce<F> {
    /// Create a new reduction estimator given an initial value and a reduction.
    #[inline]
    pub fn from_value_and_fn(x: f64, f: F) -> Reduce<F> {
        Reduce { x: x, reduce: f }
    }

    /// Estimate the reduction of the population.
    #[inline]
    pub fn reduction(&self) -> f64 {
        self.x
    }

}

impl<F> Estimate for Reduce<F>
    where F: Fn(f64, f64) -> f64,
{
    #[inline]
    fn add(&mut self, x: f64) {
        self.x = (self.reduce)(self.x, x);
    }

    #[inline]
    fn estimate(&self) -> f64 {
        self.reduction()
    }
}

impl<F> Merge for Reduce<F>
    where F: Fn(f64, f64) -> f64,
{
    /// Merge another sample into this one.
    #[inline]
    fn merge(&mut self, other: &Reduce<F>) {
        self.add(other.x);
    }
}
