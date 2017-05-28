use conv::{ApproxFrom, ConvAsUtil, ValueFrom};
use quickersort::sort_floats;

/// Estimate the p-quantile of a sequence of numbers ("population").
#[derive(Debug, Clone)]
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
        assert!(0. <= p && p <= 1.);
        Quantile {
            q: [0.; 5],
            n: [1, 2, 3, 4, 0],
            m: [1., 1. + 2.*p, 1. + 4.*p, 3. + 2.*p, 5.],
            dm: [0., p/2., p, (1. + p)/2., 1.],
        }
    }

    /// Add an observation sampled from the population.
    #[inline]
    pub fn add(&mut self, x: f64) {
        // n[4] is the sample size.
        if self.n[4] < 5 {
            self.q[usize::value_from(self.n[4]).unwrap()] = x;  // n[4] < 5
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
            let d: f64 = self.m[i] - f64::approx_from(self.n[i]).unwrap();
            if d >= 1. && self.n[i + 1] - self.n[i] > 1 ||
               d <= -1. && self.n[i - 1] - self.n[i] < -1 {
                let d = d.signum();
                let q_new = self.parabolic(i, d);
                if self.q[i - 1] < q_new && q_new < self.q[i + 1] {
                    self.q[i] = q_new;
                } else {
                    self.q[i] = self.linear(i, d);
                }
                self.n[i] += d.approx().unwrap();  // d == +-1
            }
        }
    }

    /// Parabolic prediction for marker height.
    #[inline]
    fn parabolic(&self, i: usize, d: f64) -> f64 {
        debug_assert_eq!(d.abs(), 1.);
        let s: i64 = d.approx().unwrap();
        self.q[i] + d / f64::approx_from(self.n[i + 1] - self.n[i - 1]).unwrap()
            * (f64::approx_from(self.n[i] - self.n[i - 1] + s).unwrap()
               * (self.q[i + 1] - self.q[i])
               / f64::approx_from(self.n[i + 1] - self.n[i]).unwrap()
               + f64::approx_from(self.n[i + 1] - self.n[i] - s).unwrap()
               * (self.q[i] - self.q[i - 1])
               / f64::approx_from(self.n[i] - self.n[i - 1]).unwrap())
    }

    /// Linear prediction for marker height.
    #[inline]
    fn linear(&self, i: usize, d: f64) -> f64 {
        debug_assert_eq!(d.abs(), 1.);
        let s: usize = d.approx().unwrap();
        self.q[i] + d * (self.q[i + s] - self.q[i])
            / f64::approx_from(self.n[i + s] - self.n[i]).unwrap()
    }

    /// Estimate the p-quantile of the population.
    #[inline]
    pub fn quantile(&self) -> f64 {
        self.q[2]
    }

    /// Return the sample size.
    #[inline]
    pub fn len(&self) -> u64 {
        u64::value_from(self.n[4]).unwrap()
        //^ Shouldn't fail on any known platform.
    }
}

#[test]
fn reference() {
    let observations = [
    0.02, 0.5, 0.74, 3.39, 0.83,
    22.37, 10.15, 15.43, 38.62, 15.92,
    34.60, 10.28, 1.47, 0.40, 0.05,
    11.39, 0.27, 0.42, 0.09, 11.37,
    ];
    let mut q = Quantile::new(0.5);
    for &o in observations.iter() {
        q.add(o);
    }
    assert_eq!(q.n, [1, 6, 10, 16, 20]);
    assert_eq!(q.m, [1., 5.75, 10.50, 15.25, 20.0]);
    assert_eq!(q.len(), 20);
    assert_eq!(q.quantile(), 4.2462394088036435);
}
