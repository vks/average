use core;

use conv::ApproxFrom;

use super::{Estimate, Merge};

include!("mean.rs");
include!("variance.rs");
include!("skewness.rs");
include!("kurtosis.rs");

/// Alias for `Variance`.
pub type MeanWithError = Variance;
