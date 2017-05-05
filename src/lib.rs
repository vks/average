#![no_std]

extern crate conv;
#[cfg(test)] extern crate rand;
#[cfg(test)] #[macro_use] extern crate std;

#[macro_use] mod macros;
mod average;

pub use average::Average;
