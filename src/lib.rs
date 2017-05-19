#![no_std]

extern crate conv;
#[cfg(test)] extern crate rand;
#[cfg(test)] #[macro_use] extern crate std;

#[macro_use] mod macros;
mod average;
mod weighted_average;
mod weighted_average2;

pub use average::Average;
pub use weighted_average::WeightedAverage;
pub use weighted_average2::WeightedAverage as WeightedAverage2;
