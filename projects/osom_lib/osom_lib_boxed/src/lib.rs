//! This crate defines ABI-stable box type.
#![deny(warnings)]
#![allow(unused_features)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::inline_always)]
#![no_std]

pub mod cbox;
pub mod errors;
mod layout;

#[cfg(feature = "std")]
pub mod std;

#[cfg(feature = "serde")]
mod serde_impl;
