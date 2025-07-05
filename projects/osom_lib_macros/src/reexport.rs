#[macro_export]
macro_rules! reexport_if_feature {
    ($feature:literal, $mod_name:ident) => {
        #[cfg(feature = $feature)]
        mod $mod_name;

        #[cfg(feature = $feature)]
        #[cfg_attr(docsrs, doc(cfg(feature = $feature)))]
        #[allow(unused_imports)]
        pub use $mod_name::*;
    };
}
