#![allow(clippy::cast_possible_truncation)]

use osom_lib_alloc::traits::Allocator;
use osom_lib_primitives::length::Length;

use crate::{
    errors::{ArrayError, ArrayIsEmptyError},
    traits::MutableArray,
};

use super::InlineArray;

impl<const TCAPACITY: usize, T, TAllocator> MutableArray<T> for InlineArray<TCAPACITY, T, TAllocator>
where
    T: Sized,
    TAllocator: Allocator,
{
    fn try_push_array<const TSIZE: usize>(&mut self, arr: [T; TSIZE]) -> Result<(), ArrayError> {
        let length = self.size.as_usize();
        let Some(new_length) = length.checked_add(TSIZE) else {
            return Err(ArrayError::LengthLimitExceeded);
        };

        if new_length > Length::MAX_LENGTH.as_usize() {
            return Err(ArrayError::LengthLimitExceeded);
        }

        let new_length = new_length as u32;

        self.reserve_if_needed(new_length)?;

        unsafe {
            let current_end = self.current_ptr_mut().add(length);
            let mut arr = arr;
            let arr_ptr = arr.as_mut_ptr();
            current_end.copy_from_nonoverlapping(arr_ptr, TSIZE);
            core::mem::forget(arr);
            self.size = Length::new_unchecked(new_length);
        }

        Ok(())
    }

    fn try_push_slice(&mut self, slice: &[T]) -> Result<(), ArrayError>
    where
        T: Clone,
    {
        let length = self.size.as_usize();
        let Some(new_length) = length.checked_add(slice.len()) else {
            return Err(ArrayError::LengthLimitExceeded);
        };

        if new_length > Length::MAX_LENGTH.as_usize() {
            return Err(ArrayError::LengthLimitExceeded);
        }

        let new_length = new_length as u32;

        self.reserve_if_needed(new_length)?;

        unsafe {
            let mut current_end = self.current_ptr_mut().add(length);
            let mut slice_val = slice.as_ptr();
            for _ in 0..slice.len() {
                current_end.write((&*slice_val).clone());
                current_end = current_end.add(1);
                slice_val = slice_val.add(1);
            }
            self.size = Length::new_unchecked(new_length);
        }

        Ok(())
    }

    fn try_pop(&mut self) -> Result<T, ArrayIsEmptyError> {
        let len = self.size.as_usize();
        if len == 0 {
            return Err(ArrayIsEmptyError);
        }

        let idx = len - 1;
        unsafe {
            self.size = Length::new_unchecked(idx as u32);
            let ptr = self.current_ptr_mut().add(idx);
            Ok(ptr.read())
        }
    }

    #[inline(always)]
    fn as_slice_mut(&mut self) -> &mut [T] {
        self.as_slice_mut_internal()
    }
}
