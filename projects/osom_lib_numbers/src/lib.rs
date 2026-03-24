//! This crate defines various helpers and numeric algorithms.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![deny(warnings)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::redundant_field_names, clippy::inline_always, clippy::unreadable_literal)]
#![no_std]

mod iter_triangular;
pub use iter_triangular::*;
