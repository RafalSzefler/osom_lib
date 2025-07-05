#![allow(clippy::cast_sign_loss, clippy::cast_possible_truncation, clippy::cast_possible_wrap)]

use core::{alloc::Layout, marker::PhantomData, ops::Deref};

use osom_lib_alloc::{AllocatedMemory as _, AllocationError, Allocator};
use osom_lib_primitives::Length;

#[cfg(feature = "std_alloc")]
use osom_lib_alloc::StdAllocator;

/// Represents an error that occurs when constructing new [`DynamicArray`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[must_use]
#[repr(u8)]
pub enum DynamicArrayConstructionError {
    /// The allocator failed to allocate memory.
    AllocationError,

    /// The passed array is too long, it exceeds [`MAX_LENGTH`][`DynamicArray::MAX_LENGTH`].
    ArrayTooLong,
}

impl From<AllocationError> for DynamicArrayConstructionError {
    fn from(_: AllocationError) -> Self {
        DynamicArrayConstructionError::AllocationError
    }
}

/// A dynamic array that grows when inserting elements. Similar to
/// `std::vec::Vec` in its nature.
///
/// The main differences are:
/// * It has slightly different growth rate, new capacity is equal to
///   3/2 of the old capacity.
/// * It allows plugging in custom allocators.
#[must_use]
pub struct DynamicArray<
    T,
    #[cfg(feature = "std_alloc")] TAllocator = StdAllocator,
    #[cfg(not(feature = "std_alloc"))] TAllocator,
> where
    TAllocator: Allocator,
{
    ptr: TAllocator::TAllocatedMemory,
    length: Length,
    capacity: Length,
    allocator: TAllocator,
    phantom: PhantomData<T>,
}

impl<T, TAllocator: Allocator> DynamicArray<T, TAllocator> {
    const fn grow_formula(current: i32) -> Length {
        unsafe { Length::new_unchecked((3 * (current / 2)) + 2) }
    }

    pub const MAX_LENGTH: usize = Length::MAX;

    /// Creates a new empty [`DynamicArray`] with the default allocator.
    #[inline(always)]
    pub fn new() -> Self {
        Self::with_allocator(TAllocator::default())
    }

    /// Creates a new empty [`DynamicArray`] with the given allocator.
    #[inline(always)]
    pub fn with_allocator(allocator: TAllocator) -> Self {
        Self {
            ptr: unsafe { allocator.dangling::<T>() },
            length: Length::ZERO,
            capacity: Length::ZERO,
            allocator: allocator,
            phantom: PhantomData,
        }
    }

    /// Creates a new empty [`DynamicArray`] with the given capacity.
    ///
    /// # Errors
    ///
    /// For details see [`DynamicArrayConstructionError`].
    #[inline(always)]
    pub fn with_capacity(capacity: Length) -> Result<Self, DynamicArrayConstructionError> {
        Self::with_capacity_and_allocator(capacity, TAllocator::default())
    }

    /// Creates a new empty [`DynamicArray`] with the given capacity and allocator.
    ///
    /// # Errors
    ///
    /// For details see [`DynamicArrayConstructionError`].
    #[inline(always)]
    pub fn with_capacity_and_allocator(
        capacity: Length,
        allocator: TAllocator,
    ) -> Result<Self, DynamicArrayConstructionError> {
        if capacity == Length::ZERO {
            return Ok(Self::with_allocator(allocator));
        }

        let mut new_array = Self::with_allocator(allocator);
        new_array.grow(capacity)?;
        Ok(new_array)
    }

    /// Returns the length of the [`DynamicArray`].
    #[inline(always)]
    pub const fn len(&self) -> Length {
        self.length
    }

    /// Returns `true` if the [`DynamicArray`] is empty, `false` otherwise.
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.length.value() == 0
    }

    /// Returns the capacity of the [`DynamicArray`].
    #[inline(always)]
    pub const fn capacity(&self) -> Length {
        self.capacity
    }

    /// Returns a reference to the allocator of the [`DynamicArray`].
    #[inline(always)]
    pub const fn allocator(&self) -> &TAllocator {
        &self.allocator
    }

    /// Represents the [`DynamicArray`] as a slice.
    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        let ptr = self.data_ptr();
        let len: usize = self.length.into();
        debug_assert!(ptr.is_aligned(), "Data pointer is not aligned correctly.");
        debug_assert!(len <= Self::MAX_LENGTH, "Length is too long.");
        unsafe { core::slice::from_raw_parts(ptr, len) }
    }

    /// Represents the [`DynamicArray`] as a mutable slice.
    #[inline(always)]
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        let ptr = self.data_ptr();
        let len: usize = self.length.into();
        debug_assert!(ptr.is_aligned(), "Data pointer is not aligned correctly.");
        debug_assert!(len <= Self::MAX_LENGTH, "Length is too long.");
        unsafe { core::slice::from_raw_parts_mut(ptr, len) }
    }

    /// Pushes a new element to the end of the [`DynamicArray`].
    ///
    /// # Errors
    ///
    /// For details see [`DynamicArrayConstructionError`].
    #[inline(always)]
    pub fn push(&mut self, value: T) -> Result<(), DynamicArrayConstructionError> {
        self.extend_from_array([value])
    }

    /// Extends the [`DynamicArray`] with the given array.
    ///
    /// # Errors
    ///
    /// For details see [`DynamicArrayConstructionError`].
    #[inline(always)]
    pub fn extend_from_array<const N: usize>(&mut self, other: [T; N]) -> Result<(), DynamicArrayConstructionError> {
        if N == 0 {
            return Ok(());
        }

        if N > Self::MAX_LENGTH {
            return Err(DynamicArrayConstructionError::ArrayTooLong);
        }

        let len = self.length.value() as usize;
        let capacity = self.capacity.value() as usize;

        if len + N > capacity {
            let missing = len + N - capacity;
            let at_least = (capacity + missing) as i32;
            let new_capacity = Self::grow_formula(at_least);
            self.grow(new_capacity)?;
        }

        let ptr = self.data_ptr();
        unsafe {
            let end_ptr = ptr.add(len);
            end_ptr.copy_from_nonoverlapping(other.as_ptr(), N);
            core::mem::forget(other);
        }

        self.length += N as i32;
        Ok(())
    }

    /// Pops last element from the [`DynamicArray`],
    /// decreasing its size.
    ///
    /// # Returns
    ///
    /// * `Some(T)` if `self.len() > 0`
    /// * `None` otherwise
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(unsafe { self.pop_unchecked() })
        }
    }

    /// Unsafe variant of [`pop`][`Self::pop`].
    ///
    /// # Safety
    ///
    /// Returns `T` if `self.len() > 0` and decreases the
    /// [`DynamicArray`] size. The behaviour is undefined if
    /// `self.len() == 0`.
    #[inline]
    pub unsafe fn pop_unchecked(&mut self) -> T {
        debug_assert!(!self.is_empty(), "Tried pop_unchecked on length 0 DynamicArray.");
        unsafe {
            let ptr = self.data_ptr();
            self.length -= 1;
            ptr.add(self.length.into()).read()
        }
    }

    #[inline(always)]
    fn data_ptr(&self) -> *mut T {
        unsafe { self.ptr.as_ptr() }
    }

    #[inline(always)]
    const fn layout(size: usize) -> Layout {
        let align = align_of::<T>();
        let byte_size = size * size_of::<T>();
        unsafe { Layout::from_size_align_unchecked(byte_size, align) }
    }

    fn grow(&mut self, new_capacity: Length) -> Result<(), AllocationError> {
        assert!(
            new_capacity > self.capacity,
            "New capacity is less than or equal to the current capacity."
        );
        let new_layout = Self::layout(new_capacity.into());
        let new_ptr = if self.capacity == Length::ZERO {
            self.allocator.allocate(new_layout)
        } else {
            let old_layout = Self::layout(self.capacity.into());
            self.ptr.clone().resize(old_layout, new_layout)
        }?;

        let data_ptr: *mut T = unsafe { new_ptr.as_ptr() };
        assert!(
            data_ptr.is_aligned(),
            "Newly allocated memory is not aligned correctly."
        );
        self.ptr = new_ptr;
        self.capacity = new_capacity;
        Ok(())
    }
}

impl<T, TAllocator: Allocator> Drop for DynamicArray<T, TAllocator> {
    fn drop(&mut self) {
        if self.capacity == Length::ZERO {
            return;
        }

        if core::mem::needs_drop::<T>() {
            unsafe {
                let mut ptr: *mut T = self.ptr.as_ptr();
                let len = self.length.into();
                let end = ptr.add(len);
                while ptr < end {
                    drop(ptr.read());
                    ptr = ptr.add(1);
                }
            }
        }
        let layout = Self::layout(self.capacity.into());
        self.ptr.clone().deallocate(layout);
    }
}

impl<T: Clone, TAllocator: Allocator> DynamicArray<T, TAllocator> {
    /// Extends the [`DynamicArray`] with the given slice. It copies
    /// slice's elements one by one.
    ///
    /// # Errors
    ///
    /// For details see [`DynamicArrayConstructionError`].
    #[inline(always)]
    pub fn extend_from_slice(&mut self, slice: &[T]) -> Result<(), DynamicArrayConstructionError> {
        let slice_len = slice.len();
        if slice_len == 0 {
            return Ok(());
        }

        if slice_len > Self::MAX_LENGTH {
            return Err(DynamicArrayConstructionError::ArrayTooLong);
        }

        let len = self.length.value() as usize;
        let capacity = self.capacity.value() as usize;

        if len + slice_len > capacity {
            let missing = len + slice_len - capacity;
            let at_least = (capacity + missing) as i32;
            let new_capacity = Self::grow_formula(at_least);
            self.grow(new_capacity)?;
        }

        let ptr = self.data_ptr();
        unsafe {
            let mut end_ptr = ptr.add(len);
            for value in slice {
                end_ptr.write(value.clone());
                end_ptr = end_ptr.add(1);
            }
        }

        self.length += slice_len as i32;
        Ok(())
    }

    /// Tries to clone the [`DynamicArray`].
    ///
    /// # Errors
    ///
    /// For details see [`DynamicArrayConstructionError`].
    #[inline(always)]
    pub fn try_clone(&self) -> Result<Self, DynamicArrayConstructionError> {
        let mut new_array = Self::with_capacity_and_allocator(self.capacity, self.allocator.clone())?;
        new_array.extend_from_slice(self.as_slice())?;
        Ok(new_array)
    }
}

impl<T: Clone, TAllocator: Allocator> Clone for DynamicArray<T, TAllocator> {
    fn clone(&self) -> Self {
        self.try_clone().expect("Failed to clone the array")
    }
}

impl<T: PartialEq, TAllocator1: Allocator, TAllocator2: Allocator> PartialEq<DynamicArray<T, TAllocator1>>
    for DynamicArray<T, TAllocator2>
{
    fn eq(&self, other: &DynamicArray<T, TAllocator1>) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T: Eq, TAllocator: Allocator> Eq for DynamicArray<T, TAllocator> {}

impl<T: core::hash::Hash, TAllocator: Allocator> core::hash::Hash for DynamicArray<T, TAllocator> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

impl<T, TAllocator: Allocator> Deref for DynamicArray<T, TAllocator> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T, TAllocator: Allocator> core::ops::DerefMut for DynamicArray<T, TAllocator> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_slice_mut()
    }
}

#[allow(clippy::missing_fields_in_debug)]
impl<T, TAllocator: Allocator> core::fmt::Debug for DynamicArray<T, TAllocator> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DynamicArray")
            .field("raw_ptr", &self.data_ptr().addr())
            .field("length", &self.length)
            .field("capacity", &self.capacity)
            .finish()
    }
}

impl<T, TAllocator: Allocator> Default for DynamicArray<T, TAllocator> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, TAllocator: Allocator> AsRef<[T]> for DynamicArray<T, TAllocator> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

unsafe impl<T: Send, TAllocator: Allocator> Send for DynamicArray<T, TAllocator> {}
unsafe impl<T: Sync, TAllocator: Allocator> Sync for DynamicArray<T, TAllocator> {}
