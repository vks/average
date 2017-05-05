#![no_std]
#![feature(test)]

extern crate conv;
#[cfg(test)] extern crate rand;
#[cfg(test)] #[macro_use] extern crate std;
#[cfg(test)] extern crate stats;
#[cfg(test)] extern crate test;

use conv::ApproxFrom;

/// Represent an average value of a sequence of numbers.
///
/// The average is calculated iteratively, so the sequence of numbers can be an
/// iterator.
///
/// ```
/// use average::Average;
///
/// let a: Average = (1..6).map(|x| x as f64).collect();
/// assert_eq!(a.avg(), 3.0);
/// assert_eq!(a.var(), 2.5);
/// ```
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

impl core::iter::FromIterator<f64> for Average {
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

    macro_rules! assert_almost_eq {
        ($a:expr, $b:expr, $prec:expr) => (
            if ($a - $b).abs() > $prec {
                panic!(format!("assertion failed: `abs(left - right) < {:e}`, (left: `{}`, right: `{}`)", $prec, $a, $b));
            }
        );
    }

#[cfg(test)]
mod tests {
    use super::*;

    use std::vec::Vec;

    use ::conv::ConvAsUtil;

    #[test]
    fn average_trivial() {
        let mut a = Average::new();
        assert_eq!(a.len(), 0);
        a.add(1.0);
        assert_eq!(a.avg(), 1.0);
        assert_eq!(a.len(), 1);
        assert_eq!(a.var(), 0.0);
        assert_eq!(a.err(), 0.0);
    }

    #[test]
    fn average_simple() {
        let a: Average = (1..6).map(|x| x.approx().unwrap()).collect();
        assert_eq!(a.avg(), 3.0);
        assert_eq!(a.len(), 5);
        assert_eq!(a.var(), 2.5);
        assert_almost_eq!(a.err(), f64::sqrt(0.5), 1e-16);
    }

    #[test]
    fn average_normal_distribution() {
        use rand::distributions::{Normal, IndependentSample};
        let normal = Normal::new(2.0, 3.0);
        let mut a = Average::new();
        for _ in 0..1000000 {
            a.add(normal.ind_sample(&mut ::rand::thread_rng()));
        }
        assert_almost_eq!(a.avg(), 2.0, 1e-2);
        assert_almost_eq!(a.var().sqrt(), 3.0, 1e-2);
    }

    fn initialize_vec() -> Vec<f64> {
        use rand::distributions::{Normal, IndependentSample};
        use rand::{XorShiftRng, SeedableRng};
        let normal = Normal::new(2.0, 3.0);
        let n = 1_000_000;
        let mut values = Vec::with_capacity(n);
        let mut rng = XorShiftRng::from_seed([1, 2, 3, 4]);
        for _ in 0..n {
            values.push(normal.ind_sample(&mut rng));
        }
        values
    }

    #[bench]
    fn bench_average(b: &mut test::Bencher) {
        let values = initialize_vec();
        b.iter(|| {
            let a: Average = values.iter().map(|x| *x).collect();
            a
        });
    }

    #[bench]
    fn bench_stats(b: &mut test::Bencher) {
        let values = initialize_vec();
        b.iter(|| {
            let a: stats::OnlineStats = values.iter().map(|x| *x).collect();
            a
        });
    }
}
