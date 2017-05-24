//! This crate provides estimators for the weighted and unweighted average of a
//! sequence of numbers, and for their standard errors. The typical workflow
//! looks like this:
//!
//! 1. Initialize your estimator of choice ([`Average`], [`AverageWithError`],
//!    [`WeightedAverage`] or [`WeightedAverageWithError`]) with `new()`.
//! 2. Add some subset (called "samples") of the sequence of numbers (called
//!    "population") for which you want to estimate the average, using `add()`
//!    or `collect()`.
//! 3. Calculate the arithmetic mean with `mean()` and its standard error with
//!    `error()`.
//!
//! You can run several estimators in parallel and merge them into one with
//! `merge()`.
//!
//! [`Average`]: ./average/struct.Average.html
//! [`AverageWithError`]: ./average/struct.AverageWithError.html
//! [`WeightedAverage`]: ./weighted_average/struct.WeightedAverage.html
//! [`WeightedAverageWithError`]: ./weighted_average/struct.WeightedAverageWithError.html
//!
//!
//! ## Example
//!
//! ```
//! use average::AverageWithError;
//!
//! let mut a: AverageWithError = (1..6).map(Into::into).collect();
//! a.add(42.);
//! println!("The average is {} ± {}.", a.mean(), a.error());
//! ```

#![no_std]

extern crate conv;

#[macro_use] mod macros;
mod average;
mod weighted_average;

pub use average::{Average, AverageWithError};
pub use weighted_average::{WeightedAverage, WeightedAverageWithError};
