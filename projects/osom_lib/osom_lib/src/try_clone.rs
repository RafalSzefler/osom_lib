//! This module re-exports the `TryClone` trait and its macros.
pub use crate::__try_clone::TryClone;

/// Macros for the `osom_lib_try_clone` crate.
pub mod macros {
    #[doc(inline)]
    pub use priv_osom_lib_proc_macros::try_clone;

    #[doc(inline)]
    pub use priv_osom_lib_proc_macros::try_clone_with_clone;
}
