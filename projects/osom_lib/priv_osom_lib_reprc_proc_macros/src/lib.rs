//! A private crate that exposes macros for osom_lib_reprc crate.
#![deny(warnings)]
#![allow(unused_features)]
#![doc(hidden)]
#![warn(clippy::all, clippy::pedantic)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{Item, parse_macro_input};

mod reprc_impl;

/// This macro attribute applies `#[repr(C)]` to the struct/enum,
/// implements the `ReprC` trait and ensure that each field is also
/// `ReprC`.
///
/// In addition in accepts a crate path as an argument, which is used to
/// reference the `ReprC` trait.
#[proc_macro_attribute]
#[doc(hidden)]
pub fn _reprc_with_crate(attr: TokenStream, item: TokenStream) -> TokenStream {
    reprc_impl::reprc_impl(attr.into(), item.into()).into()
}

/// This macro attribute applies `#[repr(C)]` to the struct/enum,
/// implements the `ReprC` trait and ensure that each field is also
/// `ReprC`.
#[proc_macro_attribute]
pub fn reprc(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as Item);
    quote! {
        #[::osom_lib_reprc::macros::_reprc_with_crate(::osom_lib_reprc)]
        #item
    }
    .into()
}
