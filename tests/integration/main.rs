#![cfg_attr(feature = "nightly",
   feature(const_generics, const_evaluatable_checked))]

mod histogram;
#[cfg(feature = "nightly")]
mod histogram_const;
#[cfg(any(feature = "std", feature = "libm"))]
mod kurtosis;
mod macros;
mod max;
mod mean;
mod min;
mod moments;
mod proptest;
#[cfg(any(feature = "std", feature = "libm"))]
mod quantile;
#[cfg(any(feature = "std", feature = "libm"))]
mod random;
#[cfg(any(feature = "std", feature = "libm"))]
mod skewness;
mod streaming_stats;
mod weighted_mean;
