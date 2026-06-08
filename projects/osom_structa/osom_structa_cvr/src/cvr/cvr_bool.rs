use core::borrow::Borrow;

use osom_lib_reprc::macros::reprc;
use osom_lib_try_clone::TryClone;

/// Represents a boolean value. Internally it is a simple `bool` value.
#[reprc]
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[must_use]
pub struct CVRBool {
    value: bool,
}

impl CVRBool {
    /// Creates a new [`CVRBool`] instance.
    pub const fn new(value: bool) -> Self {
        Self { value }
    }

    #[inline(always)]
    #[must_use]
    pub const fn inner(self) -> bool {
        self.value
    }
}

impl AsRef<bool> for CVRBool {
    fn as_ref(&self) -> &bool {
        &self.value
    }
}

impl From<bool> for CVRBool {
    fn from(value: bool) -> Self {
        Self::new(value)
    }
}

impl From<CVRBool> for bool {
    fn from(cvr: CVRBool) -> Self {
        cvr.value
    }
}

impl Borrow<bool> for CVRBool {
    fn borrow(&self) -> &bool {
        &self.value
    }
}

osom_lib_macros::unreachable_from_infallible!(CVRBool);

impl core::fmt::Display for CVRBool {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl TryClone for CVRBool {
    type Error = core::convert::Infallible;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        Ok(*self)
    }
}
