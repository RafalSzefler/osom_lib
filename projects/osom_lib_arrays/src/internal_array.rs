#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
use core::{alloc::Layout, marker::PhantomData, ptr::NonNull};

use osom_lib_alloc::traits::Allocator;
use osom_lib_primitives::length::Length;
use osom_lib_reprc::traits::ReprC;

use crate::errors::{ArrayError, ArrayIsEmptyError};

#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct InternalArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    raw_ptr: NonNull<T>,
    length: Length,
    capacity: Length,
    allocator: TAllocator,
    _phantom: PhantomData<T>,
}

unsafe impl<T, TAllocator> ReprC for InternalArray<T, TAllocator>
where
    T: ReprC,
    TAllocator: Allocator,
{
    const CHECK: () = {
        let () = T::CHECK;
        let () = <Length as ReprC>::CHECK;
        let () = <TAllocator as ReprC>::CHECK;
        let () = <PhantomData<T> as ReprC>::CHECK;
        let () = <NonNull<T> as ReprC>::CHECK;
    };
}

unsafe impl<T, TAllocator> Send for InternalArray<T, TAllocator>
where
    T: Send,
    TAllocator: Allocator,
{
}

unsafe impl<T, TAllocator> Sync for InternalArray<T, TAllocator>
where
    T: Sync,
    TAllocator: Allocator,
{
}

impl<T, TAllocator> InternalArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    const fn layout_for_size(size: Length) -> Layout {
        let tsize = size_of::<T>();
        let Some(real_size) = tsize.checked_mul(size.as_usize()) else {
            panic!("Tried to allocate array of size outside of usize range");
        };
        unsafe { Layout::from_size_align_unchecked(real_size, align_of::<T>()) }
    }

    #[inline(always)]
    const fn current_layout(&self) -> Layout {
        Self::layout_for_size(self.capacity)
    }

    #[inline(always)]
    pub const fn new(allocator: TAllocator) -> Self {
        Self {
            raw_ptr: NonNull::dangling(),
            length: Length::ZERO,
            capacity: Length::ZERO,
            allocator: allocator,
            _phantom: PhantomData,
        }
    }

    /// Initialies new [`InternalArray`], just like [`InternalArray::with_capacity`],
    /// except it sets [`InternalArray::length`] to size as well. This is deeply unsafe,
    /// since the underlying memory **is not** initialized to anything. In particular
    /// it is caller's responsibility to initialize the array properly before using it.
    #[inline(always)]
    pub unsafe fn with_size_uninitialized(size: Length, allocator: TAllocator) -> Result<Self, ArrayError> {
        let mut result = Self::with_capacity(size, allocator)?;
        result.length = size;
        Ok(result)
    }

    pub fn with_capacity(capacity: Length, allocator: TAllocator) -> Result<Self, ArrayError> {
        if capacity == Length::ZERO {
            return Ok(Self::new(allocator));
        }

        let new_ptr = allocator
            .allocate(Self::layout_for_size(capacity))
            .map_err(Into::into)?
            .cast::<T>();

        Ok(Self {
            raw_ptr: new_ptr,
            length: Length::ZERO,
            capacity: capacity,
            allocator: allocator,
            _phantom: PhantomData,
        })
    }

    pub fn try_push_array<const TSIZE: usize>(&mut self, arr: [T; TSIZE]) -> Result<(), ArrayError> {
        let length = self.length.as_usize();
        let Some(new_length) = length.checked_add(TSIZE) else {
            return Err(ArrayError::LengthLimitExceeded);
        };

        if new_length > Length::MAX_LENGTH.as_usize() {
            return Err(ArrayError::LengthLimitExceeded);
        }

        let new_length = new_length as u32;

        self.reserve_if_needed(new_length)?;

        unsafe {
            let current_end = self.raw_ptr.add(length);
            let mut arr = arr;
            let arr_ptr = NonNull::new_unchecked(arr.as_mut_ptr());
            current_end.copy_from_nonoverlapping(arr_ptr, TSIZE);
            core::mem::forget(arr);
            self.length = Length::new_unchecked(new_length);
        }

        Ok(())
    }

    pub fn try_push_slice(&mut self, slice: &[T]) -> Result<(), ArrayError>
    where
        T: Clone,
    {
        let length = self.length.as_usize();
        let Some(new_length) = length.checked_add(slice.len()) else {
            return Err(ArrayError::LengthLimitExceeded);
        };

        if new_length > Length::MAX_LENGTH.as_usize() {
            return Err(ArrayError::LengthLimitExceeded);
        }

        let new_length = new_length as u32;

        self.reserve_if_needed(new_length)?;

        unsafe {
            let mut current_end = self.raw_ptr.add(length);
            let mut slice_val = slice.as_ptr();
            for _ in 0..slice.len() {
                current_end.write((&*slice_val).clone());
                current_end = current_end.add(1);
                slice_val = slice_val.add(1);
            }
            self.length = Length::new_unchecked(new_length);
        }

        Ok(())
    }

    #[inline(always)]
    pub const fn length(&self) -> Length {
        self.length
    }

    #[inline(always)]
    pub const fn capacity(&self) -> Length {
        self.capacity
    }

    #[inline(always)]
    pub const fn as_slice(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(self.raw_ptr.as_ptr(), self.length.as_usize()) }
    }

    #[inline(always)]
    pub const fn as_slice_mut(&mut self) -> &mut [T] {
        unsafe { core::slice::from_raw_parts_mut(self.raw_ptr.as_ptr(), self.length.as_usize()) }
    }

    fn reserve_if_needed(&mut self, new_length: u32) -> Result<(), ArrayError> {
        let capacity = self.capacity.as_u32();
        if new_length <= capacity {
            return Ok(());
        }

        let new_capacity = {
            let upper_bound = ((u64::from(new_length) * 3) / 2) + 1;
            let capped = core::cmp::min(upper_bound, Length::MAX_LENGTH.as_usize() as u64) as u32;
            unsafe { Length::new_unchecked(capped) }
        };

        let new_ptr = unsafe {
            let new_layout = Self::layout_for_size(new_capacity);
            if capacity == 0 {
                self.allocator.allocate(new_layout).map_err(Into::into)?
            } else {
                let old_layout = self.current_layout();
                self.allocator
                    .resize(self.raw_ptr.cast(), old_layout, new_layout)
                    .map_err(Into::into)?
            }
        };

        self.raw_ptr = new_ptr.cast();
        self.capacity = new_capacity;
        Ok(())
    }

    pub unsafe fn deallocate(&mut self) {
        if self.capacity == Length::ZERO {
            return;
        }

        unsafe {
            if core::mem::needs_drop::<T>() {
                let mut start = self.raw_ptr;
                let end = start.add(self.length.as_usize());
                while start < end {
                    core::ptr::drop_in_place(start.as_ptr());
                    start = start.add(1);
                }
            }

            self.allocator.deallocate(self.raw_ptr.cast(), self.current_layout());
        }
    }

    pub const fn try_pop(&mut self) -> Result<T, ArrayIsEmptyError> {
        let len = self.length.as_usize();
        if len == 0 {
            return Err(ArrayIsEmptyError);
        }

        let idx = len - 1;
        unsafe {
            self.length = Length::new_unchecked(idx as u32);
            let ptr = self.raw_ptr.add(idx);
            Ok(ptr.read())
        }
    }
}

impl<T: Clone, TAllocator> InternalArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    pub fn clone(&self) -> Self {
        let mut new_array = Self::with_capacity(self.length, self.allocator.clone())
            .expect("Couldn't allocate memory during clone() call");
        new_array.try_push_slice(self.as_slice()).unwrap();
        new_array
    }
}
