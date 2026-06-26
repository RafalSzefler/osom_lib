//! Holds the definition of [`DynamicArray`] and [`InlineDynamicArray`].

pub(crate) mod internal_array;

#[allow(clippy::module_inception)]
mod dynamic_array;
pub use dynamic_array::*;

mod inline_dynamic_array;
pub use inline_dynamic_array::*;

mod aligned_dynamic_array;
pub use aligned_dynamic_array::*;
