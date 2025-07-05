//! Allocator traits, optionally implementing std allocator if enabled through `std_alloc` feature.
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

use osom_lib_macros::reexport_if_feature;

mod traits;
pub use traits::*;

#[cfg(feature = "std_alloc")]
extern crate alloc;

reexport_if_feature!("std_alloc", std_allocator);
