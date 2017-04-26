extern crate conv;

use conv::ApproxFrom;

/// Represent and average value of a sequence of numbers.
///
/// The average is calculated iteratively, so the sequence of numbers can be an
/// iterator.
#[derive(Debug, Clone)]
pub struct Average {
    avg: f64,
    n: u64,
    v: f64,
}

impl Average {
    /// Create a new average.
    pub fn new() -> Average {
        Average { avg: 0., n: 0, v: 0. }
    }

    /// Add a number to the sequence of which the average is calculated.
    pub fn add(&mut self, x: f64) {
        self.n += 1;
        let prev_avg = self.avg;
        self.avg += (x - prev_avg) / f64::approx_from(self.n).unwrap();
        self.v += (x - prev_avg) * (x - self.avg);
    }

    /// Return the average of the sequence.
    pub fn avg(&self) -> f64 {
        self.avg
    }

    /// Return the number of elements in the sequence.
    pub fn len(&self) -> u64 {
        self.n
    }

    /// Calculate the unbiased sample variance of the sequence.
    pub fn var(&self) -> f64 {
        if self.n < 2 {
            return 0.;
        }
        self.v / f64::approx_from(self.n - 1).unwrap()
    }

    /// Calculate the standard error of the average of the sequence.
    pub fn err(&self) -> f64 {
        if self.n == 0 {
            return 0.;
        }
        (self.var() / f64::approx_from(self.n).unwrap()).sqrt()
    }
}

impl std::iter::FromIterator<f64> for Average {
    fn from_iter<T>(iter: T) -> Average
        where T: IntoIterator<Item=f64>
    {
        let mut a = Average::new();
        for i in iter {
            a.add(i);
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use ::conv::ConvAsUtil;

    #[test]
    fn average() {
        let a: Average = (1..6).map(|x| x.approx().unwrap()).collect();
        assert_eq!(a.avg(), 3.0);
        assert_eq!(a.len(), 5);
        assert_eq!(a.var(), 2.5);
        assert!((a.err() - f64::sqrt(0.5)).abs() < 1e-16);
    }
}
