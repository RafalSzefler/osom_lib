//! Holds definition of immutable string and its builder.
mod errors;
mod immutable_string;
mod immutable_string_builder;
mod internal_string;
mod internal_string_layout;
mod weak_string;

pub use errors::*;
pub use immutable_string::*;
pub use immutable_string_builder::*;
pub use weak_string::*;

cfg_select! {
    feature="std" => {
        #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
        mod std;

        pub use std::*;
    },
    _ => {}
}
