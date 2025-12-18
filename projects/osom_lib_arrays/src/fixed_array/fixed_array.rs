#![allow(clippy::cast_possible_truncation)]

use core::{
    borrow::{Borrow, BorrowMut},
    hash::Hash,
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
    ops::{Index, IndexMut},
};

use osom_lib_primitives::length::Length;
use osom_lib_reprc::traits::ReprC;

use crate::{
    errors::{ArrayError, ArrayIsEmptyError},
    traits::{ImmutableArray, MutableArray},
};

/// A fixed-capacity array. This type is similar to [`DynamicArray`][`crate::dynamic_array::DynamicArray`],
/// except its capacity is fixed at compile time, and doesn't change at runtime.
///
/// Additionally this type does not require an allocator (the data is inlined inside the struct).
#[repr(C)]
#[must_use]
pub struct FixedArray<const TSIZE: usize, T: Sized> {
    length: Length,
    inner: MaybeUninit<[T; TSIZE]>,
    _phantom: PhantomData<T>,
}

unsafe impl<const TSIZE: usize, T: ReprC + Sized> ReprC for FixedArray<TSIZE, T> {
    const CHECK: () = const {
        let () = T::CHECK;
        let () = <PhantomData<T> as ReprC>::CHECK;
        let () = <Length as ReprC>::CHECK;
        let () = <MaybeUninit<[T; TSIZE]> as ReprC>::CHECK;
    };
}

impl<const TSIZE: usize, T: Sized> FixedArray<TSIZE, T> {
    /// Creates a new, empty [`FixedArray`].
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

        Self {
            length: Length::ZERO,
            inner: MaybeUninit::uninit(),
            _phantom: PhantomData,
        }
    }

    /// Returns the length of the [`FixedArray`].
    #[inline(always)]
    pub const fn length(&self) -> Length {
        self.length
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.length.as_u32() == 0
    }

    /// Returns the capacity of the [`FixedArray`]. This is `TSIZE`
    /// as [`Length`].
    #[inline(always)]
    pub const fn capacity(&self) -> Length {
        unsafe { Length::new_unchecked(TSIZE as u32) }
    }

    /// Returns the [`FixedArray`] as immutable slice.
    #[inline(always)]
    pub const fn as_slice_const(&self) -> &[T] {
        // In const context we cannot take `.as_ptr()` directly,
        // and we have to do &raw casts. The following checks are only to
        // ensure that both types have the same layout. Just in case.
        const {
            assert!(
                size_of::<[T; TSIZE]>() == size_of::<ManuallyDrop<[T; TSIZE]>>(),
                "T and ManuallyDrop<[T; TSIZE]> must have the same size"
            );
            assert!(
                align_of::<[T; TSIZE]>() == align_of::<ManuallyDrop<[T; TSIZE]>>(),
                "T and ManuallyDrop<[T; TSIZE]> must have the same alignment"
            );
        }
        let ptr = (&raw const self.inner).cast();
        unsafe { core::slice::from_raw_parts(ptr, self.length.as_usize()) }
    }

    /// Returns the [`FixedArray`] as mutable slice.
    #[inline(always)]
    pub const fn as_slice_mut_const(&mut self) -> &mut [T] {
        // In const context we cannot take `.as_ptr()` directly,
        // and we have to do &raw casts. The following checks are only to
        // ensure that both types have the same layout. Just in case.
        const {
            assert!(
                size_of::<[T; TSIZE]>() == size_of::<ManuallyDrop<[T; TSIZE]>>(),
                "T and ManuallyDrop<[T; TSIZE]> must have the same size"
            );
            assert!(
                align_of::<[T; TSIZE]>() == align_of::<ManuallyDrop<[T; TSIZE]>>(),
                "T and ManuallyDrop<[T; TSIZE]> must have the same alignment"
            );
        }
        let ptr = (&raw mut self.inner).cast();
        unsafe { core::slice::from_raw_parts_mut(ptr, self.length.as_usize()) }
    }
}

impl<const TSIZE: usize, T: Sized> Default for FixedArray<TSIZE, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const TSIZE: usize, T: Sized> Index<Length> for FixedArray<TSIZE, T> {
    type Output = T;

    fn index(&self, index: Length) -> &T {
        &self.as_slice_const()[index.as_usize()]
    }
}

impl<const TSIZE: usize, T: Sized> IndexMut<Length> for FixedArray<TSIZE, T> {
    fn index_mut(&mut self, index: Length) -> &mut T {
        &mut self.as_slice_mut_const()[index.as_usize()]
    }
}

impl<const TSIZE: usize, T: Sized> ImmutableArray<T> for FixedArray<TSIZE, T> {
    #[inline(always)]
    fn length(&self) -> Length {
        self.length()
    }

    #[inline(always)]
    fn capacity(&self) -> Length {
        self.capacity()
    }

    #[inline(always)]
    fn as_slice(&self) -> &[T] {
        self.as_slice_const()
    }
}

impl<const TSIZE: usize, T: Sized> MutableArray<T> for FixedArray<TSIZE, T> {
    fn try_push_array<const TARRSIZE: usize>(&mut self, arr: [T; TARRSIZE]) -> Result<(), ArrayError> {
        let len = self.length.as_usize();
        if len + TARRSIZE > TSIZE {
            return Err(ArrayError::LengthLimitExceeded);
        }

        unsafe {
            let mut dst = self.inner.as_mut_ptr().cast::<T>().add(len);
            let mut src = arr.as_ptr();
            let end = src.add(TARRSIZE);
            while src < end {
                dst.write(src.read());
                dst = dst.add(1);
                src = src.add(1);
            }
            core::mem::forget(arr);
            self.length = Length::new_unchecked((len + TARRSIZE) as u32);
        }
        Ok(())
    }

    fn try_push_slice(&mut self, slice: &[T]) -> Result<(), ArrayError>
    where
        T: Clone,
    {
        let len = self.length.as_usize();
        let tsize = slice.len();
        if len + tsize > TSIZE {
            return Err(ArrayError::LengthLimitExceeded);
        }

        unsafe {
            let mut dst = self.inner.as_mut_ptr().cast::<T>().add(len);
            for item in slice {
                dst.write(item.clone());
                dst = dst.add(1);
            }
            self.length = Length::new_unchecked((len + tsize) as u32);
        }
        Ok(())
    }

    fn try_pop(&mut self) -> Result<T, ArrayIsEmptyError> {
        if self.length == Length::ZERO {
            return Err(ArrayIsEmptyError);
        }

        let len = self.length.as_u32();
        let item = unsafe {
            self.length = Length::new_unchecked(len - 1);
            self.inner.as_ptr().cast::<T>().add((len - 1) as usize).read()
        };
        Ok(item)
    }

    #[inline(always)]
    fn as_slice_mut(&mut self) -> &mut [T] {
        self.as_slice_mut_const()
    }
}

impl<const TSIZE: usize, T: Sized + Clone> Clone for FixedArray<TSIZE, T> {
    fn clone(&self) -> Self {
        let mut new_instance = Self::new();
        new_instance
            .try_push_slice(self.as_slice_const())
            .expect("Failed to clone fixed array");
        new_instance
    }
}

impl<const TSIZE: usize, T: Sized + PartialEq, Rhs: AsRef<[T]>> PartialEq<Rhs> for FixedArray<TSIZE, T> {
    fn eq(&self, other: &Rhs) -> bool {
        self.as_slice_const() == other.as_ref()
    }
}

impl<const TSIZE: usize, T: Sized + Eq> Eq for FixedArray<TSIZE, T> {}

impl<const TSIZE: usize, T: Sized + Hash> Hash for FixedArray<TSIZE, T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_slice_const().hash(state);
    }
}

impl<const TSIZE: usize, T: Sized> AsRef<[T]> for FixedArray<TSIZE, T> {
    fn as_ref(&self) -> &[T] {
        self.as_slice_const()
    }
}

impl<const TSIZE: usize, T: Sized> AsMut<[T]> for FixedArray<TSIZE, T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_slice_mut_const()
    }
}

impl<const TSIZE: usize, T: Sized> Borrow<[T]> for FixedArray<TSIZE, T> {
    fn borrow(&self) -> &[T] {
        self.as_slice_const()
    }
}

impl<const TSIZE: usize, T: Sized> BorrowMut<[T]> for FixedArray<TSIZE, T> {
    fn borrow_mut(&mut self) -> &mut [T] {
        self.as_slice_mut_const()
    }
}

impl<const TSIZE: usize, T: Sized> Drop for FixedArray<TSIZE, T> {
    fn drop(&mut self) {
        if !core::mem::needs_drop::<T>() {
            return;
        }

        unsafe {
            let mut start = (&raw mut self.inner).cast::<T>();
            let end = start.add(self.length.as_usize());
            while start < end {
                core::ptr::drop_in_place(start);
                start = start.add(1);
            }
        }
    }
}
