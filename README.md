# average

Calculate the average of a sequence and its error iteratively, using constant
memory and avoiding numerical problems. The calculation can be easily parallelized
by using `Average::merge`.

[Documentation](https://docs.rs/average) |
[crates.io](https://crates.io/crates/average)

[![Build Status](https://travis-ci.org/vks/average.svg?branch=master)](https://travis-ci.org/vks/average)

## Advantages over naive calculation of average and variance

* Avoids loss of precision due to cancellation.
* Only needs a single pass over the samples, at the cost of a division inside the loop.
