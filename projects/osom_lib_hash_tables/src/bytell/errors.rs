//! Contains bytell specific errors.

use osom_lib_alloc::traits::AllocationError;

/// Represents possible errors that can occur when dealing with the bytell hash table.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(u8)]
pub enum BytellError {
    AllocationError = 0,
    TableTooBigError = 1,
}

impl From<AllocationError> for BytellError {
    #[inline(always)]
    fn from(_: AllocationError) -> Self {
        BytellError::AllocationError
    }
}
