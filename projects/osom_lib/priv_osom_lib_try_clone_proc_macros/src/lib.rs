//! A private crate that exposes macros for osom_lib_reprc crate.
#![deny(warnings)]
#![allow(unused_features)]
#![doc(hidden)]
#![warn(clippy::all, clippy::pedantic)]

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Path, parse_macro_input};

mod try_clone_impl;

/// This macro attribute implements the `TryClone` trait for the given type.
///
/// It accepts three arguments: boolean flag indicating whether to use the `Clone` trait,
/// a crate path to the `TryClone` trait, and a type name of the error type to use with the `TryClone` trait.
#[proc_macro_attribute]
#[doc(hidden)]
pub fn _priv_try_clone(attr: TokenStream, item: TokenStream) -> TokenStream {
    match try_clone_impl::try_clone_impl(attr.into(), item.into()) {
        Ok(ts) => ts.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// This macro attribute implements the `TryClone` trait for the given struct
/// or enum.
///
/// It accepts a single argument, which is the type name of error type to
/// use with the `TryClone` trait.
///
/// # Example
///
/// This definition:
///
/// ```rust,ignore
/// pub struct MyError;
///
/// #[try_clone(MyError)]
/// struct MyStruct {
///     a: u32,
///     b: bool,
/// }
/// ```
///
/// will generate the following code, in addition to the struct definition:
///
/// ```rust,ignore
/// impl TryClone for MyStruct {
///     type Error = MyError;
///
///     fn try_clone(&self) -> Result<Self, Self::Error> {
///         Ok(MyStruct {
///             a: TryClone::try_clone(&self.a)?,
///             b: TryClone::try_clone(&self.b)?,
///         })
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn try_clone(attr: TokenStream, item: TokenStream) -> TokenStream {
    let type_name = parse_macro_input!(attr as Path);
    let item: TokenStream2 = item.into();
    quote! {
        #[::osom_lib_try_clone::macros::_priv_try_clone(false, ::osom_lib_try_clone, #type_name)]
        #item
    }
    .into()
}

/// This macro attribute implements the `TryClone` trait for the given struct
/// or enum, and uses it to implement the `Clone` trait.
///
/// It accepts a single argument, which is the type name of error type to
/// use with the `TryClone` trait.
///
/// # Example
///
/// This definition:
///
/// ```rust,ignore
/// pub struct MyError;
///
/// #[try_clone_with_clone(MyError)]
/// struct MyStruct {
///     a: u32,
///     b: bool,
/// }
/// ```
///
/// will generate the following code, in addition to the struct definition:
///
/// ```rust,ignore
/// impl TryClone for MyStruct {
///     type Error = MyError;
///
///     fn try_clone(&self) -> Result<Self, Self::Error> {
///         Ok(MyStruct {
///             a: TryClone::try_clone(&self.a)?,
///             b: TryClone::try_clone(&self.b)?,
///         })
///     }
/// }
///
/// impl Clone for MyStruct {
///     fn clone(&self) -> Self {
///         TryClone::try_clone(self).expect("[MyStruct::try_clone] should not fail.")
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn try_clone_with_clone(attr: TokenStream, item: TokenStream) -> TokenStream {
    let type_name = parse_macro_input!(attr as Path);
    let item: TokenStream2 = item.into();
    quote! {
        #[::osom_lib_try_clone::macros::_priv_try_clone(true, ::osom_lib_try_clone, #type_name)]
        #item
    }
    .into()
}
