//! This crate defines and implements an entropy generator,
//! that is base on cryptographically secure PRNG. It
//! uses an OS-specific entropy only to seed the CPRNG.
//!
//! This create is `#![no_std]`.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![deny(warnings)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::inline_always)]
#![no_std]

pub mod cprng_entropy;

/// [`CPRNGEntropy`][`crate::cprng_entropy::CPRNGEntropy`] backed by `ChaCha<20>` algorithm.
pub type DefaultEntropy = cprng_entropy::CPRNGEntropy<osom_lib_prng::prngs::ChaCha<20>>;
