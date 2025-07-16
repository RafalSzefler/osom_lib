//! A module containing the implementation of the fixed array data structure.
#![allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::cast_sign_loss)]

use core::mem::MaybeUninit;

use osom_lib_primitives::Length;

/// Represents a semi-dynamic array, where the maximum size `N` is known at compile time.
/// A thin wrapper around `[T; N]` but that supports pushing and popping elements. I.e.
/// the actual length can be smaller than `N` and is kept internally as a separate field.
#[derive(Debug)]
#[must_use]
pub struct FixedArray<T, const N: usize> {
    array: [MaybeUninit<T>; N],
    length: Length,
}

#[inline(always)]
pub(super) const fn uninit_array<T, const N: usize>() -> [MaybeUninit<T>; N] {
    unsafe { MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init() }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OutOfRangeError;

impl<T, const N: usize> FixedArray<T, N> {
    /// Creates a new [`FixedArray`] from a smaller or equal array.
    pub const fn from_array<const M: usize>(array: [T; M]) -> Self {
        const {
            assert!(N < i32::MAX as usize, "N must be less than i32::MAX");
            assert!(M <= N, "from_array can only be called on smaller or equal arrays");
        }

        let mut new_array = uninit_array();
        let mut idx = 0;
        while idx < M {
            new_array[idx].write(unsafe { core::ptr::read(&array[idx]) });
            idx += 1;
        }

        core::mem::forget(array);

        Self {
            array: new_array,
            length: unsafe { Length::new_unchecked(M as i32) },
        }
    }

    /// Creates a new empty [`FixedArray`].
    #[inline(always)]
    pub const fn new() -> Self {
        const {
            assert!(N < i32::MAX as usize, "N must be less than i32::MAX");
        }
        Self {
            array: uninit_array(),
            length: Length::ZERO,
        }
    }

    /// Returns the length of the [`FixedArray`].
    #[inline(always)]
    pub const fn len(&self) -> Length {
        self.length
    }

    /// Returns the capacity of the [`FixedArray`].
    #[inline(always)]
    pub const fn capacity(&self) -> Length {
        unsafe { Length::new_unchecked(N as i32) }
    }

    /// Returns true if the [`FixedArray`] is empty.
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.length.value() == 0
    }

    /// Returns true if the current length of the [`FixedArray`] is equal its capacity.
    #[inline(always)]
    pub const fn is_full(&self) -> bool {
        self.length.value() == N as i32
    }

    /// Returns [`FixedArray`] as a slice.
    #[inline(always)]
    pub const fn as_slice(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(self.array.as_ptr().cast(), self.length.value() as usize) }
    }

    /// Returns [`FixedArray`] as a mutable slice.
    #[inline(always)]
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { core::slice::from_raw_parts_mut(self.array.as_mut_ptr().cast(), self.length.value() as usize) }
    }

    /// Pushes a value to the [`FixedArray`].
    ///
    /// # Errors
    ///
    /// Returns [`OutOfRangeError`] if the [`FixedArray`] is already full.
    #[inline(always)]
    pub fn push(&mut self, value: T) -> Result<(), OutOfRangeError> {
        self.extend_from_array([value])
    }

    /// Extends the [`FixedArray`] with the values from another array.
    ///
    /// # Errors
    ///
    /// Returns [`OutOfRangeError`] if the current length of [`FixedArray`] plus
    /// the length of the other array execeeds the capacity of the [`FixedArray`].
    pub fn extend_from_array<const M: usize>(&mut self, other: [T; M]) -> Result<(), OutOfRangeError> {
        let len = self.length.value();
        if len + M as i32 > N as i32 {
            return Err(OutOfRangeError);
        }

        let mut idx = len as usize;
        for item in other {
            self.array[idx].write(item);
            idx += 1;
        }

        self.length = unsafe { Length::new_unchecked(idx as i32) };

        Ok(())
    }

    /// Pops a value from the [`FixedArray`] and decreases its length.
    ///
    /// # Returns
    ///
    /// Returns the popped value if the [`FixedArray`] is not empty.
    pub fn pop(&mut self) -> Option<T> {
        let mut len = self.length.value();
        if len == 0 {
            return None;
        }

        len -= 1;

        unsafe {
            self.length = Length::new_unchecked(len);
            let value = self.array[len as usize].assume_init_read();
            Some(value)
        }
    }
}

impl<T: Clone, const N: usize> FixedArray<T, N> {
    /// Extends the [`FixedArray`] with the values from a slice. The values
    /// will be cloned.
    ///
    /// # Errors
    ///
    /// Returns [`OutOfRangeError`] if the current length of [`FixedArray`] plus
    /// the length of the other array execeeds the capacity of the [`FixedArray`].
    pub fn extend_from_slice(&mut self, other: &[T]) -> Result<(), OutOfRangeError> {
        let len = self.length.value();
        if len + other.len() as i32 > N as i32 {
            return Err(OutOfRangeError);
        }

        let mut idx = len as usize;
        for item in other {
            self.array[idx].write(item.clone());
            idx += 1;
        }

        self.length = unsafe { Length::new_unchecked(idx as i32) };

        Ok(())
    }
}

impl<T, const N: usize> Drop for FixedArray<T, N> {
    fn drop(&mut self) {
        if core::mem::needs_drop::<T>() {
            for i in 0..self.length.value() {
                unsafe {
                    self.array[i as usize].assume_init_drop();
                }
            }
        }
    }
}

impl<T: Clone, const N: usize> Clone for FixedArray<T, N> {
    fn clone(&self) -> Self {
        let mut array = uninit_array();
        for i in 0..self.length.value() {
            unsafe {
                array[i as usize].write(self.array[i as usize].assume_init_read());
            }
        }

        Self {
            array: array,
            length: self.length,
        }
    }
}

impl<T: PartialEq, const N: usize> PartialEq for FixedArray<T, N> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T: Eq, const N: usize> Eq for FixedArray<T, N> {}

impl<T: core::hash::Hash, const N: usize> core::hash::Hash for FixedArray<T, N> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

impl<T, const N: usize> Default for FixedArray<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> core::ops::Deref for FixedArray<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T, const N: usize> core::ops::DerefMut for FixedArray<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<T, const N: usize> AsRef<[T]> for FixedArray<T, N> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, const N: usize> AsMut<[T]> for FixedArray<T, N> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}
