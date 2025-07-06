#![allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::cast_sign_loss)]

use core::sync::atomic::Ordering;

use osom_lib_alloc::{AllocationError, Allocator};
use osom_lib_primitives::Length;

#[cfg(feature = "std_alloc")]
use osom_lib_alloc::StdAllocator;

use super::ImmutableWeakArray;
use super::internal_array::{HeapData, InternalArray, MAX_LENGTH};

/// Represents an error that occurs when constructing new [`ImmutableArray`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[must_use]
#[repr(u8)]
pub enum ImmutableArrayConstructionError {
    /// The allocator failed to allocate memory.
    AllocationError,

    /// The passed array is too long, it exceeds [`MAX_LENGTH`][`ImmutableArray::MAX_LENGTH`].
    ArrayTooLong,
}

impl From<AllocationError> for ImmutableArrayConstructionError {
    fn from(_: AllocationError) -> Self {
        ImmutableArrayConstructionError::AllocationError
    }
}

/// A smart pointer to an immutable array. It tracks both strong and
/// weak references to the array, and is thread safe.
/// Therefore cloning of this struct is very cheap.
///
/// It is also immutable, and no changes to the array are
/// allowed, except for internal mutability of course.
#[must_use]
#[repr(transparent)]
pub struct ImmutableArray<
    T: Sized,
    #[cfg(feature = "std_alloc")] TAllocator = StdAllocator,
    #[cfg(not(feature = "std_alloc"))] TAllocator,
> where
    TAllocator: Allocator,
{
    internal: InternalArray<T, TAllocator>,
}

impl<T: Sized, TAllocator: Allocator> From<InternalArray<T, TAllocator>> for ImmutableArray<T, TAllocator> {
    fn from(internal: InternalArray<T, TAllocator>) -> Self {
        Self { internal }
    }
}

impl<T: Sized, TAllocator: Allocator> ImmutableArray<T, TAllocator> {
    /// The maximum length of an array that can be constructed.
    /// It is guaranteed that [`MAX_LENGTH`][`Self::MAX_LENGTH`] is less than [`i32::MAX`].
    pub const MAX_LENGTH: usize = MAX_LENGTH;

    /// Converts the [`ImmutableArray`] into a slice.
    #[inline(always)]
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        self.internal.as_slice()
    }

    /// Returns the length of the [`ImmutableArray`].
    #[inline(always)]
    pub const fn len(&self) -> Length {
        self.internal.len()
    }

    /// Returns `true` if the [`ImmutableArray`] is empty, `false` otherwise.
    #[inline(always)]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len().value() == 0
    }

    /// Returns the capacity of the [`ImmutableArray`].
    #[inline(always)]
    pub const fn capacity(&self) -> Length {
        self.internal.capacity()
    }

    /// Returns a reference to the allocator of the [`ImmutableArray`].
    #[inline(always)]
    pub const fn allocator(&self) -> &TAllocator {
        self.internal.allocator()
    }

    /// Creates a new weak reference out of the [`ImmutableArray`].
    #[must_use]
    pub fn downgrade(instance: &Self) -> ImmutableWeakArray<T, TAllocator> {
        instance
            .internal
            .heap_data()
            .weak_counter()
            .fetch_add(1, Ordering::SeqCst);
        let internal = instance.internal.clone();
        ImmutableWeakArray::from(internal)
    }

    /// Releases the strong reference.
    ///
    /// Returns a weak reference if this was the last strong reference.
    /// Otherwise, returns `None`.
    pub fn release(mut instance: Self) -> Option<ImmutableWeakArray<T, TAllocator>> {
        let result = instance.internal_release();
        core::mem::forget(instance);
        result
    }

    /// Returns the number of strong references to the string.
    #[must_use]
    pub fn strong_count(instance: &Self) -> usize {
        instance.internal.heap_data().strong_counter().load(Ordering::SeqCst) as usize
    }

    /// Returns the number of weak references to the string.
    #[must_use]
    pub fn weak_count(instance: &Self) -> usize {
        instance.internal.heap_data().weak_counter().load(Ordering::SeqCst) as usize
    }

    /// Returns `true` if the two [`ImmutableArray`] instances refer to the same memory.
    /// Otherwise, returns `false`. This is different from `==` comparison, which
    /// checks whether the content of two strings is the same, ragardless of whether
    /// they point to the same memory or not.
    #[inline(always)]
    #[must_use]
    pub fn ref_equal(left: &Self, right: &Self) -> bool {
        let left = core::ptr::from_ref(left.internal.heap_data());
        let right = core::ptr::from_ref(right.internal.heap_data());
        core::ptr::addr_eq(left, right)
    }

    pub(crate) fn internal_release(&mut self) -> Option<ImmutableWeakArray<T, TAllocator>> {
        let strong_counter = self
            .internal
            .heap_data()
            .strong_counter()
            .fetch_sub(1, Ordering::SeqCst);
        if strong_counter == 1 {
            let internal = unsafe { core::ptr::read(&self.internal) };
            Some(ImmutableWeakArray::from(internal))
        } else {
            None
        }
    }
}

impl<T: Sized, TAllocator: Allocator> ImmutableArray<T, TAllocator> {
    /// Constructs a new [`ImmutableArray`] from a slice with default allocator.
    /// It copies the slice into the new [`ImmutableArray`].
    ///
    /// # Errors
    ///
    /// For details see [`ImmutableArrayConstructionError`].
    pub fn from_array<const N: usize>(array: [T; N]) -> Result<Self, ImmutableArrayConstructionError> {
        Self::from_array_with_allocator(array, TAllocator::default())
    }

    /// Constructs a new [`ImmutableArray`] from a slice and an allocator.
    /// It copies the slice into the new [`ImmutableArray`].
    ///
    /// # Errors
    ///
    /// For details see [`ImmutableArrayConstructionError`].
    pub fn from_array_with_allocator<const N: usize>(
        array: [T; N],
        allocator: TAllocator,
    ) -> Result<Self, ImmutableArrayConstructionError> {
        let slice_len = array.len();
        if slice_len > Self::MAX_LENGTH {
            return Err(ImmutableArrayConstructionError::ArrayTooLong);
        }

        let slice_len = unsafe { Length::new_unchecked(slice_len as i32) };

        let mut internal: InternalArray<T, TAllocator> = InternalArray::allocate(slice_len, slice_len, allocator)?;

        unsafe {
            let ptr = internal.heap_data_mut().data().as_ptr();
            debug_assert!(ptr.is_aligned(), "Data pointer is not aligned.");
            ptr.copy_from_nonoverlapping(array.as_ptr(), slice_len.into());
            core::mem::forget(array);
        }

        {
            *internal.heap_data_mut().strong_counter_mut().get_mut() = 1;
            *internal.heap_data_mut().weak_counter_mut().get_mut() = 1;
        }

        Ok(Self { internal })
    }
}

impl<T: Sized + Clone, TAllocator: Allocator> ImmutableArray<T, TAllocator> {
    /// Constructs a new [`ImmutableArray`] from a slice with default allocator.
    /// It clones the slice into the new [`ImmutableArray`].
    ///
    /// # Errors
    ///
    /// For details see [`ImmutableArrayConstructionError`].
    pub fn from_slice(slice: &[T]) -> Result<Self, ImmutableArrayConstructionError> {
        Self::from_slice_with_allocator(slice, TAllocator::default())
    }

    /// Constructs a new [`ImmutableArray`] from a slice and an allocator.
    /// It clones the slice into the new [`ImmutableArray`].
    ///
    /// # Errors
    ///
    /// For details see [`ImmutableArrayConstructionError`].
    pub fn from_slice_with_allocator(
        slice: &[T],
        allocator: TAllocator,
    ) -> Result<Self, ImmutableArrayConstructionError> {
        let slice_len = slice.len();
        if slice_len > Self::MAX_LENGTH {
            return Err(ImmutableArrayConstructionError::ArrayTooLong);
        }

        let slice_len = unsafe { Length::new_unchecked(slice_len as i32) };

        let mut internal: InternalArray<T, TAllocator> = InternalArray::allocate(slice_len, slice_len, allocator)?;

        unsafe {
            let mut ptr = internal.heap_data_mut().data().as_ptr();
            debug_assert!(ptr.is_aligned(), "Data pointer is not aligned.");

            for item in slice {
                ptr.write(item.clone());
                ptr = ptr.add(1);
            }
        }

        {
            *internal.heap_data_mut().strong_counter_mut().get_mut() = 1;
            *internal.heap_data_mut().weak_counter_mut().get_mut() = 1;
        }

        Ok(Self { internal })
    }
}

impl<T: Sized, TAllocator: Allocator> Drop for ImmutableArray<T, TAllocator> {
    fn drop(&mut self) {
        self.internal_release();
    }
}

impl<T: Sized, TAllocator: Allocator> Clone for ImmutableArray<T, TAllocator> {
    fn clone(&self) -> Self {
        self.internal
            .heap_data()
            .strong_counter()
            .fetch_add(1, Ordering::SeqCst);
        Self {
            internal: self.internal.clone(),
        }
    }
}

impl<T: Sized + PartialEq, TAllocator1: Allocator, TAllocator2: Allocator> PartialEq<ImmutableArray<T, TAllocator1>>
    for ImmutableArray<T, TAllocator2>
{
    fn eq(&self, other: &ImmutableArray<T, TAllocator1>) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T: Sized + Eq, TAllocator: Allocator> Eq for ImmutableArray<T, TAllocator> {}

impl<T: Sized + core::hash::Hash, TAllocator: Allocator> core::hash::Hash for ImmutableArray<T, TAllocator> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

impl<T: Sized, TAllocator: Allocator> core::fmt::Debug for ImmutableArray<T, TAllocator> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let ptr = core::ptr::from_ref::<HeapData<T>>(self.internal.heap_data());
        f.debug_struct("ImmutableArray")
            .field("strong_count", &Self::strong_count(self))
            .field("weak_count", &Self::weak_count(self))
            .field("len", &self.len())
            .field("capacity", &self.internal.capacity())
            .field("raw_ptr", &ptr.addr())
            .finish()
    }
}

impl<T: Sized, TAllocator: Allocator> core::ops::Deref for ImmutableArray<T, TAllocator> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T: Sized, TAllocator: Allocator> AsRef<[T]> for ImmutableArray<T, TAllocator> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}
