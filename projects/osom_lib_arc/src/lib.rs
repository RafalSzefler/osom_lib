//! This crate defines ABI-stable atomic reference counted pointers.
#![deny(warnings)]
#![allow(unused_features)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::inline_always, clippy::module_inception)]
#![no_std]

pub mod carc;
pub mod carc_array;
pub mod consts;
pub mod errors;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod std;
