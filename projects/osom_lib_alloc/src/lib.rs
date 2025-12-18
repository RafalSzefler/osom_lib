//! This crate defines allocator related traits and structs.
//!
//! The crate is `#![no_std]`.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![deny(warnings)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::inline_always)]
#![no_std]

pub mod traits;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod std_allocator;
