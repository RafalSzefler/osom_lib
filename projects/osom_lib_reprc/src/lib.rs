//! This crate holds tools that help with `#[repr(C)]` representations.
//!
//! This crate is `#![no_std]`.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![deny(warnings)]
#![warn(clippy::all, clippy::pedantic)]
#![no_std]

pub mod macros;
pub mod traits;
