//! Holds the definition of [`DynamicArray`].

use core::{
    borrow::{Borrow, BorrowMut},
    hash::Hash,
};

use osom_lib_alloc::traits::Allocator;
use osom_lib_primitives::length::Length;
use osom_lib_reprc::traits::ReprC;
use osom_lib_try_clone::TryClone;

use crate::{
    errors::{ArrayError, ArrayIsEmptyError, ArrayTryCloneError},
    traits::{ImmutableArray, MutableArray},
};

use super::internal_array::InternalArray;

/// A `#[repr(C)]` variant of the standard `vec` struct.
///
/// Functionally similar, and implements [`ReprC`] for `T: ReprC`. However,
/// unlike `vec` this struct multiplies capacity by `3/2` when resizing is
/// needed.
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
    const CHECK: () = const {
        osom_lib_reprc::hidden::is_reprc::<T>();
        osom_lib_reprc::hidden::is_reprc::<InternalArray<T, TAllocator>>();
    };
}

impl<T, TAllocator> DynamicArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    /// Creates a new, empty [`DynamicArray`].
    #[inline(always)]
    pub fn new() -> Self
    where
        TAllocator: Default,
    {
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
    pub fn with_capacity(capacity: Length) -> Result<Self, ArrayError>
    where
        TAllocator: Default,
    {
        Self::with_capacity_and_allocator(capacity, TAllocator::default())
    }

    /// Creates a new [`DynamicArray`] with a given size, generated through a given factory.
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
    pub fn with_factory<Factory: FnMut(usize) -> T>(size: Length, factory: Factory) -> Result<Self, ArrayError>
    where
        TAllocator: Default,
    {
        Self::with_factory_and_allocator(size, factory, TAllocator::default())
    }

    /// Creates a new [`DynamicArray`] with a given size, generated through a given factory,
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
    pub fn with_factory_and_allocator<Factory: FnMut(usize) -> T>(
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

    /// Creates a new [`DynamicArray`] with a given size, but uninitialized.
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

    /// Creates a new [`DynamicArray`] with a given size and allocator, but uninitialized.
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
    fn try_push_slice(&mut self, slice: &[T]) -> Result<(), ArrayTryCloneError>
    where
        T: TryClone,
    {
        self.inner.try_push_slice(slice)
    }

    #[inline(always)]
    fn try_pop(&mut self) -> Result<T, ArrayIsEmptyError> {
        self.inner.try_pop()
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
    TAllocator: Allocator + Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, TAllocator> Clone for DynamicArray<T, TAllocator>
where
    T: TryClone + Clone,
    TAllocator: Allocator + TryClone + Clone,
{
    fn clone(&self) -> Self {
        self.try_clone().expect("Failed to clone dynamic array")
    }
}

impl<T, TAllocator> TryClone for DynamicArray<T, TAllocator>
where
    T: TryClone,
    TAllocator: Allocator + TryClone,
{
    type Error = ArrayTryCloneError;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        let inner = self.inner.try_clone_with_capacity()?;
        Ok(Self { inner })
    }
}

impl<T, TAllocator, Rhs> PartialEq<Rhs> for DynamicArray<T, TAllocator>
where
    T: PartialEq,
    TAllocator: Allocator,
    Rhs: AsRef<[T]>,
{
    fn eq(&self, other: &Rhs) -> bool {
        self.as_ref() == other.as_ref()
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
        self.as_ref().hash(state);
    }
}

impl<T, TAllocator> AsRef<[T]> for DynamicArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    fn as_ref(&self) -> &[T] {
        self.inner.as_slice()
    }
}

impl<T, TAllocator> AsMut<[T]> for DynamicArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    fn as_mut(&mut self) -> &mut [T] {
        self.inner.as_slice_mut()
    }
}

impl<T, TAllocator> Borrow<[T]> for DynamicArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    #[inline(always)]
    fn borrow(&self) -> &[T] {
        self.as_ref()
    }
}

impl<T, TAllocator> BorrowMut<[T]> for DynamicArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    #[inline(always)]
    fn borrow_mut(&mut self) -> &mut [T] {
        self.as_mut()
    }
}
