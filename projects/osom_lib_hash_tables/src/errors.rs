//! Holds definitions of various array errors.
use osom_lib_reprc::macros::reprc;

/// Represents a general issue that can occure when dealing
/// with hash tables.
#[reprc]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[must_use]
pub enum HashTableError {
    /// The underlying allocator returned an error,
    /// likely due to out of memory.
    AllocationError = 0,

    /// Tried to initialize a hash table or push to it beyond its internal limit.
    LengthLimitExceeded = 1,

    /// The underlying allocator cloning failed.
    AllocatorCloningError = 2,
}

osom_lib_macros::unreachable_from_infallible!(HashTableError);

impl core::fmt::Display for HashTableError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            HashTableError::AllocationError => write!(f, "HashTableError::AllocationError"),
            HashTableError::LengthLimitExceeded => write!(f, "HashTableError::LengthLimitExceeded"),
            HashTableError::AllocatorCloningError => write!(f, "HashTableError::AllocatorCloningError"),
        }
    }
}

/// Represents possible errors when trying to clone a hash table.
#[reprc]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[must_use]
pub enum TryCloneHashTableError {
    /// The underlying hash table returned an error.
    HashTableError(HashTableError) = 0,

    /// The key or value returned an error.
    KeyOrValueError = 1,
}

osom_lib_macros::unreachable_from_infallible!(TryCloneHashTableError);

impl From<HashTableError> for TryCloneHashTableError {
    fn from(error: HashTableError) -> Self {
        Self::HashTableError(error)
    }
}

impl core::fmt::Display for TryCloneHashTableError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TryCloneHashTableError::HashTableError(error) => {
                write!(f, "TryCloneHashTableError::HashTableError({error})")
            }
            TryCloneHashTableError::KeyOrValueError => write!(f, "TryCloneHashTableError::KeyOrValueError"),
        }
    }
}
