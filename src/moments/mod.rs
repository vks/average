use core;

use conv::ApproxFrom;
use num_traits::pow;
use num_integer::{IterBinomial, binomial};

use super::{Estimate, Merge};

include!("mean.rs");
include!("variance.rs");
include!("skewness.rs");
include!("kurtosis.rs");


/// Alias for `Variance`.
pub type MeanWithError = Variance;

const MAX_P: usize = 4;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// https://doi.org/10.1007/s00180-015-0637-z.
pub struct Moments {
    /// Number of samples.
    ///
    /// Technically, this is the same as m_0, but we want this to be an integer
    /// to avoid numerical issues, so we store it separately.
    n: u64,
    /// Average.
    avg: f64,
    /// Moments times `n`.
    ///
    /// Starts with m_2. m_0 is the same as `n` and m_1 is 0 by definition.
    m: [f64; MAX_P - 1],
}

impl Moments {
    #[inline]
    pub fn new() -> Moments {
        Moments {
            n: 0,
            avg: 0.,
            m: [0.; MAX_P - 1],
        }
    }

    #[inline]
    pub fn len(&self) -> u64 {
        self.n
    }

    #[inline]
    pub fn mean(&self) -> f64 {
        self.avg
    }

    #[inline]
    pub fn central_moment(&self, p: usize) -> f64 {
        let n = f64::approx_from(self.n).unwrap();
        match p {
            0 => 1.,
            1 => 0.,
            _ => self.m[p - 2] / n
        }
    }

    #[inline]
    pub fn standardized_moment(&self, p: usize) -> f64 {
        let variance = self.central_moment(2);
        assert_ne!(variance, 0.);
        let n = f64::approx_from(self.n).unwrap();
        match p {
            0 => n,
            1 => 0.,
            2 => 1.,
            _ => self.central_moment(p) / pow(variance.sqrt(), p),
        }
    }

    #[inline]
    pub fn sample_variance(&self) -> f64 {
        if self.n < 2 {
            return 0.;
        }
        self.m[0] / f64::approx_from(self.n - 1).unwrap()
    }

    #[inline]
    pub fn sample_skewness(&self) -> f64 {
        if self.n < 2 {
            return 0.;
        }
        let n = f64::approx_from(self.n).unwrap();
        if self.n < 3 {
            // Method of moments
            return self.central_moment(3) /
                (n * (self.central_moment(2) / (n - 1.)).powf(1.5))
        }
        // Adjusted Fisher-Pearson standardized moment coefficient
        (n * (n - 1.)).sqrt() / (n * (n - 2.)) *
            self.central_moment(3) / (self.central_moment(2) / n).powf(1.5)
    }

    #[inline]
    pub fn sample_kurtosis(&self) -> f64 {
        if self.n < 4 {
            return 0.;
        }
        let n = f64::approx_from(self.n).unwrap();
        (n + 1.) * n * self.central_moment(4) /
            ((n - 1.) * (n - 2.) * (n - 3.) * pow(self.central_moment(2), 2)) -
            3. * pow(n - 1., 2) / ((n - 2.) * (n - 3.))
    }

    #[inline]
    pub fn add(&mut self, x: f64) {
        self.n += 1;
        let delta = x - self.avg;
        let n = f64::approx_from(self.n).unwrap();
        self.avg += delta / n;

        let mut coeff_delta = delta;
        let over_n = 1. / n;
        let mut term1 = (n - 1.) * (-over_n);
        let factor1 = -over_n;
        let mut term2 = (n - 1.) * over_n;
        let factor2 = (n - 1.) * over_n;

        let factor_coeff = -delta * over_n;

        let prev_m = self.m;
        for p in 2..=MAX_P {
            term1 *= factor1;
            term2 *= factor2;
            coeff_delta *= delta;
            self.m[p - 2] += (term1 + term2) * coeff_delta;

            let mut coeff = 1.;
            let mut binom = IterBinomial::new(p as u64);
            binom.next().unwrap();  // Skip k = 0.
            for k in 1..=(p - 2) {
                coeff *= factor_coeff;
                self.m[p - 2] += f64::approx_from(binom.next().unwrap()).unwrap() *
                    prev_m[p - 2 - k] * coeff;
            }
        }
    }
}

impl Merge for Moments {
    #[inline]
    fn merge(&mut self, other: &Moments) {
        let mut result = Moments::new();
        result.n = self.n + other.n;
        if result.n == 0 {
            return;
        }
        for i in 0..result.m.len() {
            result.m[i] = self.m[i] + other.m[i];
        }
        let n = f64::approx_from(result.n).unwrap();
        let n_a = f64::approx_from(self.n).unwrap();
        let n_b = f64::approx_from(other.n).unwrap();
        let n_a_over_n = n_a / n;
        let n_b_over_n = n_b / n;
        let delta = other.avg - self.avg;
        result.avg = self.avg + n_b_over_n * delta;

        let factor_a = -n_b_over_n * delta;
        let factor_b = n_a_over_n * delta;
        let mut term_a = n_a * factor_a;
        let mut term_b = n_b * factor_b;
        for p in 2..=MAX_P {
            term_a *= factor_a;
            term_b *= factor_b;
            result.m[p - 2] += term_a + term_b;

            let mut coeff_a = 1.;
            let mut coeff_b = 1.;
            let mut coeff_delta = 1.;
            for k in 1..=(p - 2) {
                coeff_a *= -n_b_over_n;
                coeff_b *= n_a_over_n;
                coeff_delta *= delta;
                result.m[p - 2] += f64::approx_from(binomial(p, k)).unwrap() *
                    coeff_delta *
                    (self.m[p - 2 - k] * coeff_a + other.m[p - 2 - k] * coeff_b);
                // TODO: use IterBinomial
            }
        }

        *self = result;
    }
}

impl core::default::Default for Moments {
    fn default() -> Moments {
        Moments::new()
    }
}

impl_from_iterator!(Moments);
