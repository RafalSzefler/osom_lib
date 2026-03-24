//! This crate holds wait timers, that are useful for accurate sleep.
//!
//! This crate is `#![no_std]`.
#![deny(warnings)]
#![allow(unused_features)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::needless_return, clippy::inline_always)]
#![no_std]

pub mod traits;

mod platforms;
pub use platforms::*;
