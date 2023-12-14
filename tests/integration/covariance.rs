use average::Covariance;

#[test]
fn simple() {
    let mut cov = Covariance::new();
    assert!(cov.mean_x().is_nan());
    assert!(cov.mean_y().is_nan());
    assert!(cov.population_covariance().is_nan());
    assert!(cov.sample_covariance().is_nan());
    assert!(cov.population_pearson().is_nan());
    assert!(cov.sample_pearson().is_nan());

    cov.add(1., 5.);
    assert_eq!(cov.mean_x(), 1.);
    assert_eq!(cov.mean_y(), 5.);
    assert_eq!(cov.population_covariance(), 0.);
    assert!(cov.sample_covariance().is_nan());
    // TODO: pearson

    cov.add(2., 4.);
    assert_eq!(cov.mean_x(), 1.5);
    assert_eq!(cov.mean_y(), 4.5);
    assert_eq!(cov.population_covariance(), -0.25);
    assert_eq!(cov.sample_covariance(), -0.5);

    cov.add(3., 3.);
    assert_eq!(cov.mean_x(), 2.);
    assert_eq!(cov.mean_y(), 4.);
    assert_eq!(cov.population_covariance(), -2./3.);
    assert_eq!(cov.sample_covariance(), -1.);

    cov.add(4., 2.);
    assert_eq!(cov.mean_x(), 2.5);
    assert_eq!(cov.mean_y(), 3.5);
    assert_eq!(cov.population_covariance(), -1.25);
    assert_eq!(cov.sample_covariance(), -5./3.);

    cov.add(5., 1.);
    assert_eq!(cov.mean_x(), 3.);
    assert_eq!(cov.mean_y(), 3.);
    assert_eq!(cov.population_covariance(), -2.0);
    assert_eq!(cov.sample_covariance(), -2.5);
}