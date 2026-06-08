//! Holds the [`Length`] primitive.

use core::{convert::Infallible, fmt::Display};

use osom_lib_reprc::macros::reprc;
use osom_lib_try_clone::TryClone;

/// Represents length internally used by osom tools. Unlike Rust `usize`
/// type, the [`Length`] type is a thin wrapper around `u32`. In particular
/// it is 32-bit on 64-bit machines. While limiting, the osom libs won't
/// be using such big arrays anyway. And it saves us space.
#[reprc]
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
#[must_use]
pub struct Length {
    size: u32,
}

/// Represents possible [`Length`] errors.
#[reprc]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LengthError {
    /// [`Length`] is negative.
    Negative = 0,

    /// [`Length`] is bigger than [`Length::MAX_LENGTH`].
    OutOfMaxRange = 1,
}

impl Display for LengthError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            LengthError::Negative => write!(f, "LengthError::Negative"),
            LengthError::OutOfMaxRange => write!(f, "LengthError::OutOfMaxRange"),
        }
    }
}

impl Length {
    /// The amount of bytes one can store safely in a buffer,
    /// without exceeding `u32` range. In other words [`Length::MAX_LENGTH`] plus
    /// [`Length::SAFE_MARGIN`] is guaranteed to never exceed [`u32::MAX`].
    ///
    /// This can be useful if one wants to store some additional data in a buffer whose
    /// length is represented by [`Length`].
    pub const SAFE_MARGIN: u32 = 2048;

    /// The maximal value [`Length`] can take. Basically this is
    /// `i32::MAX - Self::SAFE_MARGIN`.
    pub const MAX_LENGTH: Self = const {
        const I32_MAX: u32 = i32::MAX as u32;
        assert!(Self::SAFE_MARGIN >= 64, "SAFE_MARGIN has to be at least 64.");
        assert!(
            Self::SAFE_MARGIN < I32_MAX,
            "SAFE_MARGIN has to be smaller than i32::MAX"
        );
        unsafe { Self::new_unchecked(I32_MAX - Self::SAFE_MARGIN) }
    };

    /// Represents [`Length`] zero.
    pub const ZERO: Self = unsafe { Self::new_unchecked(0) };

    /// Represents [`Length`] one.
    pub const ONE: Self = unsafe { Self::new_unchecked(1) };

    /// Creates a new [`Length`] out of `u32`.
    ///
    /// # Safety
    ///
    /// This function does not validate `value`. It is up to the
    /// caller to ensure that its value is below or equal to
    /// [`Length::MAX_LENGTH`].
    #[inline(always)]
    pub const unsafe fn new_unchecked(value: u32) -> Self {
        Self { size: value }
    }

    /// Creates a new [`Length`] from `u32`.
    ///
    /// # Errors
    ///
    /// For details see [`LengthError`].
    #[inline(always)]
    pub const fn try_from_u32(value: u32) -> Result<Self, LengthError> {
        if value > Self::MAX_LENGTH.as_u32() {
            Err(LengthError::OutOfMaxRange)
        } else {
            Ok(unsafe { Self::new_unchecked(value) })
        }
    }

    /// Creates a new [`Length`] from `usize`.
    ///
    /// # Errors
    ///
    /// For details see [`LengthError`].
    #[inline(always)]
    pub const fn try_from_usize(value: usize) -> Result<Self, LengthError> {
        #[allow(clippy::cast_possible_truncation)]
        if value > Self::MAX_LENGTH.as_usize() {
            Err(LengthError::OutOfMaxRange)
        } else {
            Ok(unsafe { Self::new_unchecked(value as u32) })
        }
    }

    /// Creates a new [`Length`] from `i32`.
    ///
    /// # Errors
    ///
    /// For details see [`LengthError`].
    #[inline(always)]
    pub const fn try_from_i32(value: i32) -> Result<Self, LengthError> {
        if value < 0 {
            return Err(LengthError::Negative);
        }

        #[allow(clippy::cast_sign_loss)]
        let value = value as u32;

        Self::try_from_u32(value)
    }

    /// Turns the [`Length`] into `u32`.
    #[inline(always)]
    #[must_use]
    pub const fn as_u32(&self) -> u32 {
        self.size
    }

    /// Turns the [`Length`] into `usize`.
    #[inline(always)]
    #[must_use]
    pub const fn as_usize(&self) -> usize {
        self.size as usize
    }
}

impl TryClone for Length {
    type Error = Infallible;
    fn try_clone(&self) -> Result<Self, Self::Error> {
        Ok(*self)
    }
}

impl TryFrom<i32> for Length {
    type Error = LengthError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::try_from_i32(value)
    }
}

impl TryFrom<u32> for Length {
    type Error = LengthError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from_u32(value)
    }
}

impl TryFrom<usize> for Length {
    type Error = LengthError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::try_from_usize(value)
    }
}

impl From<Length> for u32 {
    fn from(value: Length) -> Self {
        value.as_u32()
    }
}

impl From<Length> for usize {
    fn from(value: Length) -> Self {
        value.as_usize()
    }
}

impl core::fmt::Display for Length {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.as_u32().fmt(f)
    }
}

const _: () = const {
    assert!(size_of::<Length>() == 4, "Length is expected to be of size 4");
};
