//! Holds all custom errors for that crate.
use osom_lib_alloc::{AllocationError, DetailedAllocationError};

/// Represents an error that can occur when working with a tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[must_use]
#[repr(u8)]
pub enum TreeError {
    /// The allocator failed to allocate memory.
    AllocationError,

    /// The tree is too big, it exceeds `MAX_SIZE`.
    TreeTooBig,
}

impl<T: Sized> From<DetailedAllocationError<T>> for TreeError {
    fn from(_: DetailedAllocationError<T>) -> Self {
        TreeError::AllocationError
    }
}

impl From<AllocationError> for TreeError {
    fn from(_: AllocationError) -> Self {
        TreeError::AllocationError
    }
}
