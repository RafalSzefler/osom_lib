//! Holds implementations of various PRNGs.

pub(crate) mod helpers;

macro_rules! reexport {
    ( $id: ident ) => {
        mod $id;
        #[doc(inline)]
        pub use $id::*;
    };
    ( $id: ident, $($ids:ident),* $(,)?) => {
        reexport!($id);
        reexport!($($ids),*);
    };
}

reexport!(lcg, splitmix, chacha);
