//! A private crate that exposes macros for osom_lib_reprc crate.
#![doc(hidden)]
#![deny(warnings)]
#![warn(clippy::all, clippy::pedantic)]

use proc_macro::TokenStream;

mod reprc_impl;

/// This macro attribute applies `#[repr(C)]` to the struct/enum,
/// implements the `ReprC` trait and ensure that each field is also
/// `ReprC`.
#[proc_macro_attribute]
pub fn reprc(attr: TokenStream, item: TokenStream) -> TokenStream {
    reprc_impl::reprc_impl(attr.into(), item.into()).into()
}
