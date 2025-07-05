//! Crate that combines and reexports other `osom_lib` crates.
#![deny(warnings)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::needless_return,
    clippy::redundant_field_names,
    clippy::unreadable_literal,
    clippy::inline_always,
    clippy::module_name_repetitions
)]
#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]

pub use osom_lib_alloc as alloc;
pub use osom_lib_arrays as arrays;
pub use osom_lib_macros as macros;
pub use osom_lib_primitives as primitives;
pub use osom_lib_strings as strings;
pub use osom_lib_hash as hash;
