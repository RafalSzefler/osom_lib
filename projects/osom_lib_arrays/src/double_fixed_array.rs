//! A module containing the implementation of the fixed array data structure.
#![allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::cast_sign_loss)]

use core::mem::MaybeUninit;

use osom_lib_primitives::Length;

use crate::{OutOfRangeError, uninit_array};

/// Represents a semi-dynamic array, where the maximum size `N+M` is known at compile time.
/// This is functionally equivalent to [`FixedArray<T, N+M>`][super::FixedArray], except
/// generic `N+M` parameter is not allowed by Rust yet.
#[derive(Debug)]
#[repr(C)]
#[must_use]
pub struct DoubleFixedArray<T, const N: usize, const M: usize> {
    first_array: [MaybeUninit<T>; N],
    second_array: [MaybeUninit<T>; M],
    length: Length,
}

impl<T, const N: usize, const M: usize> DoubleFixedArray<T, N, M> {
    /// Creates a new empty [`DoubleFixedArray`].
    #[inline(always)]
    pub const fn new() -> Self {
        const {
            assert!(N + M < i32::MAX as usize, "N+M must be less than i32::MAX");
        }
        Self {
            first_array: uninit_array(),
            second_array: uninit_array(),
            length: Length::ZERO,
        }
    }

    /// Returns the length of the [`DoubleFixedArray`].
    #[inline(always)]
    pub const fn len(&self) -> Length {
        self.length
    }

    /// Returns true if the [`DoubleFixedArray`] is empty.
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.length.value() == 0
    }

    /// Returns true if the current length of the [`DoubleFixedArray`] is equal
    /// its total capacity, i.e. `N + M`.
    #[inline(always)]
    pub const fn is_full(&self) -> bool {
        self.length.value() == (N + M) as i32
    }

    /// Returns [`DoubleFixedArray`] as a slice.
    #[inline(always)]
    pub const fn as_slice(&self) -> &[T] {
        let real_ptr = self.real_slice().as_ptr();
        unsafe { core::slice::from_raw_parts(real_ptr.cast(), self.length.value() as usize) }
    }

    /// Returns [`DoubleFixedArray`] as a mutable slice.
    #[inline(always)]
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        let real_ptr = self.real_slice().as_mut_ptr();
        unsafe { core::slice::from_raw_parts_mut(real_ptr.cast(), self.length.value() as usize) }
    }

    /// Pushes a value to the [`DoubleFixedArray`].
    ///
    /// # Errors
    ///
    /// Returns [`OutOfRangeError`] if the [`DoubleFixedArray`] is already full.
    #[inline(always)]
    pub fn push(&mut self, value: T) -> Result<(), OutOfRangeError> {
        self.extend_from_array([value])
    }

    /// Extends the [`DoubleFixedArray`] with the values from another array.
    ///
    /// # Errors
    ///
    /// Returns [`OutOfRangeError`] if the current length of [`DoubleFixedArray`] plus
    /// the length of the other array execeeds the capacity of the [`DoubleFixedArray`].
    pub fn extend_from_array<const K: usize>(&mut self, other: [T; K]) -> Result<(), OutOfRangeError> {
        let len = self.length.value();
        if len + K as i32 > (N + M) as i32 {
            return Err(OutOfRangeError);
        }

        let real_slice = self.real_slice();

        let mut idx = len as usize;
        for item in other {
            real_slice[idx].write(item);
            idx += 1;
        }

        self.length = unsafe { Length::new_unchecked(idx as i32) };

        Ok(())
    }

    /// Pops a value from the [`DoubleFixedArray`] and decreases its length.
    pub fn pop(&mut self) -> Option<T> {
        let mut len = self.length.value();
        if len == 0 {
            return None;
        }

        len -= 1;

        unsafe {
            self.length = Length::new_unchecked(len);
            let value = self.real_slice()[len as usize].assume_init_read();
            Some(value)
        }
    }

    #[allow(clippy::mut_from_ref)]
    #[inline(always)]
    const fn real_slice(&self) -> &mut [MaybeUninit<T>] {
        let ptr = core::ptr::from_ref(&self.first_array)
            .cast::<MaybeUninit<T>>()
            .cast_mut();
        unsafe { core::slice::from_raw_parts_mut(ptr, N + M) }
    }
}

impl<T, const N: usize, const M: usize> Drop for DoubleFixedArray<T, N, M> {
    fn drop(&mut self) {
        if core::mem::needs_drop::<T>() {
            let real_slice = self.real_slice();
            for i in 0..self.length.value() {
                unsafe {
                    real_slice[i as usize].assume_init_drop();
                }
            }
        }
    }
}

impl<T: Clone, const N: usize, const M: usize> DoubleFixedArray<T, N, M> {
    /// Extends the [`DoubleFixedArray`] with the values from a slice. The values
    /// will be cloned.
    ///
    /// # Errors
    ///
    /// Returns [`OutOfRangeError`] if the current length of [`DoubleFixedArray`] plus
    /// the length of the other array execeeds the capacity of the [`DoubleFixedArray`].
    pub fn extend_from_slice(&mut self, other: &[T]) -> Result<(), OutOfRangeError> {
        let len = self.length.value();
        if len + other.len() as i32 > (N + M) as i32 {
            return Err(OutOfRangeError);
        }

        let real_slice = self.real_slice();

        let mut idx = len as usize;
        for item in other {
            real_slice[idx].write(item.clone());
            idx += 1;
        }

        self.length = unsafe { Length::new_unchecked(idx as i32) };

        Ok(())
    }
}

impl<T: Clone, const N: usize, const M: usize> Clone for DoubleFixedArray<T, N, M> {
    fn clone(&self) -> Self {
        let mut first_array = uninit_array();
        let mut second_array = uninit_array();
        let real_slice = self.real_slice();
        let real_len = self.length.value() as usize;

        let mut idx = 0;
        while idx < N && idx < real_len {
            first_array[idx].write(unsafe { real_slice[idx].assume_init_read() });
            idx += 1;
        }
        while idx < real_len {
            second_array[idx - N].write(unsafe { real_slice[idx].assume_init_read() });
            idx += 1;
        }

        Self {
            first_array,
            second_array,
            length: self.length,
        }
    }
}

impl<T: PartialEq, const N: usize, const M: usize> PartialEq for DoubleFixedArray<T, N, M> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T: Eq, const N: usize, const M: usize> Eq for DoubleFixedArray<T, N, M> {}

impl<T: core::hash::Hash, const N: usize, const M: usize> core::hash::Hash for DoubleFixedArray<T, N, M> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

impl<T, const N: usize, const M: usize> Default for DoubleFixedArray<T, N, M> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize, const M: usize> core::ops::Deref for DoubleFixedArray<T, N, M> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T, const N: usize, const M: usize> core::ops::DerefMut for DoubleFixedArray<T, N, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}
