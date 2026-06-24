//! Holds definition of [`SharedString`] and its [`SharedStringBuilder`].
mod errors;
mod shared_string;
mod shared_string_builder;
mod weak_string;

pub use errors::*;
pub use shared_string::*;
pub use shared_string_builder::*;
pub use weak_string::*;

#[cfg(feature = "serde")]
pub mod serde;

cfg_select! {
    feature="std" => {
        #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
        pub mod std;
    },
    _ => {}
}
