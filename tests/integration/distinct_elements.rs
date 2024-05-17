use average::{assert_almost_eq, DistinctElements, Estimate, MeanWithError};

#[test]
fn test_trivial_all_distinct() {
    let a: DistinctElements<10> = (0..10).collect();
    assert_eq!(a.estimate(), 10.);
}

#[test]
fn test_trivial_half_distinct() {
    let mut a: DistinctElements<10> = (0..5).collect();
    a.extend(0..5);
    assert_eq!(a.estimate(), 5.);
}

#[test]
fn test_average_all_distinct() {
    let mut avg = MeanWithError::new();
    for _ in 0..1000 {
        let a: DistinctElements<10> = (0..1000).collect();
        avg.add(a.estimate());
    }
    assert_almost_eq!(avg.mean(), 1000., 50.);
}

#[test]
fn test_average_half_distinct() {
    let mut avg = MeanWithError::new();
    for _ in 0..1000 {
        let mut a: DistinctElements<10> = (0..500).collect();
        a.extend(0..500);
        avg.add(a.estimate());
    }
    assert_almost_eq!(avg.mean(), 500., 25.);
}
