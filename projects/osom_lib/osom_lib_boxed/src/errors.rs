//! Holds the definition of [`CBoxError`].
use core::fmt::Display;

use osom_lib_reprc::macros::reprc;

/// Represents possible errors when working with [`CBox`][super::cbox::CBox].
#[reprc]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[must_use]
pub enum CBoxError {
    /// The underlying allocator returned an error.
    AllocationError = 0,
}

osom_lib_macros::unreachable_from_infallible!(CBoxError);

impl Display for CBoxError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            CBoxError::AllocationError => write!(f, "CBoxError::AllocationError"),
        }
    }
}

/// Represents possible errors when working with [`CBox`][super::cbox::CBox].
#[reprc]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[must_use]
pub enum CBoxTryCloneError {
    /// The underlying allocator returned an error.
    BoxError(CBoxError) = 0,

    /// The allocator cloning failed.
    AllocatorCloningError = 1,

    /// The item cloning failed.
    ItemCloningError = 2,
}

osom_lib_macros::unreachable_from_infallible!(CBoxTryCloneError);

impl Display for CBoxTryCloneError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            CBoxTryCloneError::BoxError(e) => write!(f, "CBoxTryCloneError::BoxError({e})"),
            CBoxTryCloneError::ItemCloningError => write!(f, "CBoxTryCloneError::ItemCloningError"),
            CBoxTryCloneError::AllocatorCloningError => todo!(),
        }
    }
}

impl From<CBoxError> for CBoxTryCloneError {
    fn from(e: CBoxError) -> Self {
        Self::BoxError(e)
    }
}
