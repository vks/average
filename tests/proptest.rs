#![cfg_attr(feature = "cargo-clippy", allow(float_cmp, map_clone))]

#[macro_use] extern crate proptest;

use core::iter::Iterator;

use average::Mean;
use proptest::prelude::*;
use crate::prop::num::f64;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10000))]
    #[test]
    fn reasonable_bounds(s in prop::collection::vec(
        f64::POSITIVE | f64::NEGATIVE | f64::SUBNORMAL | f64::ZERO, 1..100usize)) {
        // See https://hypothesis.works/articles/calculating-the-mean/.
        let max = s.iter().cloned().fold(0./0., f64::max);
        let min = s.iter().cloned().fold(0./0., f64::min);
        let a: Mean = s.iter().collect();
        let mean = a.mean();
        println!("min: {}  mean: {}  max: {}", min, mean, max);
        assert!(min <= mean);
        assert!(mean <= max);
    }
}
