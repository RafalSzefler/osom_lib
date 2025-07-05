use alloc::alloc as std_alloc;

use core::alloc::Layout;

use super::{AllocatedMemory, AllocationError, Allocator};

/// A thin wrapper around `*mut u8`.
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
#[repr(transparent)]
#[must_use]
pub struct StdAllocatedMemory {
    ptr: *mut u8,
}

impl StdAllocatedMemory {
    #[inline(always)]
    const fn new<T: Sized>(ptr: *mut T) -> Self {
        Self { ptr: ptr.cast() }
    }
}

unsafe impl AllocatedMemory for StdAllocatedMemory {
    #[inline(always)]
    unsafe fn as_ptr<T: Sized>(&self) -> *mut T {
        self.ptr.cast()
    }

    fn resize(self, old_layout: Layout, new_layout: Layout) -> Result<Self, AllocationError> {
        let new_ptr = if old_layout.align() == new_layout.align() {
            let new_ptr = unsafe { std_alloc::realloc(self.ptr, old_layout, new_layout.size()) };
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
                self.ptr.copy_to_nonoverlapping(new_ptr, copy_size);
                std_alloc::dealloc(self.ptr, old_layout);
            }

            new_ptr
        };

        Ok(StdAllocatedMemory::new(new_ptr))
    }

    fn deallocate(self, layout: Layout) {
        unsafe { std_alloc::dealloc(self.ptr, layout) };
    }
}

/// Represents the default allocator taken from the standard Rust library.
#[derive(Clone, Default, Debug)]
#[repr(C)]
#[must_use]
pub struct StdAllocator;

unsafe impl Allocator for StdAllocator {
    type TAllocatedMemory = StdAllocatedMemory;

    fn allocate(&self, layout: Layout) -> Result<Self::TAllocatedMemory, AllocationError> {
        let new_ptr = unsafe { std_alloc::alloc(layout) };
        if new_ptr.is_null() {
            return Err(AllocationError);
        }
        Ok(StdAllocatedMemory::new(new_ptr))
    }

    #[inline(always)]
    unsafe fn convert_raw_ptr<T: Sized>(&self, ptr: *mut T) -> Self::TAllocatedMemory {
        StdAllocatedMemory::new(ptr)
    }
    
    #[inline(always)]
    unsafe fn dangling<T: Sized>(&self) -> Self::TAllocatedMemory {
        StdAllocatedMemory::new(core::ptr::dangling_mut::<T>())
    }
}
