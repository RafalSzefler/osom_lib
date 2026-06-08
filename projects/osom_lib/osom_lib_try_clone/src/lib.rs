//! This crate defines the [`TryClone`] trait and its base implementations.
#![deny(warnings)]
#![allow(unused_features)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::redundant_field_names, clippy::inline_always)]
#![no_std]

mod traits;
pub use traits::*;

mod impls;

/// Macros for the `osom_lib_try_clone` crate.
pub mod macros {
    #[doc(inline)]
    pub use priv_osom_lib_try_clone_proc_macros::try_clone;

    #[doc(inline)]
    pub use priv_osom_lib_try_clone_proc_macros::try_clone_with_clone;

    #[doc(hidden)]
    pub use priv_osom_lib_try_clone_proc_macros::_priv_try_clone;
}
