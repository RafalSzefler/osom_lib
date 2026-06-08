//! This crate defines various hashing functions, which
//! are ABI stable (as in `#[repr(C)]`).
//!
//! Additionally all the hash functions here work in the
//! const context.
//!
//! The crate is `#![no_std]`.
//!
//! # Notes
//!
//! In terms of raw performance:
//! * [`FxHash`][crate::fxhash::FxHash] is the fastest.
//! * [`SipHash`][crate::siphash::SipHash] and [`FNV1a_64`][crate::fnv::FNV1a_64] are over twice as slow.
//! * [`SHA2_256_Portable`][crate::sha2::sha2_256::portable::SHA2_256_Portable] is obviously the slowest, like 40x
//!   slower than [`FxHash`][crate::fxhash::FxHash].
//! * However platform specific implementations of `SHA2_256` are actually quite fast. Especially for bigger
//!   input these are comparable with [`SipHash`][crate::siphash::SipHash].
//!
//! So if cryptographic security is not a concern, then [`FxHash`][crate::fxhash::FxHash] is a valid choice,
//! since it also is quite ok in terms of collision resistance.
#![deny(warnings)]
#![allow(unused_features)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::redundant_field_names, clippy::inline_always, clippy::unreadable_literal)]
#![no_std]

pub mod fnv;
pub mod fxhash;
pub mod sha2;
pub mod siphash;
pub mod traits;
