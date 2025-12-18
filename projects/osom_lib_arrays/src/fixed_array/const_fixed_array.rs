#![allow(clippy::cast_possible_truncation, clippy::new_without_default)]

use core::{marker::PhantomData, mem::MaybeUninit};

use osom_lib_primitives::length::Length;
use osom_lib_reprc::traits::ReprC;

use crate::{
    const_helpers::{subslice_const, subslice_mut_const},
    errors::{ArrayError, ArrayIsEmptyError},
};

/// A fixed-capacity array. This type is a const (as in: compile time) analogue of [`FixedArray`][`super::FixedArray`].
///
/// # Notes
///
/// For the array to be usable in const context, the `T` type needs to be `Copy` (which implies "no Drop").
/// And in that case `ConstFixedArray` doesn't need to be (and actually cannot be to work in const context)
/// `Drop` as well.
///
/// Unlike [`FixedArray`][`super::FixedArray`], this type does not implement
/// any of the convenient traits such as [`ImmutableArray`][`crate::traits::ImmutableArray`]
/// or `PartialEq`. These are NOT usable in const context anyway.
#[repr(C)]
#[must_use]
pub struct ConstFixedArray<const TSIZE: usize, T: Sized + Copy> {
    length: Length,
    inner: [T; TSIZE],
    _phantom: PhantomData<T>,
}

unsafe impl<const TSIZE: usize, T: ReprC + Sized + Copy> ReprC for ConstFixedArray<TSIZE, T> {
    const CHECK: () = {
        let () = T::CHECK;
        let () = <PhantomData<T> as ReprC>::CHECK;
        let () = <Length as ReprC>::CHECK;
        let () = <[T; TSIZE] as ReprC>::CHECK;
    };
}

impl<const TSIZE: usize, T: Sized + Copy> ConstFixedArray<TSIZE, T> {
    /// Creates a new, empty [`ConstFixedArray`].
    ///
    /// # Panics
    ///
    /// This function will panic if the `TSIZE` is invalid, i.e.
    /// when either `TSIZE` is 0 or exceeds [`Length::MAX_LENGTH`].
    pub const fn new() -> Self {
        const {
            assert!(
                TSIZE <= Length::MAX_LENGTH.as_usize(),
                "TSIZE cannot exceed Length::MAX_LENGTH"
            );
            assert!(TSIZE > 0, "TSIZE cannot be 0");
        }

        let empty = unsafe { MaybeUninit::<[T; TSIZE]>::zeroed().assume_init() };

        Self {
            length: Length::ZERO,
            inner: empty,
            _phantom: PhantomData,
        }
    }

    /// Returns the length of the [`ConstFixedArray`].
    #[inline(always)]
    pub const fn length(&self) -> Length {
        unsafe { core::hint::assert_unchecked(self.length.as_usize() <= TSIZE) };
        self.length
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.length.as_u32() == 0
    }

    /// Returns the capacity of the [`ConstFixedArray`]. This is `TSIZE`
    /// as [`Length`].
    #[inline(always)]
    pub const fn capacity(&self) -> Length {
        unsafe { Length::new_unchecked(TSIZE as u32) }
    }

    /// Returns the [`ConstFixedArray`] as immutable slice.
    #[inline(always)]
    pub const fn as_slice_const(&self) -> &[T] {
        unsafe {
            let result = subslice_const(&self.inner, 0..self.length.as_usize());
            core::hint::assert_unchecked(result.len() <= TSIZE);
            result
        }
    }

    /// Returns the [`ConstFixedArray`] as mutable slice.
    #[inline(always)]
    pub const fn as_slice_mut_const(&mut self) -> &mut [T] {
        unsafe {
            let result = subslice_mut_const(&mut self.inner, 0..self.length.as_usize());
            core::hint::assert_unchecked(result.len() <= TSIZE);
            result
        }
    }

    /// Pushes a raw array to the [`ConstFixedArray`].
    ///
    /// # Errors
    ///
    /// For errors see [`ArrayError`].
    #[inline(always)]
    pub const fn try_push_array_const<const TARRSIZE: usize>(&mut self, arr: [T; TARRSIZE]) -> Result<(), ArrayError> {
        self.try_push_slice_const(&arr)
    }

    /// Pushes a raw array to the [`ConstFixedArray`].
    ///
    /// # Panics
    ///
    /// Panics if the array is full. Should be consistent with [`ConstFixedArray::try_push_array_const`].
    pub const fn push_array_const<const TARRSIZE: usize>(&mut self, arr: [T; TARRSIZE]) {
        match self.try_push_array_const(arr) {
            Ok(()) => (),
            Err(err) => match err {
                ArrayError::LengthLimitExceeded => {
                    panic!("Failed to push array due to length limit exceeded");
                }
                ArrayError::AllocationError => {
                    panic!("Failed to push array due to allocation error");
                }
            },
        }
    }

    /// Pushes a slice to the [`ConstFixedArray`].
    ///
    /// # Errors
    ///
    /// For errors see [`ArrayError`].
    pub const fn try_push_slice_const(&mut self, slice: &[T]) -> Result<(), ArrayError> {
        let len = self.length.as_usize();
        let tsize = slice.len();
        if len + tsize > TSIZE {
            return Err(ArrayError::LengthLimitExceeded);
        }

        unsafe {
            let dst = subslice_mut_const(&mut self.inner, len..len + tsize);
            dst.copy_from_slice(slice);
            self.length = Length::new_unchecked((len + tsize) as u32);
        }
        Ok(())
    }

    /// Pushes a slice to the [`ConstFixedArray`].
    ///
    /// # Panics
    ///
    /// Panics if the array is full. Should be consistent with [`ConstFixedArray::try_push_slice_const`].
    #[inline(always)]
    pub const fn push_slice_const(&mut self, slice: &[T]) {
        match self.try_push_slice_const(slice) {
            Ok(()) => (),
            Err(err) => match err {
                ArrayError::LengthLimitExceeded => {
                    panic!("Failed to push slice due to length limit exceeded");
                }
                ArrayError::AllocationError => {
                    panic!("Failed to push slice due to allocation error");
                }
            },
        }
    }

    /// Pushes a single element to the [`ConstFixedArray`].
    ///
    /// # Errors
    ///
    /// For errors see [`ArrayError`].
    #[inline(always)]
    pub const fn try_push_const(&mut self, value: T) -> Result<(), ArrayError> {
        self.try_push_array_const([value])
    }

    /// Pushes a single element to the [`ConstFixedArray`].
    ///
    /// # Panics
    ///
    /// Panics if the array is full. Should be consistent with [`ConstFixedArray::try_push_const`].
    #[inline(always)]
    pub const fn push_const(&mut self, value: T) {
        self.push_array_const([value]);
    }

    /// Removes an element from the top of the [`ConstFixedArray`].
    ///
    /// # Errors
    ///
    /// Returns [`ArrayIsEmptyError`] when the array is empty.
    pub const fn try_pop_const(&mut self) -> Result<T, ArrayIsEmptyError> {
        let len = self.length.as_u32();
        if len == 0 {
            return Err(ArrayIsEmptyError);
        }

        let item = unsafe {
            self.length = Length::new_unchecked(len - 1);
            (&raw const self.inner).cast::<T>().add((len - 1) as usize).read()
        };
        Ok(item)
    }

    /// Removes an element from the top of the [`ConstFixedArray`].
    ///
    /// # Panics
    ///
    /// Panics if the array is empty. Should be consistent with [`ConstFixedArray::try_pop_const`].
    #[inline(always)]
    #[must_use]
    pub const fn pop_const(&mut self) -> T {
        match self.try_pop_const() {
            Ok(item) => item,
            Err(ArrayIsEmptyError) => panic!("Failed to pop due to array being empty"),
        }
    }

    /// Clones the [`ConstFixedArray`].
    #[inline(always)]
    pub const fn clone_const(&self) -> Self {
        Self {
            length: self.length,
            inner: self.inner,
            _phantom: PhantomData,
        }
    }

    /// Returns the raw immutable reference to the underlying array.
    ///
    /// # Safety
    ///
    /// This function is unsafe because only the buffer up to `Length`
    /// is guaranteed to be filled with a valid data.
    #[inline(always)]
    pub const unsafe fn as_raw_slice_const(&self) -> &[T; TSIZE] {
        &self.inner
    }

    /// Returns the raw mutable reference to the underlying array.
    ///
    /// # Safety
    ///
    /// This function is unsafe because only the buffer up to `Length`
    /// is guaranteed to be filled with a valid data.
    #[inline(always)]
    pub const unsafe fn as_raw_slice_mut_const(&mut self) -> &mut [T; TSIZE] {
        &mut self.inner
    }

    /// Drains the [`ConstFixedArray`]. This call sets the internal length of [`ConstFixedArray`] to 0,
    /// making it effectively empty.
    #[inline(always)]
    pub const fn drain(&mut self) {
        self.length = Length::ZERO;
    }
}
