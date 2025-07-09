//! A crate implementing several tree algorithms.
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
#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
pub mod traits;
