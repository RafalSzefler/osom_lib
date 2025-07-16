//! Holds all custom errors for that crate.
use osom_lib_alloc::{AllocationError, DetailedAllocationError};

/// Represents an error that occurs when constructing a new array.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[must_use]
#[repr(u8)]
pub enum ArrayConstructionError {
    /// The allocator failed to allocate memory.
    AllocationError,

    /// The passed array is too long, it exceeds `MAX_LENGTH`.
    ArrayTooLong,
}

impl From<AllocationError> for ArrayConstructionError {
    fn from(_: AllocationError) -> Self {
        ArrayConstructionError::AllocationError
    }
}

impl<T: Sized> From<DetailedAllocationError<T>> for ArrayConstructionError {
    fn from(_: DetailedAllocationError<T>) -> Self {
        ArrayConstructionError::AllocationError
    }
}
