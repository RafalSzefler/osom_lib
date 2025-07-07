#![allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]

use core::{alloc::Layout, marker::PhantomData, ops::Deref};

use osom_lib_alloc::{AllocatedMemory as _, AllocationError, Allocator};

#[cfg(feature = "std_alloc")]
use osom_lib_alloc::StdAllocator;

use osom_lib_primitives::Length;

/// Represents an error that occurs when constructing new [`Array`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[must_use]
#[repr(u8)]
pub enum ArrayConstructionError {
    /// The allocator failed to allocate memory.
    AllocationError,

    /// The passed array is too long, it exceeds [`MAX_LENGTH`][`Array::MAX_LENGTH`].
    ArrayTooLong,
}

impl From<AllocationError> for ArrayConstructionError {
    fn from(_: AllocationError) -> Self {
        ArrayConstructionError::AllocationError
    }
}

/// Represents a fixed-size array but allocated on the heap.
///
/// In its essence very similar to [`DynamicArray`][`crate::DynamicArray`]
/// except it cannot grow or shrink in size. The content is still mutable though.
///
/// Cloning of [`Array`] will allocate new memory and copy the data into it.
#[must_use]
pub struct Array<
    T,
    #[cfg(feature = "std_alloc")] TAllocator = StdAllocator,
    #[cfg(not(feature = "std_alloc"))] TAllocator,
> where
    TAllocator: Allocator,
{
    pub(crate) data: TAllocator::TAllocatedMemory,
    pub(crate) len: Length,
    pub(crate) allocator: TAllocator,
    pub(crate) phantom: PhantomData<T>,
}

impl<T, TAllocator> Array<T, TAllocator>
where
    TAllocator: Allocator,
{
    pub const MAX_LENGTH: usize = Length::MAX;

    #[inline(always)]
    const fn layout(len: usize) -> Layout {
        let byte_size = len * size_of::<T>();
        let alignment = align_of::<T>();
        unsafe { Layout::from_size_align_unchecked(byte_size, alignment) }
    }

    /// Creates a new empty [`Array`].
    #[inline(always)]
    pub fn empty() -> Self {
        Self::empty_with_allocator(TAllocator::default())
    }

    /// Creates a new empty [`Array`] with the given allocator.
    ///
    ///
    /// # Notes
    /// The allocator won't matter in the case of empty array.
    /// Neither allocation nor deallocation will happen in this case.
    /// We keep this method for consistency only.
    #[inline(always)]
    pub fn empty_with_allocator(allocator: TAllocator) -> Self {
        Self {
            data: unsafe { allocator.dangling::<T>() },
            len: Length::ZERO,
            allocator: allocator,
            phantom: PhantomData,
        }
    }

    /// Creates a new [`Array`] from a fixed-size array and allocator.
    ///
    /// # Notes
    ///
    /// This will allocate memory if `N > 0` and will copy the data
    /// into it. It will move ownership as well.
    ///
    /// # Errors
    ///
    /// For details see [`ArrayConstructionError`].
    pub fn from_array_with_allocator<const N: usize>(
        data: [T; N],
        allocator: TAllocator,
    ) -> Result<Self, ArrayConstructionError> {
        if N == 0 {
            return Ok(Self::empty_with_allocator(allocator));
        }

        if N > Length::MAX {
            return Err(ArrayConstructionError::ArrayTooLong);
        }

        let layout = Self::layout(N);
        let memory = allocator.allocate(layout)?;

        let array = Self {
            data: memory,
            len: unsafe { Length::new_unchecked(N as i32) },
            allocator: allocator,
            phantom: PhantomData,
        };

        let slice_ptr = data.as_ptr();
        unsafe {
            array.ptr().copy_from_nonoverlapping(slice_ptr, N);
        }

        core::mem::forget(data);

        Ok(array)
    }

    /// Creates a new [`Array`] from a fixed-size array.
    ///
    /// # Notes
    ///
    /// This will allocate memory if `N > 0` and will copy the data
    /// into it. It will move ownership as well.
    ///
    /// # Errors
    ///
    /// For details see [`ArrayConstructionError`].
    pub fn from_array<const N: usize>(data: [T; N]) -> Result<Self, ArrayConstructionError> {
        Self::from_array_with_allocator(data, TAllocator::default())
    }

    /// Returns the length of the [`Array`].
    #[inline(always)]
    pub const fn len(&self) -> Length {
        self.len
    }

    /// Returns `true` if the [`Array`] is empty, `false` otherwise.
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.len.value() == 0
    }

    /// Converts the [`Array`] into a slice.
    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(self.ptr(), self.len.into()) }
    }

    /// Converts the [`Array`] into a mutable slice.
    #[inline(always)]
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        unsafe { core::slice::from_raw_parts_mut(self.ptr(), self.len.into()) }
    }

    #[inline(always)]
    fn ptr(&self) -> *mut T {
        unsafe { self.data.as_ptr() }
    }
}

impl<T, TAllocator> Array<T, TAllocator>
where
    T: Clone,
    TAllocator: Allocator,
{
    /// Creates a new [`Array`] from a slice and allocator.
    ///
    /// # Notes
    ///
    /// This will allocate memory if `len > 0` and will copy the data
    /// into it. It will move ownership as well.
    ///
    /// # Errors
    ///
    /// For details see [`ArrayConstructionError`].
    pub fn from_slice_and_allocator(slice: &[T], allocator: TAllocator) -> Result<Self, ArrayConstructionError> {
        let len = slice.len();
        if len == 0 {
            return Ok(Self::empty_with_allocator(allocator));
        }

        if len > Length::MAX {
            return Err(ArrayConstructionError::ArrayTooLong);
        }

        let layout = Self::layout(len);
        let memory = allocator.allocate(layout)?;

        let array = Self {
            data: memory,
            len: unsafe { Length::new_unchecked(len as i32) },
            allocator: allocator,
            phantom: PhantomData,
        };

        unsafe {
            let mut target = array.ptr();
            for item in slice {
                target.write(item.clone());
                target = target.add(1);
            }
        }

        Ok(array)
    }

    /// Creates a new [`Array`] from a slice.
    ///
    /// # Notes
    ///
    /// This will allocate memory if `len > 0` and will copy the data
    /// into it. It will move ownership as well.
    ///
    /// # Errors
    ///
    /// For details see [`ArrayConstructionError`].
    pub fn from_slice(slice: &[T]) -> Result<Self, ArrayConstructionError> {
        Self::from_slice_and_allocator(slice, TAllocator::default())
    }
}

impl<T, TAllocator> Drop for Array<T, TAllocator>
where
    TAllocator: Allocator,
{
    fn drop(&mut self) {
        let len: usize = self.len.into();
        if len == 0 {
            return;
        }

        unsafe {
            if core::mem::needs_drop::<T>() {
                let mut ptr = self.ptr();
                let end = ptr.add(len);
                while ptr < end {
                    core::ptr::drop_in_place(ptr);
                    ptr = ptr.add(1);
                }
            }

            let layout = Self::layout(len);
            let data = core::ptr::read(&self.data);
            data.deallocate(layout);
        }
    }
}

impl<T, TAllocator> Clone for Array<T, TAllocator>
where
    T: Clone,
    TAllocator: Allocator,
{
    fn clone(&self) -> Self {
        Self::from_slice_and_allocator(self.as_slice(), self.allocator.clone()).unwrap()
    }
}

impl<T, TAllocator> Default for Array<T, TAllocator>
where
    TAllocator: Allocator,
{
    fn default() -> Self {
        Self::empty()
    }
}

impl<T, TAllocator> Deref for Array<T, TAllocator>
where
    TAllocator: Allocator,
{
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T, TAllocator> core::ops::DerefMut for Array<T, TAllocator>
where
    TAllocator: Allocator,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_slice_mut()
    }
}

impl<T, TAllocator> AsRef<[T]> for Array<T, TAllocator>
where
    TAllocator: Allocator,
{
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, TAllocator> AsMut<[T]> for Array<T, TAllocator>
where
    TAllocator: Allocator,
{
    fn as_mut(&mut self) -> &mut [T] {
        self.as_slice_mut()
    }
}
