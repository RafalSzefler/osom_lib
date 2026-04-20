#![allow(clippy::unnecessary_wraps)]

#[allow(unused_macros)]
macro_rules! reexport {
    ( $name:ident ) => {
        mod $name;
        pub use $name::*;
    };
}

cfg_select! {
    target_os="macos" => {
        reexport!(libc_getentropy);
    },
    target_os="linux" => {
        reexport!(libc_getrandom);
    },
    target_os="windows" => {
        reexport!(windows_sys_process_prng);
    },
    _ => {
        compile_error!("Current target is not supported.");
    }
}
