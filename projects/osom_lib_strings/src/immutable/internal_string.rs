#![allow(clippy::cast_possible_truncation, clippy::cast_ptr_alignment)]
use core::{
    ptr::NonNull,
    sync::atomic::{AtomicU32, Ordering},
};

use osom_lib_alloc::traits::Allocator;
use osom_lib_macros::debug_check_or_release_hint;
use osom_lib_primitives::length::Length;
use osom_lib_reprc::macros::reprc;

use crate::immutable::{ImmutableStringError, internal_string_layout::HEAP_OBJECT_LAYOUT};

#[reprc]
#[derive(Debug, Clone)]
pub struct InternalString<TAllocator: Allocator> {
    data: *mut u8,
    length: Length,
    capacity: Length,
    allocator: TAllocator,
}

unsafe impl<TAllocator: Allocator> Send for InternalString<TAllocator> {}
unsafe impl<TAllocator: Allocator> Sync for InternalString<TAllocator> {}

impl<TAllocator: Allocator> InternalString<TAllocator> {
    #[inline(always)]
    pub fn with_allocator_and_capacity(capacity: Length, allocator: TAllocator) -> Result<Self, ImmutableStringError> {
        let capacity = capacity.as_usize();
        let new_layout = HEAP_OBJECT_LAYOUT.calculate_for_data_size(capacity);
        let new_ptr = allocator.allocate(new_layout).map_err(Into::into)?;
        let result = Self {
            data: new_ptr.as_ptr(),
            length: Length::ZERO,
            capacity: unsafe { Length::new_unchecked(capacity as u32) },
            allocator,
        };

        result.strong().store(1, Ordering::Relaxed);
        result.weak().store(1, Ordering::Relaxed);
        Ok(result)
    }

    #[inline(always)]
    pub const fn strong(&self) -> &AtomicU32 {
        debug_check_or_release_hint!(!self.data.is_null());
        unsafe { &*self.data.add(HEAP_OBJECT_LAYOUT.strong_offset).cast::<AtomicU32>() }
    }

    #[inline(always)]
    pub const fn weak(&self) -> &AtomicU32 {
        debug_check_or_release_hint!(!self.data.is_null());
        unsafe { &*self.data.add(HEAP_OBJECT_LAYOUT.weak_offset).cast::<AtomicU32>() }
    }

    #[inline(always)]
    pub const fn data_start(&self) -> *mut u8 {
        unsafe { self.data.add(HEAP_OBJECT_LAYOUT.data_offset) }
    }

    #[inline(always)]
    pub const fn data_end(&self) -> *mut u8 {
        unsafe { self.data_start().add(self.length.as_usize()) }
    }

    #[inline(always)]
    pub const fn length(&self) -> Length {
        self.length
    }

    #[inline(always)]
    pub fn deallocate(self) {
        unsafe {
            let old_layout = HEAP_OBJECT_LAYOUT.calculate_for_data_size(self.capacity.as_usize());
            let ptr = NonNull::new_unchecked(self.data);
            self.allocator.deallocate(ptr, old_layout);
        }
    }

    pub fn try_push_slice(&mut self, array: &[u8]) -> Result<(), ImmutableStringError> {
        let len = array.len();
        if array.len() >= Length::MAX_LENGTH.as_usize() {
            return Err(ImmutableStringError::MaxLengthExceeded);
        }

        let new_size = unsafe { (u64::from(self.length.as_u32())).unchecked_add(len as u64) };
        if new_size > u64::from(Length::MAX_LENGTH.as_u32()) {
            return Err(ImmutableStringError::MaxLengthExceeded);
        }

        let new_size = new_size as u32;

        if new_size > self.capacity.as_u32() {
            self.grow_to_fit(new_size)?;
        }

        let end = self.data_end();
        unsafe {
            end.copy_from_nonoverlapping(array.as_ptr(), len);
            self.length = Length::new_unchecked(new_size);
        }
        Ok(())
    }

    fn grow_to_fit(&mut self, new_size: u32) -> Result<(), ImmutableStringError> {
        let real_new_size = self.calculate_real_new_size(new_size);
        let new_layout = HEAP_OBJECT_LAYOUT.calculate_for_data_size(real_new_size as usize);
        let old_layout = HEAP_OBJECT_LAYOUT.calculate_for_data_size(self.capacity.as_usize());
        let new_ptr = unsafe {
            let old_ptr = NonNull::new_unchecked(self.data);
            let new_ptr = self
                .allocator
                .resize(old_ptr, old_layout, new_layout)
                .map_err(Into::into)?;
            new_ptr.as_ptr()
        };
        self.data = new_ptr;
        self.capacity = unsafe { Length::new_unchecked(real_new_size) };
        Ok(())
    }

    pub fn shrink_to_fit(&mut self) -> Result<(), ImmutableStringError> {
        if self.length == self.capacity {
            return Ok(());
        }

        let old_layout = HEAP_OBJECT_LAYOUT.calculate_for_data_size(self.capacity.as_usize());
        let new_layout = HEAP_OBJECT_LAYOUT.calculate_for_data_size(self.length.as_usize());

        let new_ptr = unsafe {
            let old_ptr = NonNull::new_unchecked(self.data);
            let new_ptr = self
                .allocator
                .resize(old_ptr, old_layout, new_layout)
                .map_err(Into::into)?;
            new_ptr.as_ptr()
        };
        self.data = new_ptr;
        self.capacity = self.length;
        Ok(())
    }

    #[inline(always)]
    fn calculate_real_new_size(&self, new_size: u32) -> u32 {
        let new_size = u64::from(new_size);
        let real_new_size = 3 * (u64::from(self.length.as_u32()) + 1) / 2 + 1;
        if new_size < real_new_size {
            return real_new_size as u32;
        }

        let real_new_size = 2 * (u64::from(self.length.as_u32()) + 1) + 1;
        if new_size < real_new_size {
            return real_new_size as u32;
        }

        new_size as u32 + 1
    }
}
