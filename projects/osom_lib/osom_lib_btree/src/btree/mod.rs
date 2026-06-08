//! This module defines the [`BTree`] data structure and its related types.
#![allow(clippy::explicit_deref_methods)]

mod node_layout;
mod node_ptr;

mod config;
pub use config::*;

mod the_tree;
pub use the_tree::*;

pub mod inspect;

mod operations;
