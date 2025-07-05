//! Holds macros for osom projects. These macros should be used in tests only.
#![deny(warnings)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::needless_return,
    clippy::redundant_field_names,
    clippy::unreadable_literal,
    clippy::inline_always,
    clippy::module_name_repetitions,
    clippy::len_without_is_empty
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]

#[doc(hidden)]
pub mod _hidden;

pub mod models;
pub mod traits;

mod assert_eq_hex;
mod convert_to_fn;
