//! This crate defines entropy generators. Here by "entropy"
//! we mean "true" (or close enough) randomness. We can think
//! of it as (potentially) slow random number generators, but
//! with high quality randomness.
//!
//! The crate is `#![no_std]`. But it does use external
//! os specific dependencies (e.g. `libc` or `windows-sys`).
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![deny(warnings)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::inline_always)]
#![no_std]

pub mod std;
pub mod traits;
