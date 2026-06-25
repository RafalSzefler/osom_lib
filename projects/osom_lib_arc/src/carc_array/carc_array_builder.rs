use core::sync::atomic::Ordering;

use osom_lib_alloc::traits::Allocator;
use osom_lib_primitives::length::Length;
use osom_lib_reprc::traits::ReprC;

use crate::{
    carc_array::{CArcArray, internal::InternalArcArray},
    errors::CArcArrayError,
};

/// This is a builder for [`CArcArray`]. It is used to iteratively construct
/// a [`CArcArray`], without the need of intermediate allocations.
#[repr(transparent)]
#[must_use]
#[derive(Debug)]
pub struct CArcArrayBuilder<T, TAllocator: Allocator> {
    internal: InternalArcArray<T, TAllocator>,
}

unsafe impl<T: ReprC, TAllocator: Allocator> ReprC for CArcArrayBuilder<T, TAllocator> {
    const CHECK: () = const {
        osom_lib_reprc::hidden::is_reprc::<InternalArcArray<T, TAllocator>>();
    };
}

impl<T, TAllocator: Allocator> CArcArrayBuilder<T, TAllocator> {
    /// Creates a new [`CArcArrayBuilder`] with the default allocator.
    ///
    /// # Notes
    ///
    /// This function allocates memory.
    ///
    /// # Errors
    ///
    /// For details see [`CArcArrayError`].
    #[inline]
    pub fn new() -> Result<Self, CArcArrayError>
    where
        TAllocator: Default,
    {
        Self::with_capacity_and_allocator(Length::ZERO, TAllocator::default())
    }

    /// Creates a new [`CArcArrayBuilder`] with the given capacity and the default allocator.
    ///
    /// # Notes
    ///
    /// This function allocates memory.
    ///
    /// # Errors
    ///
    /// For details see [`CArcArrayError`].
    #[inline]
    pub fn with_capacity(capacity: Length) -> Result<Self, CArcArrayError>
    where
        TAllocator: Default,
    {
        Self::with_capacity_and_allocator(capacity, TAllocator::default())
    }

    /// Creates a new [`CArcArrayBuilder`] with the given capacity and allocator.
    ///
    /// # Notes
    ///
    /// This function allocates memory.
    ///
    /// # Errors
    ///
    /// For details see [`CArcArrayError`].
    #[inline]
    pub fn with_capacity_and_allocator(capacity: Length, allocator: TAllocator) -> Result<Self, CArcArrayError> {
        let internal = InternalArcArray::new(capacity, allocator)?;
        Ok(Self { internal })
    }

    /// Pushes a new slice to the [`CArcArrayBuilder`].
    ///
    /// # Notes
    ///
    /// This function clones the data.
    ///
    /// # Errors
    ///
    /// For details see [`CArcArrayError`].
    #[inline]
    pub fn try_push_slice(&mut self, slice: &[T]) -> Result<(), CArcArrayError>
    where
        T: Clone,
    {
        self.internal.try_push_slice(slice)
    }

    /// Pushes a new array to the [`CArcArrayBuilder`].
    ///
    /// # Notes
    ///
    /// This function moves the data.
    ///
    /// # Errors
    ///
    /// For details see [`CArcArrayError`].
    #[inline]
    pub fn try_push_array<const N: usize>(&mut self, array: [T; N]) -> Result<(), CArcArrayError> {
        self.internal.try_push_array(array)
    }

    /// Shrinks the underlying buffer to match the length of the buffer exactly.
    ///
    /// # Notes
    ///
    /// This function may reallocate the underlying buffer, depending on the
    /// allocator's behavior.
    ///
    /// # Errors
    ///
    /// For details see [`CArcArrayError`].
    #[inline]
    pub fn shrink_to_fit(&mut self) -> Result<(), CArcArrayError> {
        self.internal.shrink_to_fit()
    }

    /// Returns a reference to the underlying slice.
    #[inline]
    #[must_use]
    pub const fn data(&self) -> &[T] {
        self.internal.data_slice()
    }

    /// Returns a mutable reference to the underlying slice.
    #[inline]
    #[must_use]
    pub const fn data_mut(&mut self) -> &mut [T] {
        self.internal.data_slice_mut()
    }

    /// Returns the length of the underlying slice.
    #[inline]
    pub const fn length(&self) -> Length {
        self.internal.size()
    }

    /// Builds a new [`CArcArray`] out of the [`CArcArrayBuilder`].
    #[inline]
    pub fn build(self) -> CArcArray<T, TAllocator> {
        let internal = unsafe { core::ptr::read(&raw const self.internal) };
        core::mem::forget(self);
        internal.strong().store(1, Ordering::Relaxed);
        internal.weak().store(1, Ordering::Relaxed);
        CArcArray::from_internal(internal)
    }
}

impl<T, TAllocator: Allocator> Drop for CArcArrayBuilder<T, TAllocator> {
    fn drop(&mut self) {
        if core::mem::needs_drop::<T>() {
            for item in self.internal.data_slice_mut() {
                unsafe { core::ptr::drop_in_place(item) };
            }
        }
        unsafe { self.internal.deallocate_memory() };
    }
}
