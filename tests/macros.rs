#![cfg_attr(feature = "cargo-clippy", allow(float_cmp))]

use average::{Estimate, Min, Max, concatenate};

concatenate!(MinMax, [Min, min], [Max, max]);

#[test]
fn concatenate_simple() {
    {
        let mut s = MinMax::new();
        for i in 1..6 {
            s.add(f64::from(i));
        }

        assert_eq!(s.min(), 1.0);
        assert_eq!(s.max(), 5.0);
    }

    {
        let mut s = MinMax::default();
        for i in 1..6 {
            s.add(f64::from(i));
        }

        assert_eq!(s.min(), 1.0);
        assert_eq!(s.max(), 5.0);
    }

    {
        let s: MinMax = (1..6).map(f64::from).collect();

        assert_eq!(s.min(), 1.0);
        assert_eq!(s.max(), 5.0);
    }
}

#[test]
fn concatenate_moments() {
    use average::{Variance, Quantile};

    concatenate!(Estimator,
        [Variance, variance, mean, sample_variance],
        [Quantile, quantile, quantile]);

    let e: Estimator = (1..6).map(f64::from).collect();

    assert_eq!(e.mean(), 3.0);
    assert_eq!(e.sample_variance(), 2.5);
    assert_eq!(e.quantile(), 3.0);
}
