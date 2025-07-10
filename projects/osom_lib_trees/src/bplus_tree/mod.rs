//! A module containing the implementation of the B+ tree data structure.
#![allow(clippy::module_inception)]
mod node;
mod operation_results;

mod bplus_tree;
pub use bplus_tree::*;
