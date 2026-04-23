//! This crate defines various string helpers. All of the structs
//! are ABI stable (as in `#[repr(C)]`).
//!
//! The crate is `#![no_std]`.
#![deny(warnings)]
#![allow(unused_features)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::redundant_field_names, clippy::inline_always)]
#![no_std]

pub mod immutable;
