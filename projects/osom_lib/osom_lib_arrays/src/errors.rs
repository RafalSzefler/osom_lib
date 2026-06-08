//! Holds definitions of various array errors.
use osom_lib_primitives::length::LengthError;
use osom_lib_reprc::macros::reprc;

/// Represents a general issue that can occure when dealing
/// with arrays.
#[reprc]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[must_use]
pub enum ArrayError {
    /// The underlying allocator returned an error,
    /// likely due to out of memory.
    AllocationError = 0,

    /// Tried to initialize an array or push to array beyond its internal limit.
    LengthLimitExceeded = 1,
}

impl core::fmt::Display for ArrayError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ArrayError::AllocationError => write!(f, "ArrayError::AllocationError"),
            ArrayError::LengthLimitExceeded => write!(f, "ArrayError::LengthLimitExceeded"),
        }
    }
}

impl From<LengthError> for ArrayError {
    fn from(_: LengthError) -> Self {
        Self::LengthLimitExceeded
    }
}

osom_lib_macros::unreachable_from_infallible!(ArrayError);

#[reprc]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[must_use]
pub enum ArrayTryCloneError {
    /// The underlying allocator returned an error,
    /// likely due to out of memory.
    ArrayError(ArrayError) = 0,

    /// Tried to clone internal item in the array, but cloning failed.
    ItemCloningError = 1,
}

osom_lib_macros::unreachable_from_infallible!(ArrayTryCloneError);

impl core::fmt::Display for ArrayTryCloneError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ArrayTryCloneError::ArrayError(err) => write!(f, "ArrayError::ArrayError({err})"),
            ArrayTryCloneError::ItemCloningError => write!(f, "ArrayError::ItemCloningError"),
        }
    }
}

impl From<ArrayError> for ArrayTryCloneError {
    fn from(err: ArrayError) -> Self {
        Self::ArrayError(err)
    }
}

/// Represents an error that occures when the array is empty.
#[reprc]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[must_use]
pub struct ArrayIsEmptyError;

osom_lib_macros::unreachable_from_infallible!(ArrayIsEmptyError);

impl core::fmt::Display for ArrayIsEmptyError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ArrayIsEmptyError")
    }
}
