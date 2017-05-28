//! This crate provides estimators for the weighted and unweighted average of a
//! sequence of numbers, and for their standard errors. The typical workflow
//! looks like this:
//!
//! 1. Initialize your estimator of choice ([`Mean`], [`MeanWithError`],
//!    [`WeightedMean`] or [`WeightedMeanWithError`]) with `new()`.
//! 2. Add some subset (called "samples") of the sequence of numbers (called
//!    "population") for which you want to estimate the average, using `add()`
//!    or `collect()`.
//! 3. Calculate the arithmetic mean with `mean()` and its standard error with
//!    `error()`.
//!
//! You can run several estimators in parallel and merge them into one with
//! `merge()`.
//!
//! Everything is calculated iteratively in a single pass using constant memory,
//! so the sequence of numbers can be an iterator. The used algorithms try to
//! avoid numerical instabilities.
//!
//! [`Mean`]: ./average/struct.Mean.html
//! [`MeanWithError`]: ./average/struct.MeanWithError.html
//! [`WeightedMean`]: ./weighted_average/struct.WeightedMean.html
//! [`WeightedMeanWithError`]: ./weighted_average/struct.WeightedMeanWithError.html
//!
//!
//! ## Example
//!
//! ```
//! use average::MeanWithError;
//!
//! let mut a: MeanWithError = (1..6).map(Into::into).collect();
//! a.add(42.);
//! println!("The average is {} ± {}.", a.mean(), a.error());
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
