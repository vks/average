use core;

use super::reduce::Reduce;

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
/// let a: Min = (1..6).map(Into::into).collect();
/// println!("The minimum is {}.", a.min());
/// ```
#[derive(Debug, Clone)]
pub struct Min {
    r: Reduce<fn(f64, f64) -> f64>,
}

impl Min {
    /// Create a new minium estimator from a given value.
    #[inline]
    pub fn from_value(x: f64) -> Min {
        Min {
            r: Reduce::from_value_and_fn(x, min),
        }
    }

    /// Create a new minimum estimator.
    #[inline]
    pub fn new() -> Min {
        Min::from_value(::core::f64::INFINITY)
    }

    /// Add an observation sampled from the population.
    #[inline]
    pub fn add(&mut self, x: f64) {
        self.r.add(x);
    }

    /// Estimate the minium of the population.
    #[inline]
    pub fn min(&self) -> f64 {
        self.r.reduction()
    }

    /// Merge another sample into this one.
    ///
    ///
    /// ## Example
    ///
    /// ```
    /// use average::Min;
    ///
    /// let sequence: &[f64] = &[1., 2., 3., 4., 5., 6., 7., 8., 9.];
    /// let (left, right) = sequence.split_at(3);
    /// let min_total: Min = sequence.iter().map(|x| *x).collect();
    /// let mut min_left: Min = left.iter().map(|x| *x).collect();
    /// let min_right: Min = right.iter().map(|x| *x).collect();
    /// min_left.merge(&min_right);
    /// assert_eq!(min_total.min(), min_left.min());
    /// ```
    #[inline]
    pub fn merge(&mut self, other: &Min) {
        self.r.merge(&other.r);
    }
}

impl core::iter::FromIterator<f64> for Min {
    fn from_iter<T>(iter: T) -> Min
        where T: IntoIterator<Item=f64>
    {
        let mut a = Min::new();
        for i in iter {
            a.add(i);
        }
        a
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
/// let a: Max = (1..6).map(Into::into).collect();
/// assert_eq!(a.max(), 5.);
/// ```
#[derive(Debug, Clone)]
pub struct Max {
    r: Reduce<fn(f64, f64) -> f64>,
}

impl Max {
    /// Create a new maxium estimator from a given value.
    #[inline]
    pub fn from_value(x: f64) -> Max {
        Max {
            r: Reduce::from_value_and_fn(x, max),
        }
    }

    /// Create a new maximum estimator.
    #[inline]
    pub fn new() -> Max {
        Max::from_value(::core::f64::NEG_INFINITY)
    }

    /// Add an observation sampled from the population.
    #[inline]
    pub fn add(&mut self, x: f64) {
        self.r.add(x);
    }

    /// Estimate the maxium of the population.
    #[inline]
    pub fn max(&self) -> f64 {
        self.r.reduction()
    }

    /// Merge another sample into this one.
    ///
    ///
    /// ## Example
    ///
    /// ```
    /// use average::Max;
    ///
    /// let sequence: &[f64] = &[1., 2., 3., 4., 5., 6., 7., 8., 9.];
    /// let (left, right) = sequence.split_at(3);
    /// let max_total: Max = sequence.iter().map(|x| *x).collect();
    /// let mut max_left: Max = left.iter().map(|x| *x).collect();
    /// let max_right: Max = right.iter().map(|x| *x).collect();
    /// max_left.merge(&max_right);
    /// assert_eq!(max_total.max(), max_left.max());
    /// ```
    #[inline]
    pub fn merge(&mut self, other: &Max) {
        self.r.merge(&other.r);
    }
}

impl core::iter::FromIterator<f64> for Max {
    fn from_iter<T>(iter: T) -> Max
        where T: IntoIterator<Item=f64>
    {
        let mut a = Max::new();
        for i in iter {
            a.add(i);
        }
        a
    }
}
