# average

Calculate statistics of a sequence iteratively in a single pass, using
constant space and avoiding numerical problems. The calculations can be
easily parallelized by using `merge`.

This crate works without `std`.

[![Documentation Status]][docs.rs]
[![Latest Version]][crates.io]
[![Build Status]][travis]

[Documentation Status]: https://docs.rs/average/badge.svg
[docs.rs]: https://docs.rs/average
[Build Status]: https://travis-ci.org/vks/average.svg?branch=master
[travis]: https://travis-ci.org/vks/average
[Latest Version]: https://img.shields.io/crates/v/average.svg
[crates.io]: https://crates.io/crates/average


## Implemented statistics

* Mean and its error.
* Variance, skewness, kurtosis.
* Arbitrary moments.
* Minimum and maximum.
* Quantile.
* Histogram.


## Crate features

The following optional features are available:

* `serde1` enables serialization, via Serde version 1.
* `rayon` enables support for `rayon::iter::FromParallelIterator`.
* `nightly` enables the use of const generics for a histogram implementation
  without macros. Note that nightly features are not stable and therefore not
  all library and compiler versions will be compatible.


## Rust version requirements

Rustc version 1.36 or greater is supported.


## Related Projects

* [`quantiles`](https://crates.io/crates/quantiles):
  Provides quantile estimates with bounded error but using growing space.
