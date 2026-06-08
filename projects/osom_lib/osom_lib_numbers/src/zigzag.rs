//! Holds helpers for encoding and decoding numbers using the zigzag encoding.
//!
//! The zigzag encoding converts signed integers to unsigned integers and vice versa.
//!
//! * Positive signed integers are multiplied by `2`.
//! * Negative signed integers are multiplied by `-2` and `1` is subtracted.
//! * The final result is then converted to an unsigned integer.

/// Encodes a 32-bit signed integer to a 32-bit unsigned integer using the zigzag encoding.
#[inline]
#[must_use]
pub const fn zigzag_encode32(value: i32) -> u32 {
    ((value << 1) ^ (value >> 31)).cast_unsigned()
}

/// Decodes a 32-bit unsigned integer to a 32-bit signed integer using the zigzag encoding.
#[inline]
#[must_use]
pub const fn zigzag_decode32(value: u32) -> i32 {
    (value >> 1).cast_signed() ^ -(value & 1).cast_signed()
}

/// Encodes a 64-bit signed integer to a 64-bit unsigned integer using the zigzag encoding.
#[inline]
#[must_use]
pub const fn zigzag_encode64(value: i64) -> u64 {
    ((value << 1) ^ (value >> 63)).cast_unsigned()
}

/// Decodes a 64-bit unsigned integer to a 64-bit signed integer using the zigzag encoding.
#[inline]
#[must_use]
pub const fn zigzag_decode64(value: u64) -> i64 {
    (value >> 1).cast_signed() ^ -(value & 1).cast_signed()
}

/// Encodes a 128-bit signed integer to a 128-bit unsigned integer using the zigzag encoding.
#[inline]
#[must_use]
pub const fn zigzag_encode128(value: i128) -> u128 {
    ((value << 1) ^ (value >> 127)).cast_unsigned()
}

/// Decodes a 128-bit unsigned integer to a 128-bit signed integer using the zigzag encoding.
#[inline]
#[must_use]
pub const fn zigzag_decode128(value: u128) -> i128 {
    (value >> 1).cast_signed() ^ -(value & 1).cast_signed()
}
