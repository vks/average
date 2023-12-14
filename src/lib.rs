//! This crate provides estimators for statistics on a sequence of numbers. The
//! typical workflow looks like this:
//!
//! 1. If necessary, build your custom estimator using [`concatenate`] or
//!    [`define_moments`].
//! 2. Initialize the estimator of your choice with `new()`.
//! 3. Add some subset (called "sample") of the sequence of numbers (called
//!    "population") for which you want to estimate the statistic, using `add()`
//!    or `collect()`.
//! 4. Calculate the statistic with `mean()` or similar.
//!
//! You can run several estimators in parallel and merge them into one with
//! `merge()`.
//!
//! Everything is calculated iteratively in a single pass using constant memory,
//! so the sequence of numbers can be an iterator. The used algorithms try to
//! avoid numerical instabilities.
//!
//! If you want [Serde](https://github.com/serde-rs/serde) support,
//! include `"serde1"` in your list of features.
//!
//! Note that deserializing does not currently check for all invalid inputs.
//! For example, if you deserialize a corrupted [`Variance`] it may return
//! a negative value for variance, even though that is mathematically impossible.
//! In a future minor release some of these checks may be added.
//!
//!
//! ### Example
//!
//! ```
//! use average::{MeanWithError, Estimate};
//!
//! let mut a: MeanWithError = (1..6).map(f64::from).collect();
//! a.add(42.);
//! println!("The mean is {} ± {}.", a.mean(), a.error());
//! ```
//!
//!
//! ## Estimators
//!
//! * Mean ([`Mean`]) and its error ([`MeanWithError`]).
//! * Weighted mean ([`WeightedMean`]) and its error
//!   ([`WeightedMeanWithError`]).
//! * Variance ([`Variance`]), skewness ([`Skewness`]) and kurtosis
//!   ([`Kurtosis`]).
//! * Arbitrary higher moments ([`define_moments`]).
//! * Quantiles ([`Quantile`]).
//! * Minimum ([`Min`]) and maximum ([`Max`]).
//!
//!
//! ## Estimating several statistics at once
//!
//! The estimators are designed to have minimal state. The recommended way to
//! calculate several of them at once is to create a struct with all the
//! estimators you need. You can then implement `add` for your struct by
//! forwarding to the underlying estimators. Everything is inlined, so there
//! should be no overhead.
//!
//! You can avoid the boilerplate code by using the [`concatenate`] macro.
//!
//! Note that calculating moments requires calculating the lower moments, so you
//! only need to include the highest moment in your struct.
//!
//!
//! ## Calculating histograms
//!
//! The [`define_histogram`] macro can be used to define a histogram struct that
//! uses constant memory. See [`Histogram10`] (defined using
//! `define_histogram!(..., 10)`) and the extension trait [`Histogram`]
//! for the methods available to the generated struct.
//!
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
//! [`concatenate`]: ./macro.concatenate.html
//! [`define_moments`]: ./macro.define_moments.html
//! [`define_histogram`]: ./macro.define_histogram.html
//! [`Histogram10`]: ./struct.Histogram10.html
//! [`Histogram`]: ./trait.Histogram.html

#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![allow(clippy::float_cmp, clippy::suspicious_operation_groupings)]
#![no_std]
#![forbid(unsafe_code)]
#![forbid(missing_docs)]
#![forbid(missing_debug_implementations)]
#![cfg_attr(feature = "nightly", feature(generic_const_exprs))]
#[cfg(feature = "std")] extern crate std;

#[macro_use]
mod macros;
#[macro_use]
mod moments;
mod minmax;
#[cfg(any(feature = "std", feature = "libm"))]
#[cfg_attr(doc_cfg, doc(cfg(any(feature = "std", feature = "libm"))))]
mod quantile;
mod traits;
mod weighted_mean;
#[macro_use]
mod histogram;
#[cfg(feature = "nightly")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nightly")))]
pub mod histogram_const;
mod covariance;

#[cfg(any(feature = "std", feature = "libm"))]
#[cfg_attr(doc_cfg, doc(cfg(any(feature = "std", feature = "libm"))))]
pub use crate::moments::{Kurtosis, Skewness};
pub use crate::moments::{Mean, MeanWithError, Variance};

pub use crate::histogram::{InvalidRangeError, SampleOutOfRangeError};
pub use crate::minmax::{Max, Min};
#[cfg(any(feature = "std", feature = "libm"))]
#[cfg_attr(doc_cfg, doc(cfg(any(feature = "std", feature = "libm"))))]
pub use crate::quantile::Quantile;
pub use crate::traits::{Estimate, Histogram, Merge};
pub use crate::weighted_mean::{WeightedMean, WeightedMeanWithError};
pub use crate::covariance::Covariance;

define_histogram!(hist, 10);
pub use crate::hist::Histogram as Histogram10;

define_moments!(Moments4, 4);
