use osom_lib_primitives::kvp::KVP;

#[test]
fn test_kvp_unpack() {
    let kvp = KVP { key: 1, value: 2 };
    let (key, value) = kvp.unpack();
    assert_eq!(key, 1);
    assert_eq!(value, 2);
}

#[test]
fn test_kvp_unpack_ptr() {
    let mut kvp = KVP { key: 1, value: 2 };
    let unpacked = unsafe { KVP::unpack_ptr(&raw mut kvp) };
    unsafe {
        let first = unpacked.0.as_ref_unchecked();
        let second = unpacked.1.as_mut_unchecked();
        assert_eq!(*first, 1);
        assert_eq!(*second, 2);
        *second = 3;
    }

    assert_eq!(kvp.value, 3);
}
