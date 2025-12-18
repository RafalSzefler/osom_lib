use osom_lib_cfg_ext::{cfg_match, identity};

identity! {
    fn mapped_identity() -> i32 { 7 }
}

#[test]
fn test_identity() {
    assert_eq!(mapped_identity(), 7);
}

cfg_match!(
    (target_os = "windows") => {
        fn os_name() -> &'static str {
            "W"
        }
    },
    (target_os = "linux") => {
        fn os_name() -> &'static str {
            "L"
        }
    },
    _ => {
        fn os_name() -> &'static str {
            "U"
        }
    }
);

#[test]
fn test_os_name() {
    #[cfg(target_os = "windows")]
    assert_eq!(os_name(), "W");

    #[cfg(target_os = "linux")]
    assert_eq!(os_name(), "L");

    #[cfg(all(not(target_os = "windows"), not(target_os = "linux")))]
    assert_eq!(os_name(), "U");
}
