# average

Calculate statistics of a sequence iteratively in a single pass, using
constant memory and avoiding numerical problems. The calculations can be
easily parallelized by using `merge`.

[Documentation](https://docs.rs/average) |
[crates.io](https://crates.io/crates/average)

[![Build Status](https://travis-ci.org/vks/average.svg?branch=master)](https://travis-ci.org/vks/average)

## Implemented statistics

* Mean and its error.
* Variance, skewness, kurtosis.
* Minimum and maximum.
* Quantile.
