//! Holds the definition of [`DynamicArray`].

use core::{
    borrow::{Borrow, BorrowMut},
    hash::Hash,
};

use osom_lib_alloc::traits::Allocator;
use osom_lib_primitives::length::Length;
use osom_lib_reprc::macros::reprc;
use osom_lib_try_clone::TryClone;

use crate::{
    errors::{ArrayError, ArrayIsEmptyError, ArrayTryCloneError},
    traits::{ImmutableArray, MutableArray},
};

use super::internal_array::InternalArray;

/// A `#[repr(C)]` variant of the standard `vec` struct.
///
/// Functionally similar, and implements [`ReprC`][osom_lib_reprc::traits::ReprC] for `T: ReprC`.
/// However, unlike `vec` this struct multiplies capacity by `3/2` when resizing is needed.
///
/// Additionally it accepts `TAlign` generic parameter, which is used to enforce a specific alignment of the
/// internal buffer. The struct does not keep `TAlign` instances inside.
#[derive(Debug)]
#[reprc]
#[repr(transparent)]
#[must_use]
pub struct AlignedDynamicArray<TAlign, TItem, TAllocator>
where
    TAllocator: Allocator,
{
    inner: InternalArray<TAlign, TItem, TAllocator>,
}

impl<TAlign, TItem, TAllocator> AlignedDynamicArray<TAlign, TItem, TAllocator>
where
    TAllocator: Allocator,
{
    /// Creates a new, empty [`AlignedDynamicArray`].
    #[inline(always)]
    pub fn new() -> Self
    where
        TAllocator: Default,
    {
        Self::with_allocator(TAllocator::default())
    }

    /// Creates a new, empty [`AlignedDynamicArray`] with an allocator.
    #[inline(always)]
    pub const fn with_allocator(allocator: TAllocator) -> Self {
        Self {
            inner: InternalArray::new(allocator),
        }
    }

    /// Creates a new [`AlignedDynamicArray`] with capacity and allocator.
    /// This allocates memory only when `capacity > 0`.
    ///
    /// # Errors
    ///
    /// For details see [`ArrayError`].
    #[inline(always)]
    pub fn with_capacity_and_allocator(capacity: Length, allocator: TAllocator) -> Result<Self, ArrayError> {
        let inner = InternalArray::<TAlign, TItem, TAllocator>::with_capacity(capacity, allocator)?;
        Ok(Self { inner })
    }

    /// Creates a new [`AlignedDynamicArray`] with capacity and the default allocator.
    /// This allocates memory only when `capacity > 0`.
    ///
    /// # Errors
    ///
    /// For details see [`ArrayError`].
    #[inline(always)]
    pub fn with_capacity(capacity: Length) -> Result<Self, ArrayError>
    where
        TAllocator: Default,
    {
        Self::with_capacity_and_allocator(capacity, TAllocator::default())
    }

    /// Creates a new [`AlignedDynamicArray`] with a given size, generated through a given factory.
    /// This allocates memory only when `size > 0`.
    ///
    /// # Notes
    ///
    /// This method is functionally equivalent to initializing an empty vector and running
    /// factory one by one in a loop, and passing it to
    /// [`push`][`crate::traits::MutableArray::push`]. This way, however, is more efficient,
    /// even if you preallocate the vector with capacity. Because this method gives the
    /// compiler an opportunity to vectorize the construction, unlike sequential
    /// [`push`][`crate::traits::MutableArray::push`] calls.
    ///
    /// # Errors
    ///
    /// For details see [`ArrayError`].
    #[inline(always)]
    pub fn with_factory<Factory: FnMut(usize) -> TItem>(size: Length, factory: Factory) -> Result<Self, ArrayError>
    where
        TAllocator: Default,
    {
        Self::with_factory_and_allocator(size, factory, TAllocator::default())
    }

    /// Creates a new [`AlignedDynamicArray`] with a given size, generated through a given factory,
    /// with a custom allocator. This allocates memory only when `size > 0`.
    ///
    /// # Notes
    ///
    /// This method is functionally equivalent to initializing an empty vector and running
    /// factory one by one in a loop, and passing it to
    /// [`push`][`crate::traits::MutableArray::push`]. This way, however, is more efficient,
    /// even if you preallocate the vector with capacity. Because this method gives the
    /// compiler an opportunity to vectorize the construction, unlike sequential
    /// [`push`][`crate::traits::MutableArray::push`] calls.
    ///
    /// # Errors
    ///
    /// For details see [`ArrayError`].
    pub fn with_factory_and_allocator<Factory: FnMut(usize) -> TItem>(
        size: Length,
        mut factory: Factory,
        allocator: TAllocator,
    ) -> Result<Self, ArrayError> {
        unsafe {
            let mut array = Self::with_size_and_allocator_uninitialized(size, allocator)?;
            let slice_mut_ptr = array.as_mut().as_mut_ptr();
            for idx in 0..size.as_usize() {
                slice_mut_ptr.add(idx).write(factory(idx));
            }
            Ok(array)
        }
    }

    /// Creates a new [`AlignedDynamicArray`] with a given size, but uninitialized.
    ///
    /// # Safety
    ///
    /// The underlying array is uninitialized and reading the data is UB, unless
    /// initialized first.
    ///
    /// # Errors
    ///
    /// For details see [`ArrayError`].
    #[inline(always)]
    pub unsafe fn with_size_uninitialized(size: Length) -> Result<Self, ArrayError>
    where
        TAllocator: Default,
    {
        unsafe { Self::with_size_and_allocator_uninitialized(size, TAllocator::default()) }
    }

    /// Creates a new [`AlignedDynamicArray`] with a given size and allocator, but uninitialized.
    ///
    /// # Safety
    ///
    /// The underlying array is uninitialized and reading the data is UB, unless
    /// initialized first.
    ///
    /// # Errors
    ///
    /// For details see [`ArrayError`].
    pub unsafe fn with_size_and_allocator_uninitialized(
        size: Length,
        allocator: TAllocator,
    ) -> Result<Self, ArrayError> {
        let inner = unsafe { InternalArray::with_size_uninitialized(size, allocator) }?;
        Ok(Self { inner })
    }
}

impl<TAlign, TItem, TAllocator> ImmutableArray<TItem> for AlignedDynamicArray<TAlign, TItem, TAllocator>
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
    fn is_empty(&self) -> bool {
        self.length().as_u32() == 0
    }
}

impl<TAlign, TItem, TAllocator> MutableArray<TItem> for AlignedDynamicArray<TAlign, TItem, TAllocator>
where
    TAllocator: Allocator,
{
    #[inline(always)]
    fn try_push_array<const TSIZE: usize>(&mut self, arr: [TItem; TSIZE]) -> Result<(), ArrayError> {
        self.inner.try_push_array(arr)
    }

    #[inline(always)]
    fn try_push_slice(&mut self, slice: &[TItem]) -> Result<(), ArrayTryCloneError>
    where
        TItem: TryClone,
    {
        self.inner.try_push_slice(slice)
    }

    #[inline(always)]
    fn try_pop(&mut self) -> Result<TItem, ArrayIsEmptyError> {
        self.inner.try_pop()
    }
}

impl<TAlign, TItem, TAllocator> Drop for AlignedDynamicArray<TAlign, TItem, TAllocator>
where
    TAllocator: Allocator,
{
    fn drop(&mut self) {
        unsafe { self.inner.deallocate() };
    }
}

impl<TAlign, TItem, TAllocator> Default for AlignedDynamicArray<TAlign, TItem, TAllocator>
where
    TAllocator: Allocator + Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<TAlign, TItem, TAllocator> Clone for AlignedDynamicArray<TAlign, TItem, TAllocator>
where
    TItem: TryClone + Clone,
    TAllocator: Allocator + TryClone + Clone,
{
    fn clone(&self) -> Self {
        self.try_clone().expect("Failed to clone dynamic array")
    }
}

impl<TAlign, TItem, TAllocator> TryClone for AlignedDynamicArray<TAlign, TItem, TAllocator>
where
    TItem: TryClone,
    TAllocator: Allocator + TryClone,
{
    type Error = ArrayTryCloneError;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        let inner = self.inner.try_clone_with_capacity()?;
        Ok(Self { inner })
    }
}

impl<TAlign, TItem, TAllocator, Rhs> PartialEq<Rhs> for AlignedDynamicArray<TAlign, TItem, TAllocator>
where
    TItem: PartialEq,
    TAllocator: Allocator,
    Rhs: AsRef<[TItem]>,
{
    fn eq(&self, other: &Rhs) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl<TAlign, TItem, TAllocator> Eq for AlignedDynamicArray<TAlign, TItem, TAllocator>
where
    TItem: Eq,
    TAllocator: Allocator,
{
}

impl<TAlign, TItem, TAllocator> Hash for AlignedDynamicArray<TAlign, TItem, TAllocator>
where
    TItem: Hash,
    TAllocator: Allocator,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}

impl<TAlign, TItem, TAllocator> AsRef<[TItem]> for AlignedDynamicArray<TAlign, TItem, TAllocator>
where
    TAllocator: Allocator,
{
    fn as_ref(&self) -> &[TItem] {
        self.inner.as_slice()
    }
}

impl<TAlign, TItem, TAllocator> AsMut<[TItem]> for AlignedDynamicArray<TAlign, TItem, TAllocator>
where
    TAllocator: Allocator,
{
    fn as_mut(&mut self) -> &mut [TItem] {
        self.inner.as_slice_mut()
    }
}

impl<TAlign, TItem, TAllocator> Borrow<[TItem]> for AlignedDynamicArray<TAlign, TItem, TAllocator>
where
    TAllocator: Allocator,
{
    #[inline(always)]
    fn borrow(&self) -> &[TItem] {
        self.as_ref()
    }
}

impl<TAlign, TItem, TAllocator> BorrowMut<[TItem]> for AlignedDynamicArray<TAlign, TItem, TAllocator>
where
    TAllocator: Allocator,
{
    #[inline(always)]
    fn borrow_mut(&mut self) -> &mut [TItem] {
        self.as_mut()
    }
}
