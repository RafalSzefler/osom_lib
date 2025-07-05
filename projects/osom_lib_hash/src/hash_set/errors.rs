use osom_lib_alloc::AllocationError;
use osom_lib_arrays::{DynamicArrayConstructionError, InlineDynamicArrayConstructionError};

/// An error that can occur when working with a [`HashSet`][`super::HashSet`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[must_use]
#[repr(u8)]
pub enum HashSetError {
    /// The underlying allocator failed to allocate memory.
    AllocationError,

    /// The [`HashSet`][`super::HashSet`] is too big.
    HashSetTooBig,
}

impl From<AllocationError> for HashSetError {
    fn from(_: AllocationError) -> Self {
        HashSetError::AllocationError
    }
}

impl From<DynamicArrayConstructionError> for HashSetError {
    fn from(err: DynamicArrayConstructionError) -> Self {
        match err {
            DynamicArrayConstructionError::AllocationError => HashSetError::AllocationError,
            DynamicArrayConstructionError::ArrayTooLong => HashSetError::HashSetTooBig,
        }
    }
}

impl From<InlineDynamicArrayConstructionError> for HashSetError {
    fn from(err: InlineDynamicArrayConstructionError) -> Self {
        match err {
            InlineDynamicArrayConstructionError::AllocationError => HashSetError::AllocationError,
            InlineDynamicArrayConstructionError::ArrayTooLong => HashSetError::HashSetTooBig,
        }
    }
}
