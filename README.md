# average

Calculate statistics of a sequence iteratively in a single pass, using
constant memory and avoiding numerical problems. The calculations can be
easily parallelized by using `merge`.

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
* Minimum and maximum.
* Quantile.
