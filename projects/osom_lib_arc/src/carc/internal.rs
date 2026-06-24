#![allow(clippy::cast_ptr_alignment)]

use core::{
    marker::PhantomData,
    ptr::NonNull,
    sync::atomic::{AtomicU32, Ordering},
};

use osom_lib_alloc::traits::Allocator;
use osom_lib_reprc::traits::ReprC;

use crate::errors::CArcError;

use super::layout::CArcLayout;

#[repr(transparent)]
#[must_use]
#[derive(Debug)]
pub struct InternalArc<T, TAllocator: Allocator> {
    ptr: *mut u8,
    _phantom: PhantomData<(T, TAllocator)>,
}

unsafe impl<T: ReprC, TAllocator: Allocator> ReprC for InternalArc<T, TAllocator> {
    const CHECK: () = const {
        osom_lib_reprc::hidden::is_reprc::<T>();
        osom_lib_reprc::hidden::is_reprc::<TAllocator>();
    };
}

unsafe impl<T: Send, TAllocator: Allocator + Send> Send for InternalArc<T, TAllocator> {}
unsafe impl<T: Sync, TAllocator: Allocator + Sync> Sync for InternalArc<T, TAllocator> {}

impl<T, TAllocator: Allocator> InternalArc<T, TAllocator> {
    const LAYOUT: CArcLayout<T, TAllocator> = CArcLayout::new();

    #[inline]
    pub fn new(value: T) -> Result<Self, CArcError>
    where
        TAllocator: Default,
    {
        Self::with_allocator(value, TAllocator::default())
    }

    pub fn with_allocator(value: T, mut allocator: TAllocator) -> Result<Self, CArcError> {
        let ptr = allocator
            .allocate(Self::LAYOUT.total_layout)
            .map_err(|_| CArcError::AllocationError)?
            .as_ptr();

        let result = Self {
            ptr,
            _phantom: PhantomData,
        };

        unsafe {
            result.allocator_ptr().write(allocator);
            result.strong().store(1, Ordering::Relaxed);
            result.weak().store(1, Ordering::Relaxed);
            result.data_ptr().write(value);
        }

        Ok(result)
    }

    /// Compares the raw pointers of the two internal arcs.
    #[inline]
    pub fn raw_equals(&self, other: &Self) -> bool {
        core::ptr::eq(self.ptr, other.ptr)
    }

    #[inline]
    pub const fn strong(&self) -> &AtomicU32 {
        unsafe { &*self.ptr.add(Self::LAYOUT.strong_offset).cast::<AtomicU32>() }
    }

    #[inline]
    pub const fn weak(&self) -> &AtomicU32 {
        unsafe { &*self.ptr.add(Self::LAYOUT.weak_offset).cast::<AtomicU32>() }
    }

    #[inline]
    pub unsafe fn deallocate_memory(&self) {
        unsafe {
            let mut allocator = self.allocator_ptr().read();
            allocator.deallocate(NonNull::new_unchecked(self.ptr), Self::LAYOUT.total_layout);
        }
    }

    #[inline]
    pub const unsafe fn read_data(&self) -> T {
        unsafe { self.data_ptr().read() }
    }

    #[inline]
    pub const fn data(&self) -> &T {
        unsafe { &*self.data_ptr() }
    }

    #[inline]
    pub const fn clone(&self) -> Self {
        Self {
            ptr: self.ptr,
            _phantom: PhantomData,
        }
    }

    #[inline(always)]
    pub const fn raw_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn from_raw_ptr(ptr: *mut u8) -> Self {
        Self {
            ptr,
            _phantom: PhantomData,
        }
    }

    #[inline]
    const fn data_ptr(&self) -> *mut T {
        unsafe { self.ptr.add(Self::LAYOUT.data_offset).cast::<T>() }
    }

    #[inline]
    const fn allocator_ptr(&self) -> *mut TAllocator {
        unsafe { self.ptr.add(Self::LAYOUT.allocator_offset).cast::<TAllocator>() }
    }
}
