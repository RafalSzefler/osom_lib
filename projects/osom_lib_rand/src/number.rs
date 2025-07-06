use core::fmt::{Debug, Display};
use core::hash::Hash;
use core::mem::size_of;

trait Private {}

/// Marker trait that abstracts the following numerical types:
/// `u32`, `u64` and `u128`.
#[allow(private_bounds)]
pub trait Number:
    Clone + Copy + Debug + Display + PartialEq + Eq + Hash + PartialOrd + Ord + Default + Private
{
    #[must_use]
    fn size() -> usize;
    #[must_use]
    fn wrapping_add(self, other: Self) -> Self;
    #[must_use]
    fn wrapping_sub(self, other: Self) -> Self;
    #[must_use]
    fn wrapping_mul(self, other: Self) -> Self;
    #[must_use]
    fn wrapping_div(self, other: Self) -> Self;
    #[must_use]
    fn wrapping_rem(self, other: Self) -> Self;
    #[must_use]
    fn wrapping_shl(self, other: u32) -> Self;
    #[must_use]
    fn wrapping_shr(self, other: u32) -> Self;
    #[must_use]
    fn one() -> Self;
    #[must_use]
    fn zero() -> Self {
        Self::default()
    }
    #[must_use]
    fn highest() -> Self;
    #[must_use]
    fn as_u128(self) -> u128;
    #[must_use]
    fn from_le_bytes(bytes: &[u8]) -> Self;
}

impl Private for u32 {}
impl Private for u64 {}
impl Private for u128 {}

impl Number for u32 {
    fn size() -> usize {
        size_of::<Self>()
    }
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
}

impl Number for u64 {
    fn size() -> usize {
        size_of::<Self>()
    }
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
}

impl Number for u128 {
    fn size() -> usize {
        size_of::<Self>()
    }
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
}

/// Holds the maximum size of all number types.
pub const MAX_NUMBER_SIZE: usize = const {
    let result = size_of::<u128>();
    assert!(result >= size_of::<u64>());
    assert!(result >= size_of::<u32>());
    result
};
