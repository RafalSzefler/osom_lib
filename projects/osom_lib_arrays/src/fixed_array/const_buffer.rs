#![allow(
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::needless_return,
    clippy::new_without_default
)]

use crate::const_helpers::subslice_const;

use super::ConstFixedArray;

use osom_lib_primitives::length::Length;
use osom_lib_reprc::traits::ReprC;

/// This struct allows for iterating over a fixed-length blocks of arrays,
/// given a big chunk of arbitrary length array.
///
/// # Examples
///
/// Lets say that we have an array of 20 elements:
///
/// ```rust
/// let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
/// ```
///
/// and now we want to process it in a block-by-block fashion, with each block having 7 elements.
/// We can do this:
///
/// ```rust
/// use osom_lib_arrays::fixed_array::{ConstBuffer, ConstFixedArray};
///
/// let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
///
/// let mut bufferer = ConstBuffer::<7, _>::new();
/// let mut iterator = bufferer.buffer_const(&data);
/// assert_eq!(iterator.next(), Some(&[1, 2, 3, 4, 5, 6, 7]));
/// assert_eq!(iterator.next(), Some(&[8, 9, 10, 11, 12, 13, 14]));
/// assert_eq!(iterator.next(), None);
///
/// // The iterator is drained, but we still have remaining data:
/// let remaining: ConstFixedArray<7, _> = bufferer.release_const();
/// assert_eq!(remaining.as_slice_const(), &[15, 16, 17, 18, 19, 20]);
/// ```
///
/// # Notes
///
/// The [`ConstBuffer`] is additionally const friendly, and thus is defined
/// for `T: Copy` only. In particular it isn't `Drop`.
#[repr(transparent)]
#[must_use]
pub struct ConstBuffer<const TSIZE: usize, T: Sized + Copy> {
    array: ConstFixedArray<TSIZE, T>,
}

unsafe impl<const TSIZE: usize, T: ReprC + Sized + Copy> ReprC for ConstBuffer<TSIZE, T> {
    const CHECK: () = {
        let () = <ConstFixedArray<TSIZE, T> as ReprC>::CHECK;
    };
}

impl<const TSIZE: usize, T: Sized + Copy> ConstBuffer<TSIZE, T> {
    /// Creates a new, empty [`ConstBufferer`].
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            array: ConstFixedArray::new(),
        }
    }

    /// Returns a new [`ConstBufferer`] for the given data.
    ///
    /// # Panics
    ///
    /// The function will panic if the data length exceeds `u32::MAX`.
    #[inline(always)]
    pub const fn buffer_const<'a>(&'a mut self, data: &'a [T]) -> ConstBufferer<'a, TSIZE, T> {
        assert!(data.len() <= u32::MAX as usize, "Data length cannot exceed u32::MAX");
        ConstBufferer::new(&mut self.array, data)
    }

    /// Returns the length of the data buffered in the [`ConstBuffer`].
    #[inline(always)]
    pub const fn length(&self) -> Length {
        self.array.length()
    }

    /// Clones the [`ConstBuffer`].
    #[inline(always)]
    pub const fn clone_const(&self) -> Self {
        Self {
            array: self.array.clone_const(),
        }
    }

    /// Returns a reference to the currently buffered data.
    #[inline(always)]
    pub const fn current_state_const(&self) -> &ConstFixedArray<TSIZE, T> {
        &self.array
    }

    /// Releases the remaining data from the bufferer.
    #[inline(always)]
    pub const fn release_const(self) -> ConstFixedArray<TSIZE, T> {
        let mut array = self.array;
        if array.length().as_usize() == TSIZE {
            array.drain();
        }
        array
    }
}

/// A helper struct for buffering the data in [`ConstBuffer`].
#[repr(C)]
#[must_use]
pub struct ConstBufferer<'a, const TSIZE: usize, T: Sized + Copy + 'a> {
    array: &'a mut ConstFixedArray<TSIZE, T>,
    data: &'a [T],
    data_offset: i64,
}

unsafe impl<const TSIZE: usize, T: ReprC + Sized + Copy> ReprC for ConstBufferer<'_, TSIZE, T> {
    const CHECK: () = ();
}

impl<'a, const TSIZE: usize, T: Sized + Copy> ConstBufferer<'a, TSIZE, T> {
    #[inline(always)]
    const fn new(array: &'a mut ConstFixedArray<TSIZE, T>, data: &'a [T]) -> Self {
        Self {
            array,
            data,
            data_offset: 0,
        }
    }

    /// Returns the next block of data, if available.
    pub const fn next(&mut self) -> Option<&[T; TSIZE]> {
        if self.data_offset as usize == self.data.len() {
            return None;
        }

        unsafe {
            let real_slice = subslice_const(self.data, self.data_offset as usize..self.data.len());
            let real_len = real_slice.len() as u32;

            if self.array.length().as_usize() == TSIZE {
                self.array.drain();
            }

            let len = self.array.length().as_usize();

            if (real_len as u64) + len as u64 >= TSIZE as u64 {
                let missing = TSIZE - len;
                let subslice = subslice_const(real_slice, 0..missing);
                self.array.push_slice_const(subslice);
                self.data_offset += missing as i64;
                debug_assert!(self.array.length().as_usize() == TSIZE);
                return Some(self.array.as_raw_slice_const());
            }

            self.array.push_slice_const(real_slice);
            self.data_offset += real_len as i64;
            debug_assert!(self.array.length().as_usize() < TSIZE);
            return None;
        }
    }
}
