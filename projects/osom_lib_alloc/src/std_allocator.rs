use alloc::alloc as std_alloc;

use core::{alloc::Layout, ptr::{dangling_mut, NonNull}};

use super::{AllocationError, Allocator};

/// Represents the default allocator taken from the standard Rust library.
#[derive(Clone, Default, Debug)]
#[repr(C)]
#[must_use]
pub struct StdAllocator;

unsafe impl Allocator for StdAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, AllocationError> {
        let new_ptr = unsafe { std_alloc::alloc(layout) };
        if new_ptr.is_null() {
            return Err(AllocationError);
        }
        Ok(unsafe { NonNull::new_unchecked(new_ptr) })
    }

    #[inline(always)]
    unsafe fn dangling<T: Sized>(&self) -> NonNull<T> {
        let dangling_ptr = dangling_mut::<T>();
        debug_assert!(!dangling_ptr.is_null());
        debug_assert!(dangling_ptr.cast::<T>().is_aligned());
        unsafe { NonNull::new_unchecked(dangling_ptr) }
    }
    
    unsafe fn resize(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<u8>, AllocationError> {
        let ptr = ptr.as_ptr();
        let new_ptr = if old_layout.align() == new_layout.align() {
            let new_ptr = unsafe { std_alloc::realloc(ptr, old_layout, new_layout.size()) };
            if new_ptr.is_null() {
                return Err(AllocationError);
            } 
            new_ptr
        } else {
            let new_ptr = unsafe { std_alloc::alloc(new_layout) };
            if new_ptr.is_null() {
                return Err(AllocationError);
            }

            let copy_size = core::cmp::min(old_layout.size(), new_layout.size());

            unsafe {
                ptr.copy_to_nonoverlapping(new_ptr, copy_size);
                std_alloc::dealloc(ptr, old_layout);
            }

            new_ptr
        };
        Ok(unsafe { NonNull::new_unchecked(new_ptr) })
    }
    
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        unsafe { std_alloc::dealloc(ptr.as_ptr(), layout) };
    }
}
