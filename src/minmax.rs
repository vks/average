#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

use super::{Estimate, Merge};

/// Calculate the minimum of `a` and `b`.
fn min(a: f64, b: f64) -> f64 {
    a.min(b)
}

/// Calculate the maximum of `a` and `b`.
fn max(a: f64, b: f64) -> f64 {
    a.max(b)
}

/// Estimate the minimum of a sequence of numbers ("population").
///
///
/// ## Example
///
/// ```
/// use average::Min;
///
/// let a: Min = (1..6).map(f64::from).collect();
/// println!("The minimum is {}.", a.min());
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Min {
    x: f64,
}

impl Min {
    /// Create a new minimum estimator from a given value.
    #[inline]
    pub fn from_value(x: f64) -> Min {
        Min { x }
    }

    /// Create a new minimum estimator.
    #[inline]
    pub fn new() -> Min {
        Min::from_value(f64::INFINITY)
    }

    /// Estimate the minimum of the population.
    /// 
    /// Returns `f64::INFINITY` for an empty sample.
    #[inline]
    pub fn min(&self) -> f64 {
        self.x
    }
}

impl core::default::Default for Min {
    fn default() -> Min {
        Min::new()
    }
}

impl_from_iterator!(Min);
impl_from_par_iterator!(Min);
impl_extend!(Min);

impl Estimate for Min {
    #[inline]
    fn add(&mut self, x: f64) {
        self.x = min(self.x, x);
    }

    #[inline]
    fn estimate(&self) -> f64 {
        self.min()
    }
}

impl Merge for Min {
    /// Merge another sample into this one.
    ///
    ///
    /// ## Example
    ///
    /// ```
    /// use average::{Min, Merge};
    ///
    /// let sequence: &[f64] = &[1., 2., 3., 4., 5., 6., 7., 8., 9.];
    /// let (left, right) = sequence.split_at(3);
    /// let min_total: Min = sequence.iter().collect();
    /// let mut min_left: Min = left.iter().collect();
    /// let min_right: Min = right.iter().collect();
    /// min_left.merge(&min_right);
    /// assert_eq!(min_total.min(), min_left.min());
    /// ```
    #[inline]
    fn merge(&mut self, other: &Min) {
        self.add(other.x);
    }
}

/// Estimate the maximum of a sequence of numbers ("population").
///
///
/// ## Example
///
/// ```
/// use average::Max;
///
/// let a: Max = (1..6).map(f64::from).collect();
/// assert_eq!(a.max(), 5.);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Max {
    x: f64,
}

impl Max {
    /// Create a new maximum estimator from a given value.
    #[inline]
    pub fn from_value(x: f64) -> Max {
        Max { x }
    }

    /// Create a new maximum estimator.
    #[inline]
    pub fn new() -> Max {
        Max::from_value(f64::NEG_INFINITY)
    }

    /// Estimate the maximum of the population.
    /// 
    /// Returns `f64::NEG_INFINITY` for an empty sample.
    #[inline]
    pub fn max(&self) -> f64 {
        self.x
    }
}

impl core::default::Default for Max {
    fn default() -> Max {
        Max::new()
    }
}

impl_from_iterator!(Max);
impl_from_par_iterator!(Max);

impl Estimate for Max {
    #[inline]
    fn add(&mut self, x: f64) {
        self.x = max(self.x, x);
    }

    #[inline]
    fn estimate(&self) -> f64 {
        self.max()
    }
}

impl Merge for Max {
    /// Merge another sample into this one.
    ///
    ///
    /// ## Example
    ///
    /// ```
    /// use average::{Max, Merge};
    ///
    /// let sequence: &[f64] = &[1., 2., 3., 4., 5., 6., 7., 8., 9.];
    /// let (left, right) = sequence.split_at(3);
    /// let max_total: Max = sequence.iter().collect();
    /// let mut max_left: Max = left.iter().collect();
    /// let max_right: Max = right.iter().collect();
    /// max_left.merge(&max_right);
    /// assert_eq!(max_total.max(), max_left.max());
    /// ```
    #[inline]
    fn merge(&mut self, other: &Max) {
        self.add(other.x);
    }
}
