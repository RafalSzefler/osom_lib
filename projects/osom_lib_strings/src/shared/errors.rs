use osom_lib_arc::errors::CArcArrayError;
use osom_lib_primitives::length::LengthError;
use osom_lib_reprc::macros::reprc;

/// Represents potential errors when working with `ImmutableString`.
#[reprc]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum SharedStringError {
    /// The internal allocator returned an error.
    AllocationError = 0,

    /// Max length exceeded.
    MaxLengthExceeded = 1,
}

impl From<CArcArrayError> for SharedStringError {
    fn from(err: CArcArrayError) -> Self {
        match err {
            CArcArrayError::AllocationError => Self::AllocationError,
            CArcArrayError::ArraySizeOutOfRange => Self::MaxLengthExceeded,
            CArcArrayError::ItemCloningError => panic!("CArcArrayError::ItemCloningError should not be possible here"),
        }
    }
}

osom_lib_macros::unreachable_from_infallible!(SharedStringError);

impl From<LengthError> for SharedStringError {
    fn from(_: LengthError) -> Self {
        Self::MaxLengthExceeded
    }
}

impl core::fmt::Display for SharedStringError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SharedStringError::AllocationError => write!(f, "ImmutableStringError::AllocationError"),
            SharedStringError::MaxLengthExceeded => write!(f, "ImmutableStringError::MaxLengthExceeded"),
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
