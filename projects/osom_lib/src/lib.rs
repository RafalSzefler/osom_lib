//! Crate that combines and reexports other `osom_lib` crates.
#![deny(warnings)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::needless_return,
    clippy::redundant_field_names,
    clippy::unreadable_literal,
    clippy::inline_always,
    clippy::module_name_repetitions
)]
#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]

macro_rules! reexport {
    ($crate_name:ident) => {
        paste::paste! {
            pub use [<osom_lib_ $crate_name>] as $crate_name;
        }
    };
}

reexport!(alloc);
reexport!(arrays);
reexport!(macros);
reexport!(primitives);
reexport!(strings);
reexport!(hash);
