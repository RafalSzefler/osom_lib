//! Holds the definition of [`ConstFixedArray`], [`InlineFixedArray`] and [`FixedArray`].

mod const_fixed_array;
pub use const_fixed_array::*;

mod const_buffer;
pub use const_buffer::*;

mod inline_fixed_array;
pub use inline_fixed_array::*;

#[allow(clippy::module_inception)]
mod fixed_array;
pub use fixed_array::*;
