use core::cmp::min;

use easy_cast::{Conv, ConvFloat};
use float_ord::sort as sort_floats;
use num_traits::{Float, ToPrimitive};
#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

use super::Estimate;

/// Estimate the p-quantile of a sequence of numbers ("population").
///
/// The [P² algorithm][1] is employed. It uses constant space but the relative
/// error of the quantile estimate is not bounded by a function of the number of
/// samples. For algorithms that use growing space with bounded error, see the
/// [`quantiles`][2] crate.
///
/// It is recommended to use a different algorithm for discrete distributions
/// and a small number of samples, or for quantiles close to a singularity in
/// the distribution.
///
/// [1]: http://www.cs.wustl.edu/~jain/papers/ftp/psqr.pdf
/// [2]: https://crates.io/crates/quantiles
#[derive(Debug, Clone)]
#[cfg_attr(doc_cfg, doc(cfg(any(feature = "std", feature = "libm"))))]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Quantile {
    /// Marker heights.
    q: [f64; 5],
    /// Marker positions.
    n: [i64; 5],
    /// Desired marker positions.
    m: [f64; 5],
    /// Increment in desired marker positions.
    dm: [f64; 5],
}

impl Quantile {
    /// Create a new p-quantile estimator.
    ///
    /// Panics if `p` is not between 0 and 1.
    #[inline]
    pub fn new(p: f64) -> Quantile {
        assert!((0. ..=1.).contains(&p));
        Quantile {
            q: [0.; 5],
            n: [1, 2, 3, 4, 0],
            m: [1., 1. + 2. * p, 1. + 4. * p, 3. + 2. * p, 5.],
            dm: [0., p / 2., p, (1. + p) / 2., 1.],
        }
    }

    /// Return the value of `p` for this p-quantile.
    #[inline]
    pub fn p(&self) -> f64 {
        self.dm[2]
    }

    /// Parabolic prediction for marker height.
    #[inline]
    fn parabolic(&self, i: usize, d: f64) -> f64 {
        debug_assert_eq!(d.abs(), 1.);
        let s = i64::conv_nearest(d);
        self.q[i]
            + d / (self.n[i + 1] - self.n[i - 1]).to_f64().unwrap()
                * ((self.n[i] - self.n[i - 1] + s).to_f64().unwrap() * (self.q[i + 1] - self.q[i])
                    / (self.n[i + 1] - self.n[i]).to_f64().unwrap()
                    + (self.n[i + 1] - self.n[i] - s).to_f64().unwrap()
                        * (self.q[i] - self.q[i - 1])
                        / (self.n[i] - self.n[i - 1]).to_f64().unwrap())
    }

    /// Linear prediction for marker height.
    #[inline]
    fn linear(&self, i: usize, d: f64) -> f64 {
        debug_assert_eq!(d.abs(), 1.);
        let sum = if d < 0. { i - 1 } else { i + 1 };
        self.q[i] + d * (self.q[sum] - self.q[i]) / (self.n[sum] - self.n[i]).to_f64().unwrap()
    }

    /// Estimate the p-quantile of the population.
    ///
    /// Returns NaN for an empty sample.
    #[inline]
    pub fn quantile(&self) -> f64 {
        if self.len() >= 5 {
            return self.q[2];
        }

        // Estimate quantile by sorting the sample.
        if self.is_empty() {
            return f64::NAN;
        }
        let mut heights: [f64; 4] = [self.q[0], self.q[1], self.q[2], self.q[3]];
        let len = usize::conv(self.len());
        debug_assert!(len < 5);
        sort_floats(&mut heights[..len]);
        let desired_index = f64::conv(len) * self.p() - 1.;
        let mut index = desired_index.ceil();
        if desired_index == index && index >= 0. {
            let index = usize::conv_nearest(index);
            debug_assert!(index < 5);
            if index < len - 1 {
                // `q[index]` and `q[index + 1]` are equally valid estimates,
                // by convention we take their average.
                return 0.5 * self.q[index] + 0.5 * self.q[index + 1];
            }
        }
        index = index.max(0.);
        let mut index = usize::conv_nearest(index);
        debug_assert!(index < 5);
        index = min(index, len - 1);
        self.q[index]
    }

    /// Return the sample size.
    #[inline]
    pub fn len(&self) -> u64 {
        debug_assert!(self.n[4] >= 0);
        u64::conv(self.n[4])
    }

    /// Determine whether the sample is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl core::default::Default for Quantile {
    /// Create a new median estimator.
    fn default() -> Quantile {
        Quantile::new(0.5)
    }
}

impl Estimate for Quantile {
    #[inline]
    fn add(&mut self, x: f64) {
        // n[4] is the sample size.
        if self.n[4] < 5 {
            self.q[usize::conv(self.n[4])] = x;
            self.n[4] += 1;
            if self.n[4] == 5 {
                sort_floats(&mut self.q);
            }
            return;
        }

        // Find cell k.
        let mut k: usize;
        if x < self.q[0] {
            self.q[0] = x;
            k = 0;
        } else {
            k = 4;
            for i in 1..5 {
                if x < self.q[i] {
                    k = i;
                    break;
                }
            }
            if self.q[4] < x {
                self.q[4] = x;
            }
        };

        // Increment all positions greater than k.
        for i in k..5 {
            self.n[i] += 1;
        }
        for i in 0..5 {
            self.m[i] += self.dm[i];
        }

        // Adjust height of markers.
        for i in 1..4 {
            let d = self.m[i] - self.n[i].to_f64().unwrap();
            if d >= 1. && self.n[i + 1] - self.n[i] > 1
                || d <= -1. && self.n[i - 1] - self.n[i] < -1
            {
                let d = Float::signum(d);
                let q_new = self.parabolic(i, d);
                if self.q[i - 1] < q_new && q_new < self.q[i + 1] {
                    self.q[i] = q_new;
                } else {
                    self.q[i] = self.linear(i, d);
                }
                let delta = i64::conv_nearest(d);
                debug_assert_eq!(delta.abs(), 1);
                self.n[i] += delta;
            }
        }
    }

    fn estimate(&self) -> f64 {
        self.quantile()
    }
}

#[test]
fn reference() {
    let observations = [
        0.02, 0.5, 0.74, 3.39, 0.83, 22.37, 10.15, 15.43, 38.62, 15.92, 34.60, 10.28, 1.47, 0.40,
        0.05, 11.39, 0.27, 0.42, 0.09, 11.37,
    ];
    let mut q = Quantile::new(0.5);
    for &o in observations.iter() {
        q.add(o);
    }
    assert_eq!(q.n, [1, 6, 10, 16, 20]);
    assert_eq!(q.m, [1., 5.75, 10.50, 15.25, 20.0]);
    assert_eq!(q.len(), 20);
    assert_almost_eq!(q.quantile(), 4.2462394088036435, 2e-15);
}
