#![cfg_attr(feature = "nightly",
   feature(const_generics, const_evaluatable_checked))]

mod histogram;
#[cfg(feature = "nightly")]
mod histogram_const;
mod kurtosis;
mod macros;
mod max;
mod mean;
mod min;
mod moments;
mod proptest;
mod quantile;
mod random;
mod skewness;
mod streaming_stats;
mod weighted_mean;
