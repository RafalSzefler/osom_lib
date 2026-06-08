use core::{borrow::Borrow, cmp::Ordering};

use osom_lib_alloc::traits::Allocator;
use osom_lib_reprc::macros::reprc;
use osom_lib_strings::immutable::ImmutableString;
use osom_lib_try_clone::TryClone;

use crate::errors::{CVRCreationError, TryCloneCVRError};

/// Represents a string value. Internally it is an [`ImmutableString`] value,
/// which is a smart pointer around a string backed by reference counting.
///
/// In particular cloning this struct is cheap. However, the string
/// is immutable and cannot be modified once constructed.
#[reprc]
#[repr(transparent)]
#[derive(Debug)]
#[must_use]
pub struct CVRString<TAllocator: Allocator> {
    value: ImmutableString<TAllocator>,
}

impl<TAllocator: Allocator> CVRString<TAllocator> {
    /// Creates a new [`CVRString`] instance.
    ///
    /// # Errors
    ///
    /// See [`CVRCreationError`] for details.
    pub fn new(value: &str) -> Result<Self, CVRCreationError>
    where
        TAllocator: Default,
    {
        let inner = ImmutableString::from_str_slice(value)?;
        Ok(Self { value: inner })
    }

    /// Creates a new [`CVRString`] instance with the given allocator.
    ///
    /// # Errors
    ///
    /// See [`CVRCreationError`] for details.
    pub fn with_allocator(value: &str, allocator: TAllocator) -> Result<Self, CVRCreationError> {
        let inner = ImmutableString::from_str_slice_and_allocator(value, allocator)?;
        Ok(Self { value: inner })
    }

    /// Returns the underlying string.
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }

    /// Returns the underlying [`ImmutableString`] value.
    #[inline]
    pub fn as_immutable_string(&self) -> &ImmutableString<TAllocator> {
        &self.value
    }
}

impl<TAllocator: Allocator> PartialEq for CVRString<TAllocator> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<TAllocator: Allocator> Eq for CVRString<TAllocator> {}

impl<TAllocator: Allocator> PartialOrd for CVRString<TAllocator> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<TAllocator: Allocator> Ord for CVRString<TAllocator> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.as_str().cmp(other.value.as_str())
    }
}

impl<TAllocator: Allocator> core::hash::Hash for CVRString<TAllocator> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<TAllocator: Allocator> AsRef<str> for CVRString<TAllocator> {
    fn as_ref(&self) -> &str {
        self.value.as_str()
    }
}

impl<TAllocator: Allocator> Borrow<str> for CVRString<TAllocator> {
    fn borrow(&self) -> &str {
        self.value.as_str()
    }
}

impl<TAllocator: Allocator + TryClone> TryClone for CVRString<TAllocator> {
    type Error = TryCloneCVRError;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        let inner = self.value.try_clone().map_err(|_| TryCloneCVRError)?;
        Ok(Self { value: inner })
    }
}

impl<TAllocator: Allocator + TryClone> Clone for CVRString<TAllocator> {
    fn clone(&self) -> Self {
        self.try_clone().expect("Failed to clone CVRString")
    }
}

impl<TAllocator: Allocator> core::fmt::Display for CVRString<TAllocator> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<TAllocator: Allocator> From<ImmutableString<TAllocator>> for CVRString<TAllocator> {
    fn from(value: ImmutableString<TAllocator>) -> Self {
        Self { value: value }
    }
}

impl<TAllocator: Allocator> From<CVRString<TAllocator>> for ImmutableString<TAllocator> {
    fn from(value: CVRString<TAllocator>) -> Self {
        value.value
    }
}
