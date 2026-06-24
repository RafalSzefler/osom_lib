//! This crate defines various helpers and numeric algorithms.
#![deny(warnings)]
#![allow(unused_features)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::redundant_field_names, clippy::inline_always, clippy::unreadable_literal)]
#![no_std]

pub mod gcd;
pub mod iterators;
pub mod zigzag;
