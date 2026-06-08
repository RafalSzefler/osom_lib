//! This module holds the [`CVR`] type ant its implementation.
mod core;
pub use core::*;

mod cvr_array;
pub use cvr_array::*;

mod cvr_bool;
pub use cvr_bool::*;

mod cvr_float;
pub use cvr_float::*;

mod cvr_int;
pub use cvr_int::*;

mod cvr_string;
pub use cvr_string::*;

mod cvr_object;
pub use cvr_object::*;

cfg_select! {
    feature="serde" => {
        mod serde_impl;

        /// This module holds `serde` related items.\
        #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
        pub mod serde {
            #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
            pub use super::serde_impl::*;
        }
    },
    _ => {}
}
