//! Holds the [`Offset`] primitive.

#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_possible_wrap)]
use osom_lib_reprc::macros::reprc;

use crate::length::Length;

/// Represents offset internally used by osom tools. This is similar
/// to [`Length`][`crate::length::Length`], and internally is represented
/// as a 32-bit signed integer. The point is that [`Offset`] can be added
/// and removed from [`Length`][`crate::length::Length`].
#[reprc]
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
#[must_use]
pub struct Offset {
    value: i32,
}

/// Represents possible [`Offset`] errors.
#[reprc]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OffsetError {
    /// [`Offset`] is bigger than [`Offset::MAX_OFFSET`].
    AboveMaxRange = 0,

    /// [`Offset`] is smaller than [`Offset::MIN_OFFSET`].
    BelowMinRange = 1,
}

impl Offset {
    const SAFE_MARGIN: i32 = const {
        assert!(Length::SAFE_MARGIN < i32::MAX as u32);
        Length::SAFE_MARGIN as i32
    };

    /// The maximal value [`Offset`] can take.
    pub const MAX_OFFSET: Offset = const {
        assert!(Self::SAFE_MARGIN >= 64, "SAFE_MARGIN has to be at least 64.");
        assert!(
            Self::SAFE_MARGIN < i32::MAX,
            "SAFE_MARGIN has to be smaller than i32::MAX"
        );
        unsafe { Offset::new_unchecked(i32::MAX - Self::SAFE_MARGIN) }
    };

    /// The minimal value [`Offset`] can take.
    pub const MIN_OFFSET: Offset = const {
        assert!(Self::SAFE_MARGIN >= 64, "SAFE_MARGIN has to be at least 64.");
        assert!(
            Self::SAFE_MARGIN < i32::MAX,
            "SAFE_MARGIN has to be smaller than i32::MAX"
        );
        unsafe { Offset::new_unchecked(i32::MIN + Self::SAFE_MARGIN) }
    };

    /// Represents [`Offset`] zero.
    pub const ZERO: Self = unsafe { Self::new_unchecked(0) };

    /// Represents [`Offset`] one.
    pub const ONE: Self = unsafe { Self::new_unchecked(1) };

    /// Represents [`Offset`] minus one.
    pub const MINUS_ONE: Self = unsafe { Self::new_unchecked(-1) };

    /// Creates a new [`Offset`] out of `i32`.
    ///
    /// # Safety
    ///
    /// This function does not validate `value`. It is up to the
    /// caller to ensure that its value is between [`Offset::MIN_OFFSET`]
    /// and [`Offset::MAX_OFFSET`].
    #[inline(always)]
    pub const unsafe fn new_unchecked(value: i32) -> Self {
        Self { value }
    }

    /// Creates a new [`Offset`] from `i32`.
    ///
    /// # Errors
    ///
    /// For details see [`OffsetError`].
    #[inline(always)]
    pub const fn try_from_i32(value: i32) -> Result<Self, OffsetError> {
        if value < Self::MIN_OFFSET.as_i32() {
            Err(OffsetError::BelowMinRange)
        } else if value > Self::MAX_OFFSET.as_i32() {
            Err(OffsetError::AboveMaxRange)
        } else {
            Ok(unsafe { Self::new_unchecked(value) })
        }
    }

    /// Creates a new [`Offset`] from `u32`.
    ///
    /// # Errors
    ///
    /// For details see [`OffsetError`].
    #[inline(always)]
    pub const fn try_from_u32(value: u32) -> Result<Self, OffsetError> {
        if value > Self::MAX_OFFSET.as_i32() as u32 {
            return Err(OffsetError::AboveMaxRange);
        }

        let value = value as i32;

        Self::try_from_i32(value)
    }

    /// Turns the [`Offset`] into `i32`.
    #[inline(always)]
    #[must_use]
    pub const fn as_i32(&self) -> i32 {
        self.value
    }

    /// Turns the [`Offset`] into `isize`.
    #[inline(always)]
    #[must_use]
    pub const fn as_isize(&self) -> isize {
        self.value as isize
    }
}

impl TryFrom<i32> for Offset {
    type Error = OffsetError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::try_from_i32(value)
    }
}

impl TryFrom<u32> for Offset {
    type Error = OffsetError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from_u32(value)
    }
}

impl TryFrom<isize> for Offset {
    type Error = OffsetError;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value > Offset::MAX_OFFSET.as_isize() {
            Err(OffsetError::AboveMaxRange)
        } else if value < Offset::MIN_OFFSET.as_isize() {
            Err(OffsetError::BelowMinRange)
        } else {
            let value = value as i32;

            Ok(unsafe { Self::new_unchecked(value) })
        }
    }
}

impl From<Offset> for i32 {
    fn from(value: Offset) -> Self {
        value.as_i32()
    }
}

impl From<Offset> for isize {
    fn from(value: Offset) -> Self {
        value.as_isize()
    }
}

impl core::fmt::Display for Offset {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.as_i32().fmt(f)
    }
}

const _: () = const {
    assert!(
        size_of::<Offset>() == size_of::<Length>(),
        "Offset and length have to be of the same size"
    );
    assert!(
        align_of::<Offset>() == align_of::<Length>(),
        "Offset and length have to have the same alignment"
    );
};
