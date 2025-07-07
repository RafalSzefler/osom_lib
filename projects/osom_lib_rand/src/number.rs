//! The module contains the definition of the [`Number`] trait
//! and its implementations for `u32`, `u64` and `u128`.
#![allow(clippy::cast_possible_truncation)]

use core::fmt::{Debug, Display};
use core::hash::Hash;
use core::mem::size_of;

trait Private {}

/// Represents all the number types that [`Number`] trait
/// is implemented for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NumberType {
    U32,
    U64,
    U128,
}

/// Marker trait that abstracts the following numerical types:
/// `u32`, `u64` and `u128`.
///
/// # Notes
///
/// This trait depends on private trait, and thus extending it
/// is not possible.
#[allow(private_bounds)]
pub trait Number:
    'static + Clone + Copy + Debug + Display + PartialEq + Eq + Hash + PartialOrd + Ord + Default + Private
{
    /// Represents the associated byte representation of the number, e.g. `[u8; 4]` for `u32`.
    type ByteRepr: AsRef<[u8]> + AsMut<[u8]> + Default;

    /// The type of the number.
    const NUMBER_TYPE: NumberType;

    /// The size of the number in bytes.
    const SIZE: usize;

    /// The number zero.
    const ZERO: Self;

    /// The number one.
    const ONE: Self;

    /// The highest possible value of the number.
    const HIGHEST: Self;

    /// Returns the sum of two numbers, wrapping around if the result is too large.
    #[must_use]
    fn wrapping_add(self, other: Self) -> Self;

    /// Returns the difference of two numbers, wrapping around if the result is too small.
    #[must_use]
    fn wrapping_sub(self, other: Self) -> Self;

    /// Returns the product of two numbers, wrapping around if the result is too large.
    #[must_use]
    fn wrapping_mul(self, other: Self) -> Self;

    /// Returns the quotient of two numbers, wrapping around if the result is too large.
    #[must_use]
    fn wrapping_div(self, other: Self) -> Self;

    /// Returns the remainder of the division of two numbers.
    #[must_use]
    fn wrapping_rem(self, other: Self) -> Self;

    /// Returns the result of the left shift of the number by the given number of bits.
    #[must_use]
    fn wrapping_shl(self, other: u32) -> Self;

    /// Returns the result of the right shift of the number by the given number of bits.
    #[must_use]
    fn wrapping_shr(self, other: u32) -> Self;

    /// Returns the number as a `u128`, which is guaranteed to be big enough to hold
    /// any [`Number`] value.
    #[must_use]
    fn as_u128(self) -> u128;

    /// Creates a number from a `u32` value, which is guaranteed to be small enough
    /// to fit into the number.
    #[must_use]
    fn from_u32(value: u32) -> Self;

    /// Creates a number from its byte representation.
    ///
    /// # Notes
    ///
    /// This method is not portable, in particular it uses platform-specific
    /// endianness.
    #[must_use]
    fn from_bytes(bytes: &[u8]) -> Self;

    /// Returns the byte representation of the number.
    ///
    /// # Notes
    ///
    /// This method is not portable, in particular it uses platform-specific
    /// endianness.
    #[must_use]
    fn to_bytes(self) -> Self::ByteRepr;

    /// Creates a number from a `u32` value
    ///
    /// # Safety
    ///
    /// It does not check whether passed value is small enough to fit into `Self`.
    unsafe fn from_u64_unchecked(value: u64) -> Self;

    /// Creates a number from a `u128` value
    ///
    /// # Safety
    ///
    /// It does not check whether passed value is small enough to fit into `Self`.
    unsafe fn from_u128_unchecked(value: u128) -> Self;
}

impl Private for u32 {}
impl Private for u64 {}
impl Private for u128 {}

impl Number for u32 {
    type ByteRepr = [u8; 4];

    const NUMBER_TYPE: NumberType = NumberType::U32;
    const SIZE: usize = size_of::<Self>();
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const HIGHEST: Self = u32::MAX;

    fn wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }
    fn wrapping_sub(self, other: Self) -> Self {
        self.wrapping_sub(other)
    }
    fn wrapping_mul(self, other: Self) -> Self {
        self.wrapping_mul(other)
    }
    fn wrapping_div(self, other: Self) -> Self {
        self.wrapping_div(other)
    }
    fn wrapping_rem(self, other: Self) -> Self {
        self.wrapping_rem(other)
    }
    fn wrapping_shl(self, other: u32) -> Self {
        self.wrapping_shl(other)
    }
    fn wrapping_shr(self, other: u32) -> Self {
        self.wrapping_shr(other)
    }
    fn as_u128(self) -> u128 {
        u128::from(self)
    }

    #[inline(always)]
    fn from_u32(value: u32) -> Self {
        value
    }
    fn from_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() >= size_of::<Self>());
        let mut result = [0u8; size_of::<Self>()];
        result.copy_from_slice(bytes);
        Self::from_ne_bytes(result)
    }
    fn to_bytes(self) -> Self::ByteRepr {
        self.to_ne_bytes()
    }

    #[inline(always)]
    unsafe fn from_u64_unchecked(value: u64) -> Self {
        value as u32
    }

    #[inline(always)]
    unsafe fn from_u128_unchecked(value: u128) -> Self {
        value as u32
    }
}

impl Number for u64 {
    type ByteRepr = [u8; 8];

    const NUMBER_TYPE: NumberType = NumberType::U64;
    const SIZE: usize = size_of::<Self>();
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const HIGHEST: Self = u64::MAX;

    fn wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }
    fn wrapping_shl(self, other: u32) -> Self {
        self.wrapping_shl(other)
    }
    fn wrapping_shr(self, other: u32) -> Self {
        self.wrapping_shr(other)
    }
    fn wrapping_sub(self, other: Self) -> Self {
        self.wrapping_sub(other)
    }
    fn wrapping_mul(self, other: Self) -> Self {
        self.wrapping_mul(other)
    }
    fn wrapping_div(self, other: Self) -> Self {
        self.wrapping_div(other)
    }
    fn wrapping_rem(self, other: Self) -> Self {
        self.wrapping_rem(other)
    }
    fn as_u128(self) -> u128 {
        u128::from(self)
    }

    #[inline(always)]
    fn from_u32(value: u32) -> Self {
        Self::from(value)
    }
    fn from_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() >= size_of::<Self>());
        let mut result = [0u8; size_of::<Self>()];
        result.copy_from_slice(bytes);
        Self::from_ne_bytes(result)
    }
    fn to_bytes(self) -> Self::ByteRepr {
        self.to_ne_bytes()
    }

    #[inline(always)]
    unsafe fn from_u64_unchecked(value: u64) -> Self {
        value
    }

    #[inline(always)]
    unsafe fn from_u128_unchecked(value: u128) -> Self {
        value as u64
    }
}

impl Number for u128 {
    type ByteRepr = [u8; 16];

    const NUMBER_TYPE: NumberType = NumberType::U128;
    const SIZE: usize = size_of::<Self>();
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const HIGHEST: Self = u128::MAX;

    fn wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }
    fn wrapping_shl(self, other: u32) -> Self {
        self.wrapping_shl(other)
    }
    fn wrapping_shr(self, other: u32) -> Self {
        self.wrapping_shr(other)
    }
    fn wrapping_sub(self, other: Self) -> Self {
        self.wrapping_sub(other)
    }
    fn wrapping_mul(self, other: Self) -> Self {
        self.wrapping_mul(other)
    }
    fn wrapping_div(self, other: Self) -> Self {
        self.wrapping_div(other)
    }
    fn wrapping_rem(self, other: Self) -> Self {
        self.wrapping_rem(other)
    }
    fn as_u128(self) -> u128 {
        self
    }

    #[inline(always)]
    fn from_u32(value: u32) -> Self {
        Self::from(value)
    }
    fn from_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() >= size_of::<Self>());
        let mut result = [0u8; size_of::<Self>()];
        result.copy_from_slice(bytes);
        Self::from_le_bytes(result)
    }
    fn to_bytes(self) -> Self::ByteRepr {
        self.to_ne_bytes()
    }

    #[inline(always)]
    unsafe fn from_u64_unchecked(value: u64) -> Self {
        u128::from(value)
    }

    #[inline(always)]
    unsafe fn from_u128_unchecked(value: u128) -> Self {
        value
    }
}

/// Holds the maximum size of all number types.
pub const MAX_NUMBER_SIZE: usize = const {
    let result = size_of::<u128>();
    assert!(result >= size_of::<u64>());
    assert!(result >= size_of::<u32>());
    result
};
