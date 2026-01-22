//! This crate defines various array implementations. All of them
//! are ABI stable (as in `#[repr(C)]`).
//!
//! The crate is `#![no_std]`.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![deny(warnings)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::redundant_field_names, clippy::inline_always)]
#![no_std]

pub mod const_helpers;
pub mod dynamic_array;
pub mod errors;
pub mod fixed_array;
pub mod inline_array;
pub mod traits;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod std;

pub(crate) mod internal_array;
