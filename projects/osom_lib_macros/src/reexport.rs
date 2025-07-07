/// Reexports a module if a feature is enabled.
///
/// # Example
/// Lets say we have a module `baz` that we want to reexport
/// only when `test` feature is enabled.
///
/// We can simply do:
///
/// ```ignore
/// use osom_lib_macros::reexport_if_feature;
/// reexport_if_feature!("test", baz);
/// ```
///
/// which will be expanded to:
///
/// ```ignore
/// mod baz;
///
/// #[cfg(feature = "test")]
/// #[cfg_attr(docsrs, doc(cfg(feature = "test")))]
/// #[allow(unused_imports)]
/// pub use baz::*;
/// ```
///
/// The `docsrs` is a custom attribute that you can set to
/// enable nightly features in the docs, like this:
///
/// ```
/// #![cfg_attr(docsrs, feature(doc_cfg))]
/// ```
///
/// and then pass `--cfg docsrs` to `RUSTFLAGS` and `RUSTDOCFLAGS`
/// environment variables during docs build.
#[macro_export]
macro_rules! reexport_if_feature {
    ($feature:literal, $mod_name:ident) => {
        mod $mod_name;

        #[cfg(feature = $feature)]
        #[cfg_attr(docsrs, doc(cfg(feature = $feature)))]
        #[allow(unused_imports)]
        pub use $mod_name::*;
    };
}
