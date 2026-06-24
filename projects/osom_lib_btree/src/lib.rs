//! This crate defines and implements the B Tree related
//! data structures and algorithms.
#![deny(warnings)]
#![allow(unused_features)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::redundant_field_names, clippy::inline_always)]
#![no_std]

pub mod btree;
pub mod defaults;
pub mod errors;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod std;
