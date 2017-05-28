//! This crate provides estimators for statistics on a sequence of numbers. The
//! typical workflow looks like this:
//!
//! 1. Initialize the estimator of your choice with `new()`.
//! 2. Add some subset (called "sample") of the sequence of numbers (called
//!    "population") for which you want to estimate the statistic, using `add()`
//!    or `collect()`.
//! 3. Calculate the statistic with `mean()` or similar.
//!
//! You can run several estimators in parallel and merge them into one with
//! `merge()`.
//!
//! Everything is calculated iteratively in a single pass using constant memory,
//! so the sequence of numbers can be an iterator. The used algorithms try to
//! avoid numerical instabilities.
//!
//!
//! ## Estimators
//!
//! * Mean ([`Mean`]) and its error ([`MeanWithError`]).
//! * Weighted mean ([`WeightedMean`]) and its error
//!   ([`WeightedMeanWithError`]).
//! * Variance ([`Variance`]), skewness ([`Skewness`]) and kurtosis
//!   ([`Kurtosis`]).
//! * Quantiles ([`Quantile`]).
//! * Minimum ([`Min`]) and maximum ([`Max`]).
//!
//! [`Mean`]: ./struct.Mean.html
//! [`MeanWithError`]: ./type.MeanWithError.html
//! [`WeightedMean`]: ./struct.WeightedMean.html
//! [`WeightedMeanWithError`]: ./struct.WeightedMeanWithError.html
//! [`Variance`]: ./struct.Variance.html
//! [`Skewness`]: ./struct.Skewness.html
//! [`Kurtosis`]: ./struct.Kurtosis.html
//! [`Quantile`]: ./struct.Quantile.html
//! [`Min`]: ./struct.Min.html
//! [`Max`]: ./struct.Max.html
//!
//!
//! ## Estimating several statistics at once
//!
//! The estimators are designed to have minimal state. The recommended way to
//! calculate several of them at once is to create a struct with all the
//! estimators you need. You can then implement `add` for your struct by
//! forwarding to the underlying estimators.
//!
//! Note that calculating moments requires calculating the lower moments, so you
//! only need to include the highest moment in your struct.
//!
//!
//! ## Example
//!
//! ```
//! use average::MeanWithError;
//!
//! let mut a: MeanWithError = (1..6).map(Into::into).collect();
//! a.add(42.);
//! println!("The mean is {} ± {}.", a.mean(), a.error());
//! ```

#![no_std]

extern crate conv;
extern crate quickersort;

#[macro_use] mod macros;
mod moments;
mod weighted_mean;
mod minmax;
mod reduce;
mod quantile;

pub use moments::{Mean, Variance, Skewness, Kurtosis, MeanWithError};
pub use weighted_mean::{WeightedMean, WeightedMeanWithError};
pub use minmax::{Min, Max};
pub use quantile::Quantile;
