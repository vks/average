#[cfg(feature = "derive")]
mod tests {
    use average::{Estimate, Max, Mean, Merge};

    #[derive(Default, Merge)]
    struct Stats {
        mean: Mean,
        max: Max,
    }

    #[test]
    fn derive_merge_named_fields() {
        let mut a = Stats::default();
        let mut b = Stats::default();
        for x in [1.0_f64, 2.0, 3.0] {
            a.mean.add(x);
            a.max.add(x);
        }
        for x in [4.0_f64, 5.0, 6.0] {
            b.mean.add(x);
            b.max.add(x);
        }
        a.merge(&b);
        assert_eq!(a.max.max(), 6.0);
        // Mean of [1, 2, 3, 4, 5, 6] = 3.5
        assert!((a.mean.mean() - 3.5).abs() < 1e-10);
    }

    #[test]
    fn derive_merge_tuple_struct() {
        #[derive(Default, Merge)]
        struct Pair(Mean, Max);

        let mut a = Pair::default();
        let mut b = Pair::default();
        for x in [1.0_f64, 2.0] {
            a.0.add(x);
            a.1.add(x);
        }
        for x in [3.0_f64, 4.0] {
            b.0.add(x);
            b.1.add(x);
        }
        a.merge(&b);
        assert_eq!(a.1.max(), 4.0);
        assert!((a.0.mean() - 2.5).abs() < 1e-10);
    }

    #[test]
    fn derive_merge_generic_struct() {
        #[derive(Default, Merge)]
        struct Wrapper<T: Default> {
            inner: T,
        }

        let mut a = Wrapper::<Mean>::default();
        let mut b = Wrapper::<Mean>::default();
        for x in [1.0_f64, 2.0] {
            a.inner.add(x);
        }
        for x in [3.0_f64, 4.0] {
            b.inner.add(x);
        }
        a.merge(&b);
        assert!((a.inner.mean() - 2.5).abs() < 1e-10);
    }

    #[test]
    fn derive_merge_compile_fail() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compile-fail/derive-enum.rs");
    }

    // --- #[concatenate] attribute macro tests ---

    use average_derive::concatenate;
    use average::Min;

    #[derive(Merge)]
    #[concatenate]
    struct MinMax {
        min: Min,
        max: Max,
    }

    #[test]
    fn concatenate_simple() {
        let mut s = MinMax::new();
        for i in 1..6 {
            s.add(f64::from(i));
        }
        assert_eq!(s.min(), 1.0);
        assert_eq!(s.max(), 5.0);
    }

    #[test]
    fn concatenate_default() {
        let mut s = MinMax::default();
        for i in 1..6 {
            s.add(f64::from(i));
        }
        assert_eq!(s.min(), 1.0);
        assert_eq!(s.max(), 5.0);
    }

    #[test]
    fn concatenate_from_iterator() {
        let s: MinMax = (1..6).map(f64::from).collect();
        assert_eq!(s.min(), 1.0);
        assert_eq!(s.max(), 5.0);
    }

    #[test]
    fn concatenate_from_iterator_ref() {
        let data: Vec<f64> = (1..6).map(f64::from).collect();
        let s: MinMax = data.iter().collect();
        assert_eq!(s.min(), 1.0);
        assert_eq!(s.max(), 5.0);
    }

    #[test]
    fn concatenate_merge() {
        let mut a: MinMax = (1..4).map(f64::from).collect();
        let b: MinMax = (4..7).map(f64::from).collect();
        a.merge(&b);
        assert_eq!(a.min(), 1.0);
        assert_eq!(a.max(), 6.0);
    }

    #[test]
    fn concatenate_multi_statistics() {
        use average::Variance;

        #[concatenate]
        struct Estimator {
            #[statistics(mean, sample_variance)]
            variance: Variance,
            #[statistics(max)]
            max: Max,
        }

        let e: Estimator = (1..6).map(f64::from).collect();
        assert_eq!(e.mean(), 3.0);
        assert_eq!(e.sample_variance(), 2.5);
        assert_eq!(e.max(), 5.0);
    }

    #[cfg(any(feature = "std", feature = "libm"))]
    #[test]
    fn concatenate_moments_quantile() {
        use average::{Quantile, Variance};

        #[concatenate]
        struct Estimator {
            #[statistics(mean, sample_variance)]
            variance: Variance,
            #[statistics(quantile)]
            quantile: Quantile,
        }

        let e: Estimator = (1..6).map(f64::from).collect();
        assert_eq!(e.mean(), 3.0);
        assert_eq!(e.sample_variance(), 2.5);
        assert_eq!(e.quantile(), 3.0);
    }

    #[test]
    fn concatenate_pub() {
        mod submodule {
            use average::{Min, Max};
            use average_derive::concatenate;

            #[concatenate]
            pub struct MinMax {
                min: Min,
                max: Max,
            }
        }

        let mut s = submodule::MinMax::new();
        for i in 1..6 {
            s.add(f64::from(i));
        }
        assert_eq!(s.min(), 1.0);
        assert_eq!(s.max(), 5.0);
    }
}
