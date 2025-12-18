//! Holds the definition of [`DynamicArray`].

use core::{
    borrow::{Borrow, BorrowMut},
    hash::Hash,
};

use osom_lib_alloc::traits::Allocator;
use osom_lib_primitives::length::Length;
use osom_lib_reprc::traits::ReprC;

use crate::{
    errors::{ArrayError, ArrayIsEmptyError},
    internal_array::InternalArray,
    traits::{ImmutableArray, MutableArray},
};

/// A `#[repr(C)]` variant of the standard `vec` struct.
///
/// Functionally similar, and implements [`ReprC`] for `T: ReprC`.
#[derive(Debug)]
#[repr(transparent)]
#[must_use]
pub struct DynamicArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    inner: InternalArray<T, TAllocator>,
}

unsafe impl<T, TAllocator> ReprC for DynamicArray<T, TAllocator>
where
    T: ReprC,
    TAllocator: Allocator,
{
    const CHECK: () = {
        let () = <InternalArray<T, TAllocator> as ReprC>::CHECK;
    };
}

impl<T, TAllocator> DynamicArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    /// Creates a new, empty [`DynamicArray`].
    #[inline(always)]
    pub fn new() -> Self {
        Self::with_allocator(TAllocator::default())
    }

    /// Creates a new, empty [`DynamicArray`] with an allocator.
    #[inline(always)]
    pub const fn with_allocator(allocator: TAllocator) -> Self {
        Self {
            inner: InternalArray::new(allocator),
        }
    }

    /// Creates a new [`DynamicArray`] with capacity and allocator.
    /// This allocates memory only when `capacity > 0`.
    ///
    /// # Errors
    ///
    /// For details see [`ArrayError`].
    #[inline(always)]
    pub fn with_capacity_and_allocator(capacity: Length, allocator: TAllocator) -> Result<Self, ArrayError> {
        let inner = InternalArray::<T, TAllocator>::with_capacity(capacity, allocator)?;
        Ok(Self { inner })
    }

    /// Creates a new [`DynamicArray`] with capacity and the default allocator.
    /// This allocates memory only when `capacity > 0`.
    ///
    /// # Errors
    ///
    /// For details see [`ArrayError`].
    #[inline(always)]
    pub fn with_capacity(capacity: Length) -> Result<Self, ArrayError> {
        Self::with_capacity_and_allocator(capacity, TAllocator::default())
    }
}

impl<T, TAllocator> ImmutableArray<T> for DynamicArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    #[inline(always)]
    fn length(&self) -> Length {
        self.inner.length()
    }

    #[inline(always)]
    fn capacity(&self) -> Length {
        self.inner.capacity()
    }

    #[inline(always)]
    fn as_slice(&self) -> &[T] {
        self.inner.as_slice()
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.length().as_u32() == 0
    }
}

impl<T, TAllocator> MutableArray<T> for DynamicArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    #[inline(always)]
    fn try_push_array<const TSIZE: usize>(&mut self, arr: [T; TSIZE]) -> Result<(), ArrayError> {
        self.inner.try_push_array(arr)
    }

    #[inline(always)]
    fn try_push_slice(&mut self, slice: &[T]) -> Result<(), ArrayError>
    where
        T: Clone,
    {
        self.inner.try_push_slice(slice)
    }

    #[inline(always)]
    fn try_pop(&mut self) -> Result<T, ArrayIsEmptyError> {
        self.inner.try_pop()
    }

    #[inline(always)]
    fn as_slice_mut(&mut self) -> &mut [T] {
        self.inner.as_slice_mut()
    }
}

impl<T, TAllocator> Drop for DynamicArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    fn drop(&mut self) {
        unsafe { self.inner.deallocate() };
    }
}

impl<T, TAllocator> Default for DynamicArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone, TAllocator> Clone for DynamicArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T, TAllocator, Rhs> PartialEq<Rhs> for DynamicArray<T, TAllocator>
where
    T: PartialEq,
    TAllocator: Allocator,
    Rhs: AsRef<[T]>,
{
    fn eq(&self, other: &Rhs) -> bool {
        self.as_slice() == other.as_ref()
    }
}

impl<T, TAllocator> Eq for DynamicArray<T, TAllocator>
where
    T: Eq,
    TAllocator: Allocator,
{
}

impl<T, TAllocator> Hash for DynamicArray<T, TAllocator>
where
    T: Hash,
    TAllocator: Allocator,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

impl<T, TAllocator> AsRef<[T]> for DynamicArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, TAllocator> AsMut<[T]> for DynamicArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    fn as_mut(&mut self) -> &mut [T] {
        self.as_slice_mut()
    }
}

impl<T, TAllocator> Borrow<[T]> for DynamicArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    fn borrow(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, TAllocator> BorrowMut<[T]> for DynamicArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    fn borrow_mut(&mut self) -> &mut [T] {
        self.as_slice_mut()
    }
}

impl<T, TAllocator> core::ops::Index<Length> for DynamicArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    type Output = T;

    #[inline(always)]
    fn index(&self, index: Length) -> &Self::Output {
        &self.as_slice()[index.as_usize()]
    }
}

impl<T, TAllocator> core::ops::IndexMut<Length> for DynamicArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    #[inline(always)]
    fn index_mut(&mut self, index: Length) -> &mut Self::Output {
        &mut self.as_slice_mut()[index.as_usize()]
    }
}
