//! This module holds the [`PowerOfTwo32`] and [`PowerOfTwo64`] primitives.

mod errors;
pub use errors::*;

mod power_of_two_32;
pub use power_of_two_32::*;

mod power_of_two_64;
pub use power_of_two_64::*;
