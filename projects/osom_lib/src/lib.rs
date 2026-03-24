//! This module gathers together all the osom tools into a single
//! crate.
//!
//! This crate is `#![no_std]`. If, however, the `std`
//! feature is enabled, it will include code that
//! depends on the standard Rust library (e.g. the
//! standard allocator).
#![deny(warnings)]
#![allow(unused_features)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![warn(clippy::all, clippy::pedantic)]
#![no_std]

mod macro_helpers;

crate::macro_helpers::reexport!(lib;
    primitives, alloc, arrays, cfg_ext, reprc, prng, hashes, hash_tables, macros, numbers);

crate::macro_helpers::reexport_std!(lib;
    entropy, entropy_cprng, wait_timer);
