//! This module verifies basic properties of the primitives.
//!
//! While most of them are guaranteed by the Rust compiler,
//! it won't hurt to double check them.
#![allow(unused)]

const _CHECKS: () = const {
    verify_size_and_alignment_of_primitives();
    verify_integers_layout();
};

#[rustfmt::skip]
const fn verify_size_and_alignment_of_primitives() {
    if cfg!(target_pointer_width = "32") {
        assert!(align_of::<usize>() == align_of::<u32>(), "usize and u32 are expected to have the same alignment on 32-bit target");
        assert!(size_of::<usize>() == size_of::<u32>(), "usize and u32 are expected to have the same size on 32-bit target");
    } else if cfg!(target_pointer_width = "64") {
        assert!(align_of::<usize>() == align_of::<u64>(), "usize and u64 are expected to have the same alignment on 64-bit target");
        assert!(size_of::<usize>() == size_of::<u64>(), "usize and u64 are expected to have the same size on 64-bit target");
    } else {
        panic!("Unsupported target pointer width, expected 32 or 64 bit target.");
    }

    assert!(align_of::<u8>() == 1, "u8 is expected to be of alignment 1");
    assert!(size_of::<u8>() == 1, "u8 is expected to be of size 1");
    assert!(align_of::<u16>() == 2, "u16 is expected to be of alignment 2");
    assert!(size_of::<u16>() == 2, "u16 is expected to be of size 2");
    assert!(align_of::<u32>() == 4, "u32 is expected to be of alignment 4");
    assert!(size_of::<u32>() == 4, "u32 is expected to be of size 4");
    assert!(align_of::<u64>() == 4 || align_of::<u64>() == 8, "u64 is expected to be of alignment 4 or 8");
    assert!(size_of::<u64>() == 8, "u64 is expected to be of size 8");
    assert!(align_of::<u128>() == 4 || align_of::<u128>() == 8 || align_of::<u128>() == 16, "u128 is expected to be of alignment 4 or 8 or 16");
    assert!(size_of::<u128>() == 16, "u128 is expected to be of size 16");
    assert!(align_of::<usize>() == align_of::<isize>(), "usize and isize are expected to have the same alignment");
    assert!(size_of::<usize>() == size_of::<isize>(), "usize and isize are expected to have the same size");
}

#[rustfmt::skip]
const fn verify_integers_layout() {
    assert!(eq(&(-1i8).to_le_bytes(), &[0xFF]), "Invalid layout for -1i8");
    assert!(eq(&(-1i16).to_le_bytes(), &[0xFF, 0xFF]), "Invalid layout for -1i16");
    assert!(eq(&(-1i32).to_le_bytes(), &[0xFF, 0xFF, 0xFF, 0xFF]), "Invalid layout for -1i32");
    assert!(eq(&(-1i64).to_le_bytes(), &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]), "Invalid layout for -1i64");
    assert!(eq(&(-1i128).to_le_bytes(), &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]), "Invalid layout for -1i128");

    assert!(eq(&(0i8).to_le_bytes(), &[0x00]), "Invalid layout for 0i8");
    assert!(eq(&(0i16).to_le_bytes(), &[0x00, 0x00]), "Invalid layout for 0i16");
    assert!(eq(&(0i32).to_le_bytes(), &[0x00, 0x00, 0x00, 0x00]), "Invalid layout for 0i32");
    assert!(eq(&(0i64).to_le_bytes(), &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]), "Invalid layout for 0i64");
    assert!(eq(&(0i128).to_le_bytes(), &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]), "Invalid layout for 0i128");

    assert!(eq(&(-3i8).to_le_bytes(), &[0xFD]), "Invalid layout for -3i8");
    assert!(eq(&(-4i16).to_le_bytes(), &[0xFC, 0xFF]), "Invalid layout for -4i16");
    assert!(eq(&(-5i32).to_le_bytes(), &[0xFB, 0xFF, 0xFF, 0xFF]), "Invalid layout for -5i32");
    assert!(eq(&(-6i64).to_le_bytes(), &[0xFA, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]), "Invalid layout for -6i64");
    assert!(eq(&(-7i128).to_le_bytes(), &[0xF9, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]), "Invalid layout for -7i128");
}

const fn eq<const SIZE: usize>(a: &[u8; SIZE], b: &[u8; SIZE]) -> bool {
    let mut idx = 0;
    while idx < SIZE {
        if a[idx] != b[idx] {
            return false;
        }
        idx += 1;
    }
    true
}
