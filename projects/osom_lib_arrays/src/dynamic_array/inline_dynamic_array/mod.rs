//! Holds the definition of [`InlineDynamicArray`].
#![allow(clippy::module_inception)]

mod immutable_inline_array;
mod inline_array;
mod mutable_inline_array;
mod other_inline_array_traits;

pub use inline_array::*;
