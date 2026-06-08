use osom_lib_arc::errors::CArcArrayError;
use osom_lib_primitives::length::LengthError;
use osom_lib_reprc::macros::reprc;

/// Represents potential errors when working with `ImmutableString`.
#[reprc]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ImmutableStringError {
    /// The internal allocator returned an error.
    AllocationError = 0,

    /// Max length exceeded.
    MaxLengthExceeded = 1,
}

impl From<CArcArrayError> for ImmutableStringError {
    fn from(err: CArcArrayError) -> Self {
        match err {
            CArcArrayError::AllocationError => Self::AllocationError,
            CArcArrayError::ArraySizeOutOfRange => Self::MaxLengthExceeded,
        }
    }
}

osom_lib_macros::unreachable_from_infallible!(ImmutableStringError);

impl From<LengthError> for ImmutableStringError {
    fn from(_: LengthError) -> Self {
        Self::MaxLengthExceeded
    }
}

impl core::fmt::Display for ImmutableStringError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ImmutableStringError::AllocationError => write!(f, "ImmutableStringError::AllocationError"),
            ImmutableStringError::MaxLengthExceeded => write!(f, "ImmutableStringError::MaxLengthExceeded"),
        }
    }
}

/// Represents an error that occures when the maximum number of references is exceeded.
#[reprc]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct MaxReferencesExceededError;

osom_lib_macros::unreachable_from_infallible!(MaxReferencesExceededError);

impl From<osom_lib_arc::errors::MaxReferencesExceededError> for MaxReferencesExceededError {
    fn from(_: osom_lib_arc::errors::MaxReferencesExceededError) -> Self {
        Self
    }
}
