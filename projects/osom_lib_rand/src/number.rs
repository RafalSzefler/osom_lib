//! The module contains the definition of the [`Number`] trait
//! and its implementations for `u32`, `u64` and `u128`.

use core::fmt::{Debug, Display};
use core::hash::Hash;
use core::mem::size_of;

trait Private {}

/// Marker trait that abstracts the following numerical types:
/// `u32`, `u64` and `u128`.
/// 
/// # Notes
/// 
/// This trait depends on private trait, and thus extending it
/// is not possible.
#[allow(private_bounds)]
pub trait Number:
    Clone + Copy + Debug + Display + PartialEq + Eq + Hash + PartialOrd + Ord + Default + Private
{
    /// Represents the associated byte representation of the number, e.g. `[u8; 4]` for `u32`.
    type ByteRepr: AsRef<[u8]> + AsMut<[u8]>;

    /// Returns the size of the number in bytes.
    #[must_use]
    fn size() -> usize {
        size_of::<Self>()
    }

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

    /// Returns the number one.
    #[must_use]
    fn one() -> Self;

    /// Returns the number zero.
    #[must_use]
    fn zero() -> Self {
        Self::default()
    }

    /// Returns the highest possible value of the number.
    #[must_use]
    fn highest() -> Self;

    /// Returns the number as a `u128`, which is guaranteed to be big enough to hold
    /// any [`Number`] value.
    #[must_use]
    fn as_u128(self) -> u128;

    /// Creates a number from its little-endian representation in bytes.
    #[must_use]
    fn from_le_bytes(bytes: &[u8]) -> Self;

    /// Returns the little-endian representation of the number in bytes.
    #[must_use]
    fn to_le_bytes(self) -> Self::ByteRepr;
}

impl Private for u32 {}
impl Private for u64 {}
impl Private for u128 {}

impl Number for u32 {
    type ByteRepr = [u8; 4];

    fn one() -> Self {
        1
    }
    fn highest() -> Self {
        u32::MAX
    }
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
    fn from_le_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() >= size_of::<Self>());
        let mut result = [0u8; size_of::<Self>()];
        result.copy_from_slice(bytes);
        Self::from_le_bytes(result)
    }
    fn to_le_bytes(self) -> Self::ByteRepr {
        self.to_le_bytes()
    }
}

impl Number for u64 {
    type ByteRepr = [u8; 8];

    fn one() -> Self {
        1
    }
    fn highest() -> Self {
        u64::MAX
    }
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
    fn from_le_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() >= size_of::<Self>());
        let mut result = [0u8; size_of::<Self>()];
        result.copy_from_slice(bytes);
        Self::from_le_bytes(result)
    }
    fn to_le_bytes(self) -> Self::ByteRepr {
        self.to_le_bytes()
    }
}

impl Number for u128 {
    type ByteRepr = [u8; 16];

    fn one() -> Self {
        1
    }
    fn highest() -> Self {
        u128::MAX
    }
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
    fn from_le_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() >= size_of::<Self>());
        let mut result = [0u8; size_of::<Self>()];
        result.copy_from_slice(bytes);
        Self::from_le_bytes(result)
    }
    fn to_le_bytes(self) -> Self::ByteRepr {
        self.to_le_bytes()
    }
}

/// Holds the maximum size of all number types.
pub const MAX_NUMBER_SIZE: usize = const {
    let result = size_of::<u128>();
    assert!(result >= size_of::<u64>());
    assert!(result >= size_of::<u32>());
    result
};
