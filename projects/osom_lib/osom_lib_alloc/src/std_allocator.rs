//! Holds the implementation of the standard allocator, based on `alloc` crate.
extern crate alloc;

use core::{alloc::Layout, convert::Infallible, ptr::NonNull};

use osom_lib_reprc::macros::reprc;
use osom_lib_try_clone::TryClone;

use super::traits::{Allocator, WithMessage};

/// The standard allocator based on `alloc` crate.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
#[reprc]
pub struct StdAllocator;

impl TryClone for StdAllocator {
    type Error = Infallible;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        Ok(*self)
    }
}

/// The standard allocator error.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[reprc]
pub enum StdAllocationError {
    /// A generic allocation error. Likely because of out of memory.
    AllocationError = 0,

    /// The result is misaligned. Likely because the requested alignment is above
    /// what malloc supports.
    MisalignedResult = 1,
}

impl WithMessage for StdAllocationError {
    fn message(&self) -> &str {
        match self {
            StdAllocationError::AllocationError => "Allocation failed",
            StdAllocationError::MisalignedResult => "Misaligned result",
        }
    }
}

impl TryClone for StdAllocationError {
    type Error = Infallible;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        Ok(*self)
    }
}

unsafe impl Allocator for StdAllocator {
    type SpecificAllocationError = StdAllocationError;

    fn allocate(&mut self, layout: Layout) -> Result<NonNull<u8>, Self::SpecificAllocationError> {
        let raw_ptr = raw_aligned_malloc(layout);
        if raw_ptr.is_null() {
            return Err(StdAllocationError::AllocationError);
        }

        if raw_ptr.align_offset(layout.align()) != 0 {
            raw_aligned_free(raw_ptr.cast(), layout);
            return Err(StdAllocationError::MisalignedResult);
        }

        Ok(unsafe { NonNull::new_unchecked(raw_ptr.cast()) })
    }

    #[inline(always)]
    unsafe fn deallocate(&mut self, ptr: NonNull<u8>, layout: Layout) {
        raw_aligned_free(ptr.as_ptr().cast(), layout);
    }

    unsafe fn resize(
        &mut self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<u8>, Self::SpecificAllocationError> {
        let new_align = new_layout.align();
        if old_layout.align() == new_align {
            let raw_ptr = raw_aligned_realloc(ptr.as_ptr().cast(), old_layout, new_layout.size());
            if raw_ptr.is_null() {
                return Err(StdAllocationError::AllocationError);
            }

            if raw_ptr.align_offset(new_align) != 0 {
                raw_aligned_free(raw_ptr.cast(), new_layout);
                return Err(StdAllocationError::MisalignedResult);
            }

            Ok(unsafe { NonNull::new_unchecked(raw_ptr.cast()) })
        } else {
            unsafe {
                let new_ptr = self.allocate(new_layout)?;
                let size = core::cmp::min(new_layout.size(), old_layout.size());
                new_ptr.copy_from_nonoverlapping(ptr, size);
                self.deallocate(ptr, old_layout);
                Ok(new_ptr)
            }
        }
    }
}

#[inline(always)]
fn raw_aligned_malloc(layout: Layout) -> *mut u8 {
    unsafe { alloc::alloc::alloc(layout) }
}

#[inline(always)]
fn raw_aligned_realloc(ptr: *mut u8, old_layout: Layout, new_size: usize) -> *mut u8 {
    unsafe { alloc::alloc::realloc(ptr, old_layout, new_size) }
}

#[inline(always)]
fn raw_aligned_free(ptr: *mut u8, layout: Layout) {
    unsafe { alloc::alloc::dealloc(ptr, layout) }
}
