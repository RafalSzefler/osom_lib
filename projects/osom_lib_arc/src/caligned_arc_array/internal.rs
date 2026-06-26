#![allow(clippy::cast_ptr_alignment, clippy::cast_possible_truncation)]

use core::{
    marker::PhantomData,
    ptr::NonNull,
    sync::atomic::{AtomicU32, Ordering},
};

use osom_lib_alloc::traits::Allocator;
use osom_lib_primitives::length::Length;
use osom_lib_reprc::traits::ReprC;
use osom_lib_try_clone::TryClone;

use crate::errors::CArcArrayError;

use super::layout::CArcLayout;

#[repr(transparent)]
#[must_use]
#[derive(Debug)]
pub struct InternalAlignedArcArray<TAlign, TItem, TAllocator: Allocator> {
    ptr: *mut u8,
    _phantom: PhantomData<(TAlign, TItem, TAllocator)>,
}

unsafe impl<TAlign, TItem: ReprC, TAllocator: Allocator> ReprC for InternalAlignedArcArray<TAlign, TItem, TAllocator> {
    const CHECK: () = const {
        osom_lib_reprc::hidden::is_reprc::<TItem>();
        osom_lib_reprc::hidden::is_reprc::<TAllocator>();
    };
}

unsafe impl<TAlign, TItem: Send, TAllocator: Allocator + Send> Send
    for InternalAlignedArcArray<TAlign, TItem, TAllocator>
{
}
unsafe impl<TAlign, TItem: Sync, TAllocator: Allocator + Sync> Sync
    for InternalAlignedArcArray<TAlign, TItem, TAllocator>
{
}

impl<TAlign, TItem, TAllocator: Allocator> InternalAlignedArcArray<TAlign, TItem, TAllocator> {
    const LAYOUT: CArcLayout<TAlign, TItem, TAllocator> = CArcLayout::new();

    pub fn new(capacity: Length, mut allocator: TAllocator) -> Result<Self, CArcArrayError> {
        let layout = Self::LAYOUT.calculate_for_capacity(capacity.as_u32())?;
        let ptr = allocator
            .allocate(layout)
            .map_err(|_| CArcArrayError::AllocationError)?
            .as_ptr();

        let result = Self {
            ptr,
            _phantom: PhantomData,
        };

        unsafe {
            result.allocator_ptr().write(allocator);
            result.strong().store(1, Ordering::Relaxed);
            result.weak().store(1, Ordering::Relaxed);
            result.size_ptr().write(0);
            result.capacity_ptr().write(capacity.as_u32());
        }

        Ok(result)
    }

    /// Compares the raw pointers of the two internal arcs.
    #[inline]
    pub fn raw_equals(&self, other: &Self) -> bool {
        core::ptr::eq(self.ptr, other.ptr)
    }

    pub fn shrink_to_fit(&mut self) -> Result<(), CArcArrayError> {
        let current_size = self.size().as_u32();
        let current_capacity = self.capacity().as_u32();
        if current_size == current_capacity {
            return Ok(());
        }

        let old_layout = Self::LAYOUT.calculate_for_capacity(current_capacity)?;
        let new_layout = Self::LAYOUT.calculate_for_capacity(current_size)?;
        unsafe {
            let old_ptr = NonNull::new_unchecked(self.ptr);
            let new_ptr = self
                .allocator_ptr()
                .as_mut_unchecked()
                .resize(old_ptr, old_layout, new_layout)
                .map_err(|_| CArcArrayError::AllocationError)?;
            self.ptr = new_ptr.as_ptr();
            self.capacity_ptr().write(current_size);
            Ok(())
        }
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
    pub const fn size(&self) -> Length {
        unsafe { Length::new_unchecked(self.size_ptr().read()) }
    }

    #[inline]
    pub const fn capacity(&self) -> Length {
        unsafe { Length::new_unchecked(self.capacity_ptr().read()) }
    }

    pub fn try_push_slice(&mut self, slice: &[TItem]) -> Result<(), CArcArrayError>
    where
        TItem: TryClone,
    {
        let slice_len = slice.len();
        if slice_len > Length::MAX_LENGTH.as_usize() {
            return Err(CArcArrayError::ArraySizeOutOfRange);
        }
        let slice_len = slice_len as u64;
        let current_size = self.size().as_u32();
        if u64::from(current_size) + slice_len > u64::from(Length::MAX_LENGTH.as_u32()) {
            return Err(CArcArrayError::ArraySizeOutOfRange);
        }

        let slice_len = slice_len as u32;

        let new_size = unsafe { current_size.unchecked_add(slice_len) };

        if new_size > self.capacity().as_u32() {
            self.grow(new_size)?;
        }

        unsafe {
            let mut data_ptr = self.data_ptr().add(current_size as usize);
            let mut slice_ptr = slice.as_ptr();
            for _ in 0..slice_len {
                data_ptr.write(
                    slice_ptr
                        .as_ref_unchecked()
                        .try_clone()
                        .map_err(|_| CArcArrayError::ItemCloningError)?,
                );
                data_ptr = data_ptr.add(1);
                slice_ptr = slice_ptr.add(1);
            }
            self.size_ptr().write(new_size);
        }

        Ok(())
    }

    pub fn try_push_array<const N: usize>(&mut self, array: [TItem; N]) -> Result<(), CArcArrayError> {
        if N > Length::MAX_LENGTH.as_usize() {
            return Err(CArcArrayError::ArraySizeOutOfRange);
        }
        let slice_len = N as u64;
        let current_size = self.size().as_u32();
        if u64::from(current_size) + slice_len > u64::from(Length::MAX_LENGTH.as_u32()) {
            return Err(CArcArrayError::ArraySizeOutOfRange);
        }

        let slice_len = slice_len as u32;

        let new_size = unsafe { current_size.unchecked_add(slice_len) };

        if new_size > self.capacity().as_u32() {
            self.grow(new_size)?;
        }

        unsafe {
            let data_ptr = self.data_ptr().add(current_size as usize);
            data_ptr.copy_from_nonoverlapping(array.as_ptr(), N);
            core::mem::forget(array);
            self.size_ptr().write(new_size);
        }

        Ok(())
    }

    #[inline]
    pub unsafe fn deallocate_memory(&self) {
        unsafe {
            let mut allocator = self.allocator_ptr().read();
            let layout = Self::LAYOUT
                .calculate_for_capacity(self.size().as_u32())
                .expect("Layout should be calculatable");
            allocator.deallocate(NonNull::new_unchecked(self.ptr), layout);
        }
    }

    #[inline]
    pub const fn raw_clone(&self) -> Self {
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
    pub const fn data_slice(&self) -> &[TItem] {
        unsafe { core::slice::from_raw_parts(self.data_ptr(), self.size().as_usize()) }
    }

    #[inline]
    pub const fn data_slice_mut(&mut self) -> &mut [TItem] {
        unsafe { core::slice::from_raw_parts_mut(self.data_ptr(), self.size().as_usize()) }
    }

    fn grow(&mut self, new_size: u32) -> Result<(), CArcArrayError> {
        let current_capacity = self.capacity().as_u32();
        let mut new_capacity = current_capacity;
        for _ in 0..4 {
            new_capacity = 1 + 2 * new_capacity / 3;
            if new_capacity >= new_size {
                break;
            }
        }

        if new_capacity < new_size {
            new_capacity = new_size + 4;
            if new_capacity > Length::MAX_LENGTH.as_u32() {
                return Err(CArcArrayError::ArraySizeOutOfRange);
            }
        }

        let old_layout = Self::LAYOUT.calculate_for_capacity(current_capacity)?;
        let new_layout = Self::LAYOUT.calculate_for_capacity(new_capacity)?;
        unsafe {
            let allocator = self.allocator_ptr().as_mut_unchecked();
            let new_ptr = allocator
                .resize(NonNull::new_unchecked(self.ptr), old_layout, new_layout)
                .map_err(|_| CArcArrayError::AllocationError)?;
            let raw_ptr = new_ptr.as_ptr();
            self.ptr = raw_ptr;
            self.capacity_ptr().write(new_capacity);
        };

        Ok(())
    }

    #[inline]
    const fn data_ptr(&self) -> *mut TItem {
        unsafe { self.ptr.add(Self::LAYOUT.data_offset).cast::<TItem>() }
    }

    #[inline]
    const fn allocator_ptr(&self) -> *mut TAllocator {
        unsafe { self.ptr.add(Self::LAYOUT.allocator_offset).cast::<TAllocator>() }
    }

    #[inline]
    const fn size_ptr(&self) -> *mut u32 {
        unsafe { self.ptr.add(Self::LAYOUT.size_offset).cast::<u32>() }
    }

    #[inline]
    const fn capacity_ptr(&self) -> *mut u32 {
        unsafe { self.ptr.add(Self::LAYOUT.capacity_offset).cast::<u32>() }
    }
}
