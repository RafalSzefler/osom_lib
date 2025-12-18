macro_rules! reexport {
    ($variant:ident; $name: ident) => {
        paste::paste! {
            #[doc(no_inline)]
            pub use [< osom_ $variant _ $name >] as $name;
        }
    };
    ($variant: ident; $name: ident, $($names:ident),* $(,)?) => {
        crate::macro_helpers::reexport!($variant; $name);
        crate::macro_helpers::reexport!($variant; $($names),*);
    };
}

pub(crate) use reexport;

macro_rules! reexport_std {
    ($variant:ident; $name: ident) => {
        paste::paste! {
            #[cfg(feature="std")]
            #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
            #[doc(no_inline)]
            pub use [< osom_ $variant _ $name >] as $name;
        }
    };
    ($variant: ident; $name: ident, $($names:ident),* $(,)?) => {
        crate::macro_helpers::reexport_std!($variant; $name);
        crate::macro_helpers::reexport_std!($variant; $($names),*);
    };
}

pub(crate) use reexport_std;
