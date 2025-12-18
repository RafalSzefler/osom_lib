//! Holds the definition of [`FixedArray`].

mod const_fixed_array;
pub use const_fixed_array::*;

mod const_buffer;
pub use const_buffer::*;

#[allow(clippy::module_inception)]
mod fixed_array;
pub use fixed_array::*;
