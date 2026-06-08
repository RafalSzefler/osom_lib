use core::convert::Infallible;

use osom_lib_reprc::macros::reprc;
use osom_lib_try_clone::TryClone;

use super::PowerOfTwoError;

/// Represents a power of two, as a 32-bit value, which includes zero.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[reprc]
#[repr(transparent)]
#[must_use]
pub struct PowerOfTwo32 {
    value: u32,
}

impl PowerOfTwo32 {
    /// Represents [`PowerOfTwo32`] zero.
    pub const ZERO: Self = unsafe { Self::new_unchecked(0) };

    /// Creates a new [`PowerOfTwo32`] from a 32-bit value.
    ///
    /// # Safety
    ///
    /// This function does not validate `value`. It is up to the
    /// caller to ensure that its value is a power of two.
    #[inline(always)]
    pub const unsafe fn new_unchecked(value: u32) -> Self {
        Self { value }
    }

    /// Creates a new [`PowerOfTwo32`] from a 32-bit value.
    ///
    /// # Errors
    ///
    /// Returns [`PowerOfTwoError::NotAPowerOfTwo`] if `value` is not a power of two.
    #[inline(always)]
    pub const fn new(value: u32) -> Result<Self, PowerOfTwoError> {
        if value == 0 || value.is_power_of_two() {
            Ok(unsafe { Self::new_unchecked(value) })
        } else {
            Err(PowerOfTwoError::NotAPowerOfTwo)
        }
    }

    /// Returns the next power of two.
    ///
    /// # Panics
    ///
    /// Panics in debug mode if the next power of two is greater than [`u32::MAX`].
    /// In release mode it wraps to zero.
    #[inline(always)]
    pub fn next(self) -> Self {
        debug_assert!(self.value < (u32::MAX >> 1));
        let result = core::hint::select_unpredictable(self.value == 0, 1, self.value << 1);
        Self { value: result }
    }

    /// Returns the underlying value of the [`PowerOfTwo32`].
    #[inline(always)]
    #[must_use]
    pub const fn value(self) -> u32 {
        self.value
    }

    /// Returns the underlying value as `usize`.
    ///
    /// # Panics
    ///
    /// Panics if `usize` is smaller than `u32`.
    #[inline(always)]
    #[must_use]
    pub const fn as_usize(self) -> usize {
        #[allow(clippy::cast_possible_truncation)]
        {
            assert!(size_of::<usize>() >= size_of::<u32>(), "usize is smaller than u32");
            self.value as usize
        }
    }
}

impl core::fmt::Display for PowerOfTwo32 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.value.fmt(f)
    }
}

impl TryClone for PowerOfTwo32 {
    type Error = Infallible;
    fn try_clone(&self) -> Result<Self, Self::Error> {
        Ok(*self)
    }
}
