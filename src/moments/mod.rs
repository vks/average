use core;

use conv::ApproxFrom;

use super::{Estimate, Merge};

include!("mean.rs");
include!("variance.rs");
include!("skewness.rs");
include!("kurtosis.rs");

// It is possible to calculate higher moments the same way,
// see https://doi.org/10.1007/s00180-015-0637-z.

/// Alias for `Variance`.
pub type MeanWithError = Variance;
