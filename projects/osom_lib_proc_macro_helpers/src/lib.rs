//! This is a private crate that holds various proc-macro helpers.
#![deny(warnings)]
#![allow(unused_features)]
#![doc(hidden)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::inline_always)]

pub mod options;
