#![cfg(feature = "serde")]

mod direct_string;

mod seeded;
pub use seeded::*;

cfg_select! {
    feature="std" => {
        mod std_string_cache;

        #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
        pub use std_string_cache::*;
    },
    _ => { }
}
