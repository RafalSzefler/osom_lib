//! Holds definition of [`ImmutableString`] and its [`ImmutableStringBuilder`].
mod errors;
mod immutable_string;
mod immutable_string_builder;
mod weak_string;

pub use errors::*;
pub use immutable_string::*;
pub use immutable_string_builder::*;
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
