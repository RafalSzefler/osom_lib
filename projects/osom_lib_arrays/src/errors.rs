//! Holds definitions of various array errors.

use osom_lib_alloc::traits::AllocationError;
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

impl From<AllocationError> for ArrayError {
    fn from(_: AllocationError) -> Self {
        Self::AllocationError
    }
}

/// Represents an error that occures when the array is empty.
#[reprc]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
#[must_use]
pub struct ArrayIsEmptyError;
