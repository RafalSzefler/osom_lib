use core::borrow::Borrow;

use osom_lib_reprc::macros::reprc;
use osom_lib_try_clone::TryClone;

use crate::errors::TryFromCVRIntError;

/// Represents an integer value. Internally represented by `i128` value.
#[reprc]
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[must_use]
pub struct CVRInt {
    value: i128,
}

impl CVRInt {
    /// Creates a new [`CVRInt`] instance.
    pub const fn new(value: i128) -> Self {
        Self { value }
    }

    /// Returns the underlying `i128` value.
    #[inline(always)]
    #[must_use]
    pub const fn inner(self) -> i128 {
        self.value
    }
}

impl AsRef<i128> for CVRInt {
    fn as_ref(&self) -> &i128 {
        &self.value
    }
}

impl Borrow<i128> for CVRInt {
    fn borrow(&self) -> &i128 {
        &self.value
    }
}

impl From<i128> for CVRInt {
    fn from(value: i128) -> Self {
        Self::new(value)
    }
}

impl From<CVRInt> for i128 {
    fn from(cvr: CVRInt) -> Self {
        cvr.value
    }
}

impl From<i32> for CVRInt {
    fn from(value: i32) -> Self {
        Self::new(i128::from(value))
    }
}

impl From<i64> for CVRInt {
    fn from(value: i64) -> Self {
        Self::new(i128::from(value))
    }
}

impl From<u32> for CVRInt {
    fn from(value: u32) -> Self {
        Self::new(i128::from(value))
    }
}

impl From<u64> for CVRInt {
    fn from(value: u64) -> Self {
        Self::new(i128::from(value))
    }
}

impl TryFrom<CVRInt> for i32 {
    type Error = TryFromCVRIntError;

    fn try_from(cvr: CVRInt) -> Result<Self, Self::Error> {
        Ok(i32::try_from(cvr.value)?)
    }
}

impl TryFrom<CVRInt> for i64 {
    type Error = TryFromCVRIntError;

    fn try_from(cvr: CVRInt) -> Result<Self, Self::Error> {
        Ok(i64::try_from(cvr.value)?)
    }
}

impl TryFrom<CVRInt> for u32 {
    type Error = TryFromCVRIntError;

    fn try_from(cvr: CVRInt) -> Result<Self, Self::Error> {
        Ok(u32::try_from(cvr.value)?)
    }
}

impl TryFrom<CVRInt> for u64 {
    type Error = TryFromCVRIntError;

    fn try_from(cvr: CVRInt) -> Result<Self, Self::Error> {
        Ok(u64::try_from(cvr.value)?)
    }
}

osom_lib_macros::unreachable_from_infallible!(CVRInt);

impl core::fmt::Display for CVRInt {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl TryClone for CVRInt {
    type Error = core::convert::Infallible;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        Ok(*self)
    }
}
