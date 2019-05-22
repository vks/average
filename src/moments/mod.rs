use core;

use conv::ApproxFrom;

use super::{Estimate, Merge};

include!("mean.rs");
include!("variance.rs");
include!("skewness.rs");
include!("kurtosis.rs");

/// Alias for `Variance`.
pub type MeanWithError = Variance;

/// Define an estimator of all moments up to a number given at compile time.
///
/// This uses a [general algorithm][paper] and is slightly less efficient than
/// the specialized implementations (such as [`Mean`], [`Variance`],
/// [`Skewness`] and [`Kurtosis`]), but it works for any number of moments >= 4.
///
/// (In practise, there is an upper limit due to integer overflow and possibly
/// numerical issues.)
///
/// [paper]: https://doi.org/10.1007/s00180-015-0637-z.
/// [`Mean`]: ./struct.Mean.html
/// [`Variance`]: ./struct.Variance.html
/// [`Skewness`]: ./struct.Skewness.html
/// [`Kurtosis`]: ./struct.Kurtosis.html
///
///
/// # Example
///
/// ```
/// # extern crate core;
/// # extern crate conv;
/// # extern crate num_traits;
/// #[cfg(feature = "serde1")]
/// extern crate serde;
/// #[cfg(feature = "serde1")]
/// #[macro_use] extern crate serde_derive;
/// # #[macro_use] extern crate average;
/// # fn main() {
/// define_moments!(Moments4, 4);
///
/// let mut a: Moments4 = (1..6).map(f64::from).collect();
/// assert_eq!(a.len(), 5);
/// assert_eq!(a.mean(), 3.0);
/// assert_eq!(a.central_moment(0), 1.0);
/// assert_eq!(a.central_moment(1), 0.0);
/// assert_eq!(a.central_moment(2), 2.0);
/// assert_eq!(a.standardized_moment(0), 5.0);
/// assert_eq!(a.standardized_moment(1), 0.0);
/// assert_eq!(a.standardized_moment(2), 1.0);
/// a.add(1.0);
/// // skewness
/// assert_almost_eq!(a.standardized_moment(3), 0.2795084971874741, 1e-15);
/// // kurtosis
/// assert_almost_eq!(a.standardized_moment(4), -1.365 + 3.0, 1e-14);
/// # }
/// ```
#[macro_export]
macro_rules! define_moments {
    ($name:ident, $MAX_MOMENT:expr) => (
        use ::conv::ApproxFrom;
        use ::num_traits::pow;

        /// An iterator over binomial coefficients.
        struct IterBinomial {
            a: u64,
            n: u64,
            k: u64,
        }

        impl IterBinomial {
            /// For a given n, iterate over all binomial coefficients binomial(n, k), for k=0...n.
            #[inline]
            pub fn new(n: u64) -> IterBinomial {
                IterBinomial {
                    k: 0,
                    a: 1,
                    n: n,
                }
            }
        }

        impl Iterator for IterBinomial {
            type Item = u64;

            #[inline]
            fn next(&mut self) -> Option<u64> {
                if self.k > self.n {
                    return None;
                }
                self.a = if !(self.k == 0) {
                    self.a * (self.n - self.k + 1) / self.k
                } else {
                    1
                };
                self.k += 1;
                Some(self.a)
            }
        }

        /// The maximal order of the moment to be calculated.
        const MAX_MOMENT: usize = $MAX_MOMENT;

        /// Estimate the first N moments of a sequence of numbers ("population").
        #[derive(Debug, Clone)]
        #[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
        pub struct $name {
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
            m: [f64; MAX_MOMENT - 1],
        }

        impl $name {
            /// Create a new moments estimator.
            #[inline]
            pub fn new() -> $name {
                $name {
                    n: 0,
                    avg: 0.,
                    m: [0.; MAX_MOMENT - 1],
                }
            }

            /// Determine whether the sample is empty.
            #[inline]
            pub fn is_empty(&self) -> bool {
                self.n == 0
            }

            /// Return the sample size.
            #[inline]
            pub fn len(&self) -> u64 {
                self.n
            }

            /// Estimate the mean of the population.
            ///
            /// Returns 0 for an empty sample.
            #[inline]
            pub fn mean(&self) -> f64 {
                self.avg
            }

            /// Estimate the `p`th central moment of the population.
            #[inline]
            pub fn central_moment(&self, p: usize) -> f64 {
                let n = f64::approx_from(self.n).unwrap();
                match p {
                    0 => 1.,
                    1 => 0.,
                    _ => self.m[p - 2] / n
                }
            }

            /// Estimate the `p`th standardized moment of the population.
            #[inline]
            pub fn standardized_moment(&self, p: usize) -> f64 {
                match p {
                    0 => f64::approx_from(self.n).unwrap(),
                    1 => 0.,
                    2 => 1.,
                    _ => {
                        let variance = self.central_moment(2);
                        assert_ne!(variance, 0.);
                        self.central_moment(p) / pow(variance.sqrt(), p)
                    },
                }
            }

            /// Calculate the sample variance.
            ///
            /// This is an unbiased estimator of the variance of the population.
            #[inline]
            pub fn sample_variance(&self) -> f64 {
                if self.n < 2 {
                    return 0.;
                }
                self.m[0] / f64::approx_from(self.n - 1).unwrap()
            }

            /// Calculate the sample skewness.
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

            /// Calculate the sample excess kurtosis.
            #[inline]
            pub fn sample_excess_kurtosis(&self) -> f64 {
                if self.n < 4 {
                    return 0.;
                }
                let n = f64::approx_from(self.n).unwrap();
                (n + 1.) * n * self.central_moment(4) /
                    ((n - 1.) * (n - 2.) * (n - 3.) * pow(self.central_moment(2), 2)) -
                    3. * pow(n - 1., 2) / ((n - 2.) * (n - 3.))
            }

            /// Add an observation sampled from the population.
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
                for p in 2..(MAX_MOMENT + 1) {
                    term1 *= factor1;
                    term2 *= factor2;
                    coeff_delta *= delta;
                    self.m[p - 2] += (term1 + term2) * coeff_delta;

                    let mut coeff = 1.;
                    let mut binom = IterBinomial::new(p as u64);
                    binom.next().unwrap();  // Skip k = 0.
                    for k in 1..(p - 1) {
                        coeff *= factor_coeff;
                        self.m[p - 2] += f64::approx_from(binom.next().unwrap()).unwrap() *
                            prev_m[p - 2 - k] * coeff;
                    }
                }
            }
        }

        impl $crate::Merge for $name {
            #[inline]
            fn merge(&mut self, other: &$name) {
                let n_a = f64::approx_from(self.n).unwrap();
                let n_b = f64::approx_from(other.n).unwrap();
                let delta = other.avg - self.avg;

                self.n += other.n;
                let n = f64::approx_from(self.n).unwrap();
                let n_a_over_n = n_a / n;
                let n_b_over_n = n_b / n;
                self.avg += n_b_over_n * delta;

                let factor_a = -n_b_over_n * delta;
                let factor_b = n_a_over_n * delta;
                let mut term_a = n_a * factor_a;
                let mut term_b = n_b * factor_b;
                let prev_m = self.m;
                for p in 2..(MAX_MOMENT + 1) {
                    term_a *= factor_a;
                    term_b *= factor_b;
                    self.m[p - 2] += other.m[p - 2] + term_a + term_b;

                    let mut coeff_a = 1.;
                    let mut coeff_b = 1.;
                    let mut coeff_delta = 1.;
                    let mut binom = IterBinomial::new(p as u64);
                    binom.next().unwrap();
                    for k in 1..(p - 1) {
                        coeff_a *= -n_b_over_n;
                        coeff_b *= n_a_over_n;
                        coeff_delta *= delta;
                        self.m[p - 2] +=
                            f64::approx_from(binom.next().unwrap()).unwrap() *
                            coeff_delta * (prev_m[p - 2 - k] * coeff_a +
                            other.m[p - 2 - k] * coeff_b);
                    }
                }
            }
        }

        impl core::default::Default for $name {
            fn default() -> $name {
                $name::new()
            }
        }

        impl_from_iterator!($name);
    );
}
