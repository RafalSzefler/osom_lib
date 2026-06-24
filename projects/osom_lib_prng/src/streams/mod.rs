//! Holds implementations of various PRNG streams.

macro_rules! reexport {
    ( $id: ident ) => {
        mod $id;
        #[allow(unused_imports)]
        pub use $id::*;
    };
    ( $id: ident, $($ids:ident),* $(,)?) => {
        reexport!($id);
        reexport!($($ids),*);
    };
}

reexport!(chacha);
