//! This crate defines the CVR (Canonical Value Representation) lib.
//!
//! The CVR objects are conceptually very similar to JSON objects.
//! The main difference is that the representation is canonical,
//! meaning if two JSONs represent the same value (that happens
//! for example when they differ only in key ordering), they will produce
//! the same CVR.
//!
//! Additionally all the CVR objects are ABI stable (as in `#[repr(C)]`).
//!
//! # Features
//!
//! * `std` - enables CVR backed by the standard allocator.
//! * `serde` - enables serde support.
#![deny(warnings)]
#![allow(unused_features)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::redundant_field_names, clippy::inline_always)]
#![cfg_attr(not(feature = "std"), no_std)]

mod cvr;
pub use cvr::*;

pub mod errors;
pub mod tools;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod std;
