//! Holds the definition of [`BTreeError`].
use osom_lib_reprc::macros::reprc;

use osom_lib_try_clone::TryClone;

/// Represents possible errors when working with [`BTree`][super::btree::BTree].
#[reprc]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[must_use]
pub enum BTreeError {
    /// The underlying allocator returned an error.
    AllocationError = 0,

    /// The tree size is out of range, i.e. exceeds
    /// [`Length::MAX_LENGTH`][osom_lib_primitives::length::Length::MAX_LENGTH].
    TreeSizeOutOfRange = 1,
}

impl TryClone for BTreeError {
    type Error = core::convert::Infallible;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        Ok(*self)
    }
}

impl core::fmt::Display for BTreeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            BTreeError::AllocationError => write!(f, "BTreeError::AllocationError"),
            BTreeError::TreeSizeOutOfRange => write!(f, "BTreeError::TreeSizeOutOfRange"),
        }
    }
}

osom_lib_macros::unreachable_from_infallible!(BTreeError);

/// Represents possible errors when trying to clone a [`BTree`][super::btree::BTree].
#[reprc]
#[repr(u8)]
#[must_use]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum BTreeTryCloneError {
    /// The key cloning failed.
    KeyCloningError = 1,

    /// The value cloning failed.
    ValueCloningError = 2,

    /// Other error. Either due to allocator failure or other unexpected error.
    OtherError = 3,
}

impl core::fmt::Display for BTreeTryCloneError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            BTreeTryCloneError::OtherError => write!(f, "BTreeTryCloneError::OtherError"),
            BTreeTryCloneError::KeyCloningError => write!(f, "BTreeTryCloneError::KeyCloningError"),
            BTreeTryCloneError::ValueCloningError => write!(f, "BTreeTryCloneError::ValueCloningError"),
        }
    }
}

osom_lib_macros::unreachable_from_infallible!(BTreeTryCloneError);
