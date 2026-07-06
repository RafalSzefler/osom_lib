use osom_lib_arrays::errors::{ArrayError, ArrayTryCloneError};
use osom_lib_reprc::macros::reprc;

/// Represents errors that can occur when working with [`OwnedString`][super::OwnedString]
/// and [`OwnedStringBuilder`][super::OwnedStringBuilder].
#[reprc]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum OwnedStringError {
    /// The underlying allocator failed to allocate memory.
    AllocationError = 0,

    /// The string length exceeded the maximum allowed length.
    MaxLengthExceeded = 1,
}

osom_lib_macros::unreachable_from_infallible!(OwnedStringError);

impl From<ArrayError> for OwnedStringError {
    fn from(err: ArrayError) -> Self {
        match err {
            ArrayError::AllocationError => Self::AllocationError,
            ArrayError::LengthLimitExceeded => OwnedStringError::MaxLengthExceeded,
        }
    }
}

impl From<ArrayTryCloneError> for OwnedStringError {
    fn from(err: ArrayTryCloneError) -> Self {
        match err {
            ArrayTryCloneError::ArrayError(array_error) => array_error.into(),
            ArrayTryCloneError::ItemCloningError => {
                unreachable!("ArrayTryCloneError::ItemCloningError should not be possible")
            }
        }
    }
}

impl core::fmt::Display for OwnedStringError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            OwnedStringError::AllocationError => write!(f, "OwnedStringError::AllocationError"),
            OwnedStringError::MaxLengthExceeded => write!(f, "OwnedStringError::MaxLengthExceeded"),
        }
    }
}
