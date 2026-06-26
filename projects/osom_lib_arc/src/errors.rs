//! Holds the definition of [`CArcError`].
use core::fmt::Display;

use osom_lib_reprc::macros::reprc;

/// Represents possible errors when working with [`CArc`][super::carc::CArc].
#[reprc]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[must_use]
pub enum CArcError {
    /// The underlying allocator returned an error.
    AllocationError = 0,
}

osom_lib_macros::unreachable_from_infallible!(CArcError);

impl Display for CArcError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            CArcError::AllocationError => write!(f, "CArcError::AllocationError"),
        }
    }
}

/// Represents possible errors when working with [`CArc`][super::carc::CArc].
#[reprc]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[must_use]
pub enum CArcArrayError {
    /// The underlying allocator returned an error.
    AllocationError = 0,

    /// The array size is out of range.
    ArraySizeOutOfRange = 1,

    /// Tried to clone internal item in the array, but cloning failed.
    ItemCloningError = 2,
}

osom_lib_macros::unreachable_from_infallible!(CArcArrayError);

impl Display for CArcArrayError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            CArcArrayError::AllocationError => write!(f, "CArcArrayError::AllocationError"),
            CArcArrayError::ArraySizeOutOfRange => write!(f, "CArcArrayError::ArraySizeOutOfRange"),
            CArcArrayError::ItemCloningError => write!(f, "CArcArrayError::ItemCloningError"),
        }
    }
}

/// Represents an error that occurs when the maximum number of references is exceeded.
#[reprc]
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[must_use]
pub struct MaxReferencesExceededError;

osom_lib_macros::unreachable_from_infallible!(MaxReferencesExceededError);

impl Display for MaxReferencesExceededError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "MaxReferencesExceededError")
    }
}

/// Represents possible errors when upgrading a weak reference to a strong reference.
#[reprc]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[must_use]
pub enum WeakUpgradeError {
    /// The maximum number of references is exceeded.
    MaxReferencesExceeded = 0,

    /// There are no strong references alive.
    NoStrongReferencesAlive = 1,
}
