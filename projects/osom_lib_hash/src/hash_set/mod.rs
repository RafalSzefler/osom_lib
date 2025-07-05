//! This module contains the implementation of the [`HashSet`] type.
#![allow(clippy::module_inception)]

mod quadratic_index_sequence;

mod equivalent;
pub use equivalent::*;

mod hash_set;
pub use hash_set::*;

pub mod errors;
pub mod operation_results;
