use core::{convert::Infallible, fmt::Write};

use osom_lib_alloc::traits::Allocator;
use osom_lib_arrays::{
    dynamic_array::DynamicArray,
    traits::{ImmutableArray, MutableArray},
};
use osom_lib_primitives::length::Length;
use osom_lib_reprc::traits::ReprC;
use osom_lib_try_clone::TryClone;

use crate::errors::{CVRCreationError, TryCloneCVRError};

use super::CVR;

/// Represents an array of [`CVR`] values.
#[repr(transparent)]
#[derive(Debug)]
#[must_use]
pub struct CVRArray<TAllocator: Allocator> {
    value: DynamicArray<CVR<TAllocator>, TAllocator>,
}

unsafe impl<TAllocator: Allocator> ReprC for CVRArray<TAllocator> {
    const CHECK: () = const {
        osom_lib_reprc::hidden::is_reprc::<CVR<TAllocator>>();
        osom_lib_reprc::hidden::is_reprc::<DynamicArray<CVR<TAllocator>, TAllocator>>();
    };
}

impl<TAllocator: Allocator> PartialEq for CVRArray<TAllocator> {
    fn eq(&self, other: &Self) -> bool {
        self.value.as_ref() == other.value.as_ref()
    }
}

impl<TAllocator: Allocator> Eq for CVRArray<TAllocator> {}

impl<TAllocator: Allocator> core::hash::Hash for CVRArray<TAllocator> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.value.as_ref().hash(state);
    }
}

impl<TAllocator: Allocator> CVRArray<TAllocator> {
    /// Creates a new empty [`CVRArray`] instance.
    #[inline]
    pub fn new() -> Self
    where
        TAllocator: Default,
    {
        Self {
            value: DynamicArray::new(),
        }
    }

    /// Creates a new empty [`CVRArray`] instance with the given allocator.
    #[inline]
    pub fn with_allocator(allocator: TAllocator) -> Self {
        Self {
            value: DynamicArray::with_allocator(allocator),
        }
    }

    /// Creates a new [`CVRArray`] instance with the given capacity.
    ///
    /// # Errors
    ///
    /// See [`CVRCreationError`] for details.
    #[inline]
    pub fn with_capacity(capacity: Length) -> Result<Self, CVRCreationError>
    where
        TAllocator: Default,
    {
        Ok(Self {
            value: DynamicArray::with_capacity(capacity)?,
        })
    }

    /// Creates a new [`CVRArray`] instance with the given capacity and allocator.
    ///
    /// # Errors
    ///
    /// See [`CVRCreationError`] for details.
    #[inline]
    pub fn with_capacity_and_allocator(capacity: Length, allocator: TAllocator) -> Result<Self, CVRCreationError> {
        Ok(Self {
            value: DynamicArray::with_capacity_and_allocator(capacity, allocator)?,
        })
    }

    #[inline(always)]
    pub const fn inner_ref(&self) -> &impl ImmutableArray<CVR<TAllocator>> {
        &self.value
    }

    #[inline(always)]
    pub const fn inner_mut(&mut self) -> &mut impl MutableArray<CVR<TAllocator>> {
        &mut self.value
    }
}

impl<TAllocator: Allocator + Default> Default for CVRArray<TAllocator> {
    fn default() -> Self {
        Self::new()
    }
}

impl<TAllocator: Allocator> From<Infallible> for CVRArray<TAllocator> {
    #[inline]
    fn from(_: Infallible) -> Self {
        unreachable!("From<Infallible> for CVRArray<TAllocator> is not possible");
    }
}

impl<TAllocator: Allocator> core::fmt::Display for CVRArray<TAllocator> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_char('[')?;
        let mut iterator = self.inner_ref().as_ref().iter();
        if let Some(item) = iterator.next() {
            write!(f, "{item}")?;
            for item in iterator {
                write!(f, ", {item}")?;
            }
        }
        f.write_char(']')
    }
}

impl<TAllocator: Allocator + TryClone> TryClone for CVRArray<TAllocator> {
    type Error = TryCloneCVRError;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        let inner = self.value.try_clone().map_err(|_| TryCloneCVRError)?;
        Ok(Self { value: inner })
    }
}

impl<TAllocator: Allocator + TryClone> Clone for CVRArray<TAllocator> {
    fn clone(&self) -> Self {
        self.try_clone().expect("Failed to clone CVRArray")
    }
}

impl<TAllocator: Allocator> From<DynamicArray<CVR<TAllocator>, TAllocator>> for CVRArray<TAllocator> {
    fn from(value: DynamicArray<CVR<TAllocator>, TAllocator>) -> Self {
        Self { value }
    }
}

impl<TAllocator: Allocator> From<CVRArray<TAllocator>> for DynamicArray<CVR<TAllocator>, TAllocator> {
    fn from(value: CVRArray<TAllocator>) -> Self {
        value.value
    }
}
