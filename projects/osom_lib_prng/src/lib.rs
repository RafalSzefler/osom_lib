//! This crate defines pseudo random number generators (PRNG).
//!
//! The crate is `#![no_std]`.
#![deny(warnings)]
#![allow(unused_features)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::inline_always, clippy::unreadable_literal)]
#![no_std]

pub mod defaults;
pub mod errors;
pub mod prngs;
pub mod stream_prng;
pub mod streams;
pub mod traits;

mod aligned_array;
