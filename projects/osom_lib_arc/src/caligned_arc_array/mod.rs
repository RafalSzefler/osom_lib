//! This module holds the [`CAlignedArcArray`] and [`CAlignedWeakArray`] types and their implementations.
//! It also provides the [`CAlignedArcArrayBuilder`] type for iteratively constructing [`CAlignedArcArray`].

mod internal;
mod layout;

mod caligned_arc_array;
pub use caligned_arc_array::*;

mod caligned_weak_array;
pub use caligned_weak_array::*;

mod caligned_arc_array_builder;
pub use caligned_arc_array_builder::*;
