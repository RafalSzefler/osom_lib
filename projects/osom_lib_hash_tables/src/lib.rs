//! This crate defines hash table traits, with some implementations.
//!
//! The crate is `#![no_std]` unless the `std` feature is enabled.
//!
//! Enable `serde` support by adding `serde` feature.
#![deny(warnings)]
#![allow(unused_features)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::redundant_field_names, clippy::inline_always, clippy::unreadable_literal)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod abseil;
pub mod bytell;
pub mod errors;
pub(crate) mod helpers;
pub mod traits;

pub mod defaults;

#[cfg(feature = "std")]
mod std_hash_map;

#[cfg(feature = "serde")]
mod serde_impl;
