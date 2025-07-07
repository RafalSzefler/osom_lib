#![allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::cast_sign_loss)]

use osom_lib_alloc::Allocator;
use osom_lib_primitives::Length;

#[cfg(feature = "std_alloc")]
use osom_lib_alloc::StdAllocator;

use crate::errors::ArrayConstructionError;

use super::internal_array::{InternalArray, MAX_LENGTH};
use super::{ImmutableArray, ImmutableWeakArray};

const INITIAL_CAPACITY: Length = unsafe { Length::new_unchecked(16) };

/// A builder for [`ImmutableArray`][`super::ImmutableArray`].
///
/// # Notes
///
/// This struct is very similar to a mutable vec. It is used to construct
/// [`ImmutableArray`][`super::ImmutableArray`] incrementally, in place.
pub struct ImmutableArrayBuilder<
    T: Sized,
    #[cfg(feature = "std_alloc")] TAllocator = StdAllocator,
    #[cfg(not(feature = "std_alloc"))] TAllocator,
> where
    TAllocator: Allocator,
{
    internal: InternalArray<T, TAllocator>,
}

impl<T: Sized, TAllocator: Allocator> ImmutableArrayBuilder<T, TAllocator> {
    #[inline(always)]
    const fn grow_formula(current: usize) -> Length {
        unsafe { Length::new_unchecked((3 * (current / 2)) as i32) }
    }

    /// Shrinks current capacity to the length.
    ///
    /// # Notes
    ///
    /// This method reallocates entire buffer if the current capacity is greater than the length.
    ///
    /// # Errors
    ///
    /// For details see [`ArrayConstructionError`].
    pub fn shrink_to_fit(&mut self) -> Result<(), ArrayConstructionError> {
        let internal = &mut self.internal;
        let internal_len = internal.len();
        let internal_capacity = internal.capacity();
        if internal_len == internal_capacity {
            return Ok(());
        }

        let mut new_internal: InternalArray<T, TAllocator> =
            InternalArray::allocate(internal_len, internal_len, internal.allocator().clone())?;
        let heap_data = new_internal.heap_data_mut();
        let data_ptr = heap_data.data().as_ptr();
        unsafe {
            data_ptr.copy_from_nonoverlapping(internal.heap_data().data().as_ptr(), internal_len.value() as usize);
        }
        self.internal = new_internal;
        Ok(())
    }

    /// Builds the [`ImmutableArray`] from the builder.
    ///
    /// # Notes
    ///
    /// This method is basically free. It is here only to move
    /// ownership from mutable builder to immutable array.
    #[inline(always)]
    pub fn build(self) -> ImmutableArray<T, TAllocator> {
        let internal = unsafe { core::ptr::read(&self.internal) };
        core::mem::forget(self);
        ImmutableArray::from(internal)
    }

    /// Creates a new builder with the default allocator.
    ///
    /// # Errors
    ///
    /// For details see [`ArrayConstructionError`].
    #[inline(always)]
    pub fn new() -> Result<Self, ArrayConstructionError> {
        Self::with_allocator(TAllocator::default())
    }

    /// Creates a new builder with the specified allocator.
    ///
    /// # Errors
    ///
    /// For details see [`ArrayConstructionError`].
    #[inline(always)]
    pub fn with_allocator(allocator: TAllocator) -> Result<Self, ArrayConstructionError> {
        let internal = InternalArray::allocate(Length::ZERO, INITIAL_CAPACITY, allocator)?;
        Ok(Self { internal })
    }

    /// Pushes a new value to the end of the builder.
    ///
    /// # Errors
    ///
    /// For details see [`ArrayConstructionError`].
    #[inline(always)]
    pub fn push(&mut self, value: T) -> Result<(), ArrayConstructionError> {
        self.extend_from_array([value])
    }

    /// Extends the builder with the specified values.
    ///
    /// # Errors
    ///
    /// For details see [`ArrayConstructionError`].
    pub fn extend_from_array<const N: usize>(&mut self, values: [T; N]) -> Result<(), ArrayConstructionError> {
        if N == 0 {
            return Ok(());
        }

        if N > MAX_LENGTH {
            return Err(ArrayConstructionError::ArrayTooLong);
        }

        let internal = &mut self.internal;
        let internal_len = internal.len().value() as usize;
        let internal_capacity = internal.capacity().value() as usize;
        if internal_len + N > internal_capacity {
            let missing = internal_len + N - internal_capacity;
            let new_capacity = Self::grow_formula(internal_capacity + missing);
            internal.grow(new_capacity)?;
        }

        let heap_data = internal.heap_data_mut();
        let data_ptr = heap_data.data().as_ptr();
        unsafe {
            let end_ptr = data_ptr.add(internal_len);
            end_ptr.copy_from_nonoverlapping(values.as_ptr(), N);
        }
        core::mem::forget(values);
        *internal.len_mut() += N as i32;
        Ok(())
    }

    #[inline(always)]
    pub const fn len(&self) -> Length {
        self.internal.len()
    }

    #[inline(always)]
    pub const fn capacity(&self) -> Length {
        self.internal.capacity()
    }
}

impl<T: Sized, TAllocator: Allocator> Drop for ImmutableArrayBuilder<T, TAllocator> {
    fn drop(&mut self) {
        // We still need drop, in case someone crates builder but does not actually
        // call `build` method. Note that the `build` method disables drop.
        let internal = unsafe { core::ptr::read(&self.internal) };
        let _ = ImmutableWeakArray::from(internal);
    }
}

impl<T: Sized + Clone, TAllocator: Allocator> ImmutableArrayBuilder<T, TAllocator> {
    /// Extends the builder with the specified slice.
    ///
    /// # Notes
    ///
    /// This method will clone each element one by one.
    ///
    /// # Errors
    ///
    /// For details see [`ArrayConstructionError`].
    #[inline(always)]
    pub fn extend_from_slice(&mut self, slice: &[T]) -> Result<(), ArrayConstructionError> {
        let slice_len = slice.len();
        if slice_len == 0 {
            return Ok(());
        }

        if slice_len > MAX_LENGTH {
            return Err(ArrayConstructionError::ArrayTooLong);
        }

        let internal = &mut self.internal;
        let internal_len = internal.len().value() as usize;
        let internal_capacity = internal.capacity().value() as usize;
        if internal_len + slice_len > internal_capacity {
            let missing = internal_len + slice_len - internal_capacity;
            let new_capacity = Self::grow_formula(internal_capacity + missing);
            internal.grow(new_capacity)?;
        }

        let heap_data = internal.heap_data_mut();
        let data_ptr = heap_data.data().as_ptr();
        unsafe {
            let mut end_ptr = data_ptr.add(internal_len);
            for item in slice {
                end_ptr.write(item.clone());
                end_ptr = end_ptr.add(1);
            }
        }
        *internal.len_mut() += slice_len as i32;
        Ok(())
    }
}
