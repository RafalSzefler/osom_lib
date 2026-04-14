//! Holds the definition of [`FixedArray`].

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

/// A fixed-capacity array. This type is similar to [`DynamicArray`][`crate::dynamic_array::DynamicArray`],
/// except its capacity is fixed at runtime, instead of compile time. Internally it keeps a pointer
/// to heap allocated data. That heap allocated data is never resized during the array's lifetime.
///
/// This type does require an allocator. But if the size exceeds capacity it simply returns an error.
#[derive(Debug)]
#[repr(transparent)]
#[must_use]
pub struct FixedArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    inner: InternalArray<T, TAllocator>,
}

unsafe impl<T, TAllocator> ReprC for FixedArray<T, TAllocator>
where
    T: ReprC,
    TAllocator: Allocator,
{
    const CHECK: () = {
        let () = <InternalArray<T, TAllocator> as ReprC>::CHECK;
    };
}

impl<T, TAllocator> FixedArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    /// Creates a new [`FixedArray`] with capacity and allocator.
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

    /// Creates a new [`FixedArray`] with capacity and the default allocator.
    /// This allocates memory only when `capacity > 0`.
    ///
    /// # Errors
    ///
    /// For details see [`ArrayError`].
    #[inline(always)]
    pub fn with_capacity(capacity: Length) -> Result<Self, ArrayError> {
        Self::with_capacity_and_allocator(capacity, TAllocator::default())
    }

    /// Creates a new [`FixedArray`] with a given size, generated through a given factory.
    /// This allocates memory only when `size > 0`.
    ///
    /// # Errors
    ///
    /// For details see [`ArrayError`].
    #[inline(always)]
    pub fn with_factory<Factory: FnMut(usize) -> T>(size: Length, factory: Factory) -> Result<Self, ArrayError> {
        Self::with_factory_and_allocator(size, factory, TAllocator::default())
    }

    /// Creates a new [`FixedArray`] with a given size, generated through a given factory,
    /// with a custom allocator. This allocates memory only when `size > 0`.
    ///
    /// # Errors
    ///
    /// For details see [`ArrayError`].
    pub fn with_factory_and_allocator<Factory: FnMut(usize) -> T>(
        size: Length,
        mut factory: Factory,
        allocator: TAllocator,
    ) -> Result<Self, ArrayError> {
        let mut array = unsafe { Self::with_size_and_allocator_uninitialized(size, allocator) }?;

        #[allow(clippy::needless_range_loop)]
        {
            let slice_mut = array.as_slice_mut();
            for idx in 0..size.as_usize() {
                slice_mut[idx] = factory(idx);
            }
        }
        Ok(array)
    }

    /// Creates a new [`FixedArray`] with a given size, but uninitialized.
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
    pub unsafe fn with_size_uninitialized(size: Length) -> Result<Self, ArrayError> {
        unsafe { Self::with_size_and_allocator_uninitialized(size, TAllocator::default()) }
    }

    /// Creates a new [`FixedArray`] with a given size and allocator, but uninitialized.
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

impl<T, TAllocator> ImmutableArray<T> for FixedArray<T, TAllocator>
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

impl<T, TAllocator> MutableArray<T> for FixedArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    #[inline(always)]
    fn try_push_array<const TSIZE: usize>(&mut self, arr: [T; TSIZE]) -> Result<(), ArrayError> {
        if TSIZE > u32::MAX as usize {
            return Err(ArrayError::LengthLimitExceeded);
        }
        if unsafe { self.inner.length().as_usize().unchecked_add(TSIZE) } > self.inner.capacity().as_usize() {
            return Err(ArrayError::LengthLimitExceeded);
        }
        self.inner.try_push_array(arr)
    }

    #[inline(always)]
    fn try_push_slice(&mut self, slice: &[T]) -> Result<(), ArrayError>
    where
        T: Clone,
    {
        let len = slice.len();
        if len > u32::MAX as usize {
            return Err(ArrayError::LengthLimitExceeded);
        }

        if unsafe { self.inner.length().as_usize().unchecked_add(len) } > self.inner.capacity().as_usize() {
            return Err(ArrayError::LengthLimitExceeded);
        }
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

impl<T, TAllocator> Drop for FixedArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    fn drop(&mut self) {
        unsafe { self.inner.deallocate() };
    }
}

impl<T: Clone, TAllocator> Clone for FixedArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone_perfect(),
        }
    }
}

impl<T, TAllocator, Rhs> PartialEq<Rhs> for FixedArray<T, TAllocator>
where
    T: PartialEq,
    TAllocator: Allocator,
    Rhs: AsRef<[T]>,
{
    fn eq(&self, other: &Rhs) -> bool {
        self.as_slice() == other.as_ref()
    }
}

impl<T, TAllocator> Eq for FixedArray<T, TAllocator>
where
    T: Eq,
    TAllocator: Allocator,
{
}

impl<T, TAllocator> Hash for FixedArray<T, TAllocator>
where
    T: Hash,
    TAllocator: Allocator,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

impl<T, TAllocator> AsRef<[T]> for FixedArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, TAllocator> AsMut<[T]> for FixedArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    fn as_mut(&mut self) -> &mut [T] {
        self.as_slice_mut()
    }
}

impl<T, TAllocator> Borrow<[T]> for FixedArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    fn borrow(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, TAllocator> BorrowMut<[T]> for FixedArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    fn borrow_mut(&mut self) -> &mut [T] {
        self.as_slice_mut()
    }
}
