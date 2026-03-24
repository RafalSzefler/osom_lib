//! This is a private crate that holds various proc-macros that
//! are used by osom_lib_proc_macro_helpers.
#![deny(warnings)]
#![allow(unused_features)]
#![doc(hidden)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![warn(clippy::all, clippy::pedantic)]

mod get_options_inner;

/// A proc-macro that retrieves an option value with the specified key and value.
#[proc_macro]
pub fn get_options(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match get_options_inner::get_options_inner(item.into()) {
        Ok(ok) => ok.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
