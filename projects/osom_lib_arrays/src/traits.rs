//! Defines mutable and immutable traits for arrays.

use osom_lib_primitives::length::Length;

use crate::errors::{ArrayError, ArrayIsEmptyError};

/// Represents a simply contiguous block of memory.
pub trait ImmutableArray<T> {
    /// Returns array's length as [`Length`].
    fn length(&self) -> Length;

    /// Returns the current capacity for holding items in the array.
    fn capacity(&self) -> Length;

    /// Represents the array as immutable slice.
    #[must_use]
    fn as_slice(&self) -> &[T];

    /// Returns `true` if array is empty, `false` otherwise.
    /// Should be consistent with `self.length() == Length::ZERO`
    /// check.
    #[inline(always)]
    #[must_use]
    fn is_empty(&self) -> bool {
        self.length() == Length::ZERO
    }
}

/// Represents a simply contiguous block of memory that is not only
/// mutable internally but can also grow/shrink in size.
pub trait MutableArray<T>: ImmutableArray<T> {
    /// Pushes raw array to the array.
    ///
    /// # Errors
    ///
    /// For errors see [`ArrayError`].
    fn try_push_array<const TSIZE: usize>(&mut self, arr: [T; TSIZE]) -> Result<(), ArrayError>;

    /// Pushes slice to the array. This method requires [`Clone`]
    /// trait on `T`.
    ///
    /// # Errors
    ///
    /// For errors see [`ArrayError`].
    fn try_push_slice(&mut self, slice: &[T]) -> Result<(), ArrayError>
    where
        T: Clone;

    /// Removes element from the top of the array.
    ///
    /// # Errors
    ///
    /// Returns [`ArrayIsEmptyError`] when the array is empty.
    fn try_pop(&mut self) -> Result<T, ArrayIsEmptyError>;

    /// Represents the array as mutable slice.
    fn as_slice_mut(&mut self) -> &mut [T];

    /// Removes element from the top of the array.
    ///
    /// # Panics
    ///
    /// Panics if is the array is empty. Should be consistent with [`MutableArray::try_pop`].
    #[inline(always)]
    #[must_use]
    fn pop(&mut self) -> T {
        self.try_pop()
            .expect("Couldn't pop from the array, since it was empty.")
    }

    /// Pushes a single element to the array.
    ///
    /// # Errors
    ///
    /// For errors see [`ArrayError`].
    #[inline(always)]
    fn try_push(&mut self, value: T) -> Result<(), ArrayError> {
        self.try_push_array([value])
    }

    /// Pushes raw array to the array.
    ///
    /// # Panics
    ///
    /// Panics whenever [`MutableArray::try_push_array`] would.
    #[inline(always)]
    fn push_array<const TSIZE: usize>(&mut self, arr: [T; TSIZE]) {
        self.try_push_array(arr)
            .expect("Failed to push_array due to array error.");
    }

    /// Pushes raw slice to the array. This method requires [`Clone`]
    /// trait on `T`.
    ///
    /// # Panics
    ///
    /// Panics whenever [`MutableArray::try_push_slice`] would.
    #[inline(always)]
    fn push_slice(&mut self, slice: &[T])
    where
        T: Clone,
    {
        self.try_push_slice(slice)
            .expect("Failed to push_slice due to array error.");
    }

    /// Pushes a single element to the array.
    ///
    /// # Panics
    ///
    /// Panics whenever [`MutableArray::try_push`] would.
    #[inline(always)]
    fn push(&mut self, value: T) {
        self.try_push(value).expect("Failed to push due to array error.");
    }
}
