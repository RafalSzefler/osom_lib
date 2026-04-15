//! Holds the implementation of the standard allocator, based on `libc` crate.

use core::{alloc::Layout, ptr::NonNull};

use osom_lib_reprc::macros::reprc;

use super::traits::{AllocationError, Allocator};

/// The standard allocator based on `libc` crate.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
#[reprc]
pub struct StdAllocator;

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

impl From<StdAllocationError> for AllocationError {
    fn from(_: StdAllocationError) -> Self {
        Self
    }
}

unsafe impl Allocator for StdAllocator {
    type SpecificAllocationError = StdAllocationError;

    fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, Self::SpecificAllocationError> {
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
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        raw_aligned_free(ptr.as_ptr().cast(), layout);
    }

    unsafe fn resize(
        &self,
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
                new_ptr.copy_from_nonoverlapping(ptr, new_layout.size());
                self.deallocate(ptr, old_layout);
                Ok(new_ptr)
            }
        }
    }
}

osom_lib_cfg_ext::cfg_match!(
    (miri) => {
        extern crate alloc;

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
    },
    (target_os="windows") => {
        #[inline(always)]
        fn raw_aligned_malloc(layout: Layout) -> *mut u8 {
            unsafe { ::libc::aligned_malloc(layout.size(), layout.align()).cast() }
        }

        #[inline(always)]
        fn raw_aligned_realloc(ptr: *mut u8, old_layout: Layout, new_size: usize) -> *mut u8 {
            unsafe { ::libc::aligned_realloc(ptr.cast(), new_size, old_layout.align()).cast() }
        }

        #[inline(always)]
        fn raw_aligned_free(ptr: *mut u8, _layout: Layout) {
            unsafe { ::libc::aligned_free(ptr.cast()) }
        }
    },
    _ => {
        #[inline(always)]
        fn raw_aligned_malloc(layout: Layout) -> *mut u8 {
            unsafe { ::libc::malloc(layout.size()).cast() }
        }

        #[inline(always)]
        fn raw_aligned_realloc(ptr: *mut u8, _old_layout: Layout, new_size: usize) -> *mut u8 {
            unsafe { ::libc::realloc(ptr.cast(), new_size).cast() }
        }

        #[inline(always)]
        fn raw_aligned_free(ptr: *mut u8, _layout: Layout) {
            unsafe { ::libc::free(ptr.cast()) }
        }
    }
);
