# Changelog

Earlier history is in the [git log](https://github.com/vks/average/commits/master).

## 0.17.0

### Added

- `#[derive(Merge)]` proc macro (requires `derive` feature): automatically
  derives `average::Merge` for structs whose fields all implement `Merge`.
- `#[concatenate]` attribute macro (requires `derive` feature): generates
  `new()`, `Default`, `add()`, accessor methods, and `FromIterator` impls for
  a struct of estimator fields.  Use `#[statistics(method, ...)]` on a field
  to expose multiple accessors; without the attribute a single accessor named
  after the field is generated.
- `ExpMovingMean`: exponentially weighted moving average (EWMA).  Constructors:
  `with_alpha`, `with_span`, `with_half_life` (the latter two require `std` or
  `libm` feature).
- `ExpMovingVariance`: exponentially weighted moving variance.  Exposes
  `population_variance()`, `sample_variance()`, `standard_deviation()` (the
  last one requires `std` or `libm`).
- `ExpMovingMean::append` / `ExpMovingVariance::append` (requires `std` or
  `libm`): combine two estimators in time order so that the result is identical
  to feeding the concatenated stream to a single estimator.  `Merge` is
  intentionally **not** implemented for these types because exponential
  weighting is order-dependent.
- `Skewness::sample_skewness()` and `Kurtosis::sample_excess_kurtosis()`: the
  adjusted Fisher-Pearson bias-corrected estimators, matching Excel/SAS/scipy.

### Changed

- **Behavior change:** `sample_skewness()` and `sample_excess_kurtosis()` on
  types produced by `define_moments!` (including the built-in `Moments4`) now
  return `NaN` when `n < 3` and `n < 4` respectively.  Previous versions
  returned a finite (but mathematically incorrect) value for small samples via
  a method-of-moments fallback.  The formulas have also been corrected and
  verified against scipy.
- `Skewness` and `Kurtosis` docs: renamed "population variance of the sample"
  to "biased population variance" for clarity.
