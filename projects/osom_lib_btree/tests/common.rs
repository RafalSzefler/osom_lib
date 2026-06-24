#![allow(unused)]
use osom_lib_primitives::length::Length;

/// Generates a new psuedo randomly generated value
/// using the linear congruential generator algorithm.
pub fn lcg_next(current: u64) -> u64 {
    const A: u64 = 0x5851F42D4C957F2D;
    const C: u64 = 0x14057B7EF767814F;
    current.wrapping_mul(A).wrapping_add(C)
}

/// Creates a new [`Length`] from `usize`.
/// Straightforward `.unwrap()`.
pub fn make_len(value: usize) -> Length {
    Length::try_from_usize(value).unwrap()
}
