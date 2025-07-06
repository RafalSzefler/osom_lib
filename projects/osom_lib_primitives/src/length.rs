#![allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::cast_sign_loss)]

/// Represents length used by `osom_tools`.
///
/// # Notes
/// Unlike `usize` our [`Length`] has `i32` as the underlying type.
/// Thus it can handle a lot less items than `usize` on 64-bit platforms.
///
/// But it takes less space in memory, which ultimately is more useful.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
#[must_use]
pub struct Length {
    value: i32,
}

impl core::fmt::Debug for Length {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.value())
    }
}

/// Represents errors when building new [`Length`] instance.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LengthError {
    /// The length exceeds [`Length::MAX`].
    TooLarge,

    /// The length is negative.
    Negative,
}

impl Length {
    pub const MAX: usize = (i32::MAX - 1024) as usize;
    pub const ZERO: Self = unsafe { Self::new_unchecked(0) };

    /// Tries to convert a `usize` to a [`Length`].
    ///
    /// # Errors
    ///
    /// For details see [`LengthError`].
    #[inline]
    pub const fn try_from_usize(len: usize) -> Result<Self, LengthError> {
        if len > Self::MAX {
            return Err(LengthError::TooLarge);
        }

        Ok(unsafe { Self::new_unchecked(len as i32) })
    }

    /// Tries to convert a `i32` to a [`Length`].
    ///
    /// # Errors
    ///
    /// For details see [`LengthError`].
    #[inline]
    pub const fn try_from_i32(len: i32) -> Result<Self, LengthError> {
        if len < 0 {
            return Err(LengthError::Negative);
        }

        if len as usize > Self::MAX {
            return Err(LengthError::TooLarge);
        }

        Ok(unsafe { Self::new_unchecked(len) })
    }

    /// Creates a new [`Length`] from a `i32`.
    ///
    /// # Safety
    ///
    /// The `len` must be non-negative otherwise the behavior is undefined.
    #[inline(always)]
    pub const unsafe fn new_unchecked(len: i32) -> Self {
        Self { value: len }
    }

    /// Returns the value of the [`Length`].
    #[inline(always)]
    #[must_use]
    pub const fn value(&self) -> i32 {
        unsafe { core::hint::assert_unchecked(self.value >= 0) };
        self.value
    }

    /// Increments the value of the [`Length`] by the given `value`.
    ///
    /// # Errors
    ///
    /// For details see [`LengthError`].
    #[inline]
    pub const fn add(&mut self, value: i32) -> Result<(), LengthError> {
        let new_value = self.value + value;
        if new_value < 0 {
            return Err(LengthError::Negative);
        }

        if new_value > Self::MAX as i32 {
            return Err(LengthError::TooLarge);
        }

        self.value = new_value;
        Ok(())
    }

    /// Multiplies the value of the [`Length`] by the given `value`.
    ///
    /// # Errors
    ///
    /// For details see [`LengthError`].
    #[inline]
    pub const fn mul(&mut self, value: i32) -> Result<(), LengthError> {
        let new_value = self.value * value;
        if new_value < 0 {
            return Err(LengthError::Negative);
        }

        if new_value > Self::MAX as i32 {
            return Err(LengthError::TooLarge);
        }

        self.value = new_value;
        Ok(())
    }
}

impl TryFrom<usize> for Length {
    type Error = LengthError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::try_from_usize(value)
    }
}

impl TryFrom<i32> for Length {
    type Error = LengthError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::try_from_i32(value)
    }
}

impl From<Length> for usize {
    fn from(value: Length) -> Self {
        value.value() as usize
    }
}

impl From<Length> for i32 {
    fn from(value: Length) -> Self {
        value.value()
    }
}

impl From<Length> for u32 {
    fn from(value: Length) -> Self {
        value.value() as u32
    }
}

impl core::ops::Add<i32> for Length {
    type Output = Self;

    fn add(mut self, rhs: i32) -> Self::Output {
        Self::add(&mut self, rhs).unwrap();
        self
    }
}

impl core::ops::AddAssign<i32> for Length {
    fn add_assign(&mut self, rhs: i32) {
        Self::add(self, rhs).unwrap();
    }
}

impl core::ops::Add<Length> for Length {
    type Output = Self;

    fn add(self, rhs: Length) -> Self::Output {
        self.add(rhs.value())
    }
}

impl core::ops::AddAssign<Length> for Length {
    fn add_assign(&mut self, rhs: Length) {
        self.add(rhs.value()).unwrap();
    }
}

impl core::ops::Sub<i32> for Length {
    type Output = Self;

    fn sub(mut self, rhs: i32) -> Self::Output {
        self.value -= rhs;
        self
    }
}

impl core::ops::SubAssign<i32> for Length {
    fn sub_assign(&mut self, rhs: i32) {
        self.value -= rhs;
    }
}

impl core::ops::Sub<Length> for Length {
    type Output = Self;

    fn sub(self, rhs: Length) -> Self::Output {
        self + (-rhs.value())
    }
}

impl core::ops::SubAssign<Length> for Length {
    fn sub_assign(&mut self, rhs: Length) {
        self.add(-rhs.value()).unwrap();
    }
}

impl core::ops::Mul<i32> for Length {
    type Output = Self;

    fn mul(mut self, rhs: i32) -> Self::Output {
        Self::mul(&mut self, rhs).unwrap();
        self
    }
}

impl core::ops::MulAssign<i32> for Length {
    fn mul_assign(&mut self, rhs: i32) {
        Self::mul(self, rhs).unwrap();
    }
}

impl AsRef<i32> for Length {
    fn as_ref(&self) -> &i32 {
        &self.value
    }
}

impl AsMut<i32> for Length {
    fn as_mut(&mut self) -> &mut i32 {
        &mut self.value
    }
}
