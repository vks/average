use core;

/// Estimate the minimum of a sequence of numbers ("population").
///
/// Everything is calculated iteratively using constant memory, so the sequence
/// of numbers can be an iterator.
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
    x: f64,
}

fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b { a } else { b }
}

impl Min {
    /// Create a new minimum estimator.
    pub fn new() -> Min {
        Min { x: ::core::f64::INFINITY }
    }

    /// Create a new minium estimator from a given value.
    pub fn from_value(x: f64) -> Min {
        Min { x: x }
    }

    /// Add an element sampled from the population.
    pub fn add(&mut self, x: f64) {
        self.x = min(self.x, x);
    }

    /// Estimate the minium of the population.
    pub fn min(&self) -> f64 {
        self.x
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
    pub fn merge(&mut self, other: &Min) {
        self.add(other.x);
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
