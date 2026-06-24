//! This crate defines various string helpers. All of the structs
//! are ABI stable (as in `#[repr(C)]`).
//!
//! The crate is `#![no_std]` unless the `std` feature is enabled.
//!
//! Enable `serde` support by adding `serde` feature.
#![deny(warnings)]
#![allow(unused_features)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::redundant_field_names, clippy::inline_always)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod shared;
