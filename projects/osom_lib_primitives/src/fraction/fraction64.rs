use super::FractionError;

/// Represents a fraction value.
///
/// This is an `f64` value under the hood, which is in `0.0 .. 1.0`
/// range.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[repr(transparent)]
#[must_use]
pub struct Fraction64 {
    value: f64,
}

impl Fraction64 {
    /// Creates a new [`Fraction64`] instance out of passed `value`.
    ///
    /// # Safety
    ///
    /// This function does not verify that `value` is in the expected
    /// `0.0 .. 1.0` range.
    #[inline(always)]
    pub const unsafe fn new_unchecked(value: f64) -> Self {
        Self { value }
    }

    /// Creates a new [`Fraction64`] instance out of passed value.
    ///
    /// # Errors
    ///
    /// Return [`FractionError::NotAFraction`] if the passed `value`
    /// is outside of `0.0 .. 1.0` range.
    #[inline(always)]
    pub const fn new(value: f64) -> Result<Self, FractionError> {
        if value >= 0.0 && value < 1.0 {
            Ok(unsafe { Self::new_unchecked(value) })
        } else {
            Err(FractionError::NotAFraction)
        }
    }

    /// Returns the underlying `f64` value.
    #[inline(always)]
    #[must_use]
    pub const fn value(&self) -> f64 {
        self.value
    }
}
