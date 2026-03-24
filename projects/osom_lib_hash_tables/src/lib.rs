//! This crate defines hash table traits, with some implementations.
//!
//! The crate is `#![no_std]`.
#![deny(warnings)]
#![allow(unused_features)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::redundant_field_names, clippy::inline_always, clippy::unreadable_literal)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod bytell;
pub(crate) mod helpers;
pub mod traits;

pub mod defaults;

#[cfg(feature = "std")]
mod std_hash_map;
