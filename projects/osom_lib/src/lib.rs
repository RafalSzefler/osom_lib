//! This module gathers together all the osom tools into a single
//! crate.
//!
//! This crate is `#![no_std]`. If, however, the `std`
//! feature is enabled, it will include code that
//! depends on the standard Rust library (e.g. the
//! standard allocator).
//!
//! Enable `serde` support by adding `serde` feature.
#![deny(warnings)]
#![allow(unused_features)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![warn(clippy::all, clippy::pedantic)]
#![no_std]

mod macro_helpers;

crate::macro_helpers::reexport!(lib;
    primitives, alloc, arrays, arc, boxed, prng, btree,
    hashes, hash_tables, macros, numbers, strings,
);

crate::macro_helpers::reexport_std!(lib;
    entropy, entropy_cprng);

// re-export reprc and its macros
#[doc(hidden)]
pub use osom_lib_reprc as __reprc;
pub mod reprc;

// re-export try_clone and its macros
#[doc(hidden)]
pub use osom_lib_try_clone as __try_clone;
pub mod try_clone;
