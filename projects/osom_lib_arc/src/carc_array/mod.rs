//! This module holds the [`CArcArray`] and [`CWeakArray`] types and their implementations.
//! It also provides the [`CArcArrayBuilder`] type for iteratively constructing [`CArcArray`].

mod internal;
mod layout;

mod carc_array;
pub use carc_array::*;

mod weak_array;
pub use weak_array::*;

mod carc_array_builder;
pub use carc_array_builder::*;
