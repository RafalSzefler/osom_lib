//! This crate defines various primitives that osom
//! libraries use.
//!
//! The crate is `#![no_std]`.
#![deny(warnings)]
#![allow(unused_features)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::inline_always)]
#![no_std]

pub mod align;
pub mod coption;
pub mod cresult;
pub mod kvp;
pub mod length;
mod length_ops;
pub mod power_of_two;

pub(crate) mod as_i32;

mod checks;

#[cfg(feature = "serde")]
mod serde_impl;
