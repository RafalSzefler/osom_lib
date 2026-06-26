use core::sync::atomic::Ordering;

use osom_lib_alloc::traits::Allocator;
use osom_lib_primitives::length::Length;
use osom_lib_reprc::traits::ReprC;
use osom_lib_try_clone::TryClone;

use crate::errors::CArcArrayError;

use super::{CAlignedArcArray, internal::InternalAlignedArcArray};

/// This is a builder for [`CAlignedArcArray`]. It is used to iteratively construct
/// a [`CAlignedArcArray`], without the need of intermediate allocations.
#[repr(transparent)]
#[must_use]
#[derive(Debug)]
pub struct CAlignedArcArrayBuilder<TAlign, TItem, TAllocator: Allocator> {
    internal: InternalAlignedArcArray<TAlign, TItem, TAllocator>,
}

unsafe impl<TAlign, TItem: ReprC, TAllocator: Allocator> ReprC for CAlignedArcArrayBuilder<TAlign, TItem, TAllocator> {
    const CHECK: () = const {
        osom_lib_reprc::hidden::is_reprc::<InternalAlignedArcArray<TAlign, TItem, TAllocator>>();
    };
}

impl<TAlign, TItem, TAllocator: Allocator> CAlignedArcArrayBuilder<TAlign, TItem, TAllocator> {
    /// Creates a new [`CAlignedArcArrayBuilder`] with the default allocator.
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

    /// Creates a new [`CAlignedArcArrayBuilder`] with the given capacity and the default allocator.
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

    /// Creates a new [`CAlignedArcArrayBuilder`] with the given capacity and allocator.
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
        let internal = InternalAlignedArcArray::new(capacity, allocator)?;
        Ok(Self { internal })
    }

    /// Pushes a new slice to the [`CAlignedArcArrayBuilder`].
    ///
    /// # Notes
    ///
    /// This function clones the data.
    ///
    /// # Errors
    ///
    /// For details see [`CArcArrayError`].
    #[inline]
    pub fn try_push_slice(&mut self, slice: &[TItem]) -> Result<(), CArcArrayError>
    where
        TItem: TryClone,
    {
        self.internal.try_push_slice(slice)
    }

    /// Pushes a new array to the [`CAlignedArcArrayBuilder`].
    ///
    /// # Notes
    ///
    /// This function moves the data.
    ///
    /// # Errors
    ///
    /// For details see [`CArcArrayError`].
    #[inline]
    pub fn try_push_array<const N: usize>(&mut self, array: [TItem; N]) -> Result<(), CArcArrayError> {
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
    pub const fn data(&self) -> &[TItem] {
        self.internal.data_slice()
    }

    /// Returns a mutable reference to the underlying slice.
    #[inline]
    #[must_use]
    pub const fn data_mut(&mut self) -> &mut [TItem] {
        self.internal.data_slice_mut()
    }

    /// Returns the length of the underlying slice.
    #[inline]
    pub const fn length(&self) -> Length {
        self.internal.size()
    }

    /// Builds a new [`CAlignedArcArray`] out of the [`CAlignedArcArrayBuilder`].
    #[inline]
    pub fn build(self) -> CAlignedArcArray<TAlign, TItem, TAllocator> {
        let internal = unsafe { core::ptr::read(&raw const self.internal) };
        core::mem::forget(self);
        internal.strong().store(1, Ordering::Relaxed);
        internal.weak().store(1, Ordering::Relaxed);
        CAlignedArcArray::from_internal(internal)
    }
}

impl<TAlign, TItem, TAllocator: Allocator> Drop for CAlignedArcArrayBuilder<TAlign, TItem, TAllocator> {
    fn drop(&mut self) {
        if core::mem::needs_drop::<TItem>() {
            for item in self.internal.data_slice_mut() {
                unsafe { core::ptr::drop_in_place(item) };
            }
        }
        unsafe { self.internal.deallocate_memory() };
    }
}
