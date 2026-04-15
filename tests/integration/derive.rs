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
}
