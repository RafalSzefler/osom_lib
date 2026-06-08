//! Holds definitions of various `CVR` errors.
use osom_lib_arrays::errors::ArrayError;
use osom_lib_btree::errors::BTreeError;
use osom_lib_reprc::macros::reprc;
use osom_lib_strings::immutable::ImmutableStringError;
use osom_lib_try_clone::TryClone;

macro_rules! default_impls {
    ( $name:ident ) => {
        osom_lib_macros::unreachable_from_infallible!($name);

        impl TryClone for $name {
            type Error = core::convert::Infallible;

            fn try_clone(&self) -> Result<Self, Self::Error> {
                Ok(*self)
            }
        }
    };
}

/// Represents an error that occurs when converting a [`CVR`][crate::cvr::CVR] value to another type.
#[reprc]
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[must_use]
pub struct CVRConvertionError;

default_impls!(CVRConvertionError);

/// Represents an error that occurs when trying to clone any of the
/// [`CVR`][crate::cvr::CVR] structs.
#[reprc]
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[must_use]
pub struct TryCloneCVRError;

impl core::fmt::Display for TryCloneCVRError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "TryCloneCVRError")
    }
}

default_impls!(TryCloneCVRError);

/// Represents an error that occurs when convertion to/from a
/// [`CVRInt`][crate::cvr::CVRInt] fails.
#[reprc]
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[must_use]
pub struct TryFromCVRIntError;

impl core::fmt::Display for TryFromCVRIntError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "TryFromCVRIntError")
    }
}

default_impls!(TryFromCVRIntError);

impl From<core::num::TryFromIntError> for TryFromCVRIntError {
    fn from(_: core::num::TryFromIntError) -> Self {
        Self
    }
}

/// Represents an error that occurs when creating a new `CVR`
/// object. This typically means that the underlying allocator
/// returned an error.
#[reprc]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[must_use]
pub enum CVRCreationError {
    /// The underlying allocator returned an error.
    AllocationError = 0,

    /// The underlying array returned an error. This typically means
    /// either an allocation error or a length limit exceeded error.
    ArrayError = 1,

    /// The underlying string returned an error. This typically means
    /// either an allocation error or a length limit exceeded error.
    StringError = 2,
}

impl core::fmt::Display for CVRCreationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "CVRCreationError")
    }
}

impl From<ArrayError> for CVRCreationError {
    fn from(_: ArrayError) -> Self {
        Self::ArrayError
    }
}

impl From<ImmutableStringError> for CVRCreationError {
    fn from(_: ImmutableStringError) -> Self {
        Self::StringError
    }
}

default_impls!(CVRCreationError);

/// Represents an error that occurs when inserting a key-value pair into a [`CVRObject`][crate::cvr::CVRObject].
#[reprc]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[must_use]
pub enum CVRObjectInsertError {
    /// The underlying allocator returned an error.
    AllocationError = 0,

    /// The underlying btree returned an error.
    BTreeError(BTreeError) = 1,
}

impl core::fmt::Display for CVRObjectInsertError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::AllocationError => write!(f, "CVRObjectInsertError::AllocationError"),
            Self::BTreeError(err) => write!(f, "CVRObjectInsertError::BTreeError({err})"),
        }
    }
}

impl From<BTreeError> for CVRObjectInsertError {
    fn from(err: BTreeError) -> Self {
        Self::BTreeError(err)
    }
}

default_impls!(CVRObjectInsertError);
