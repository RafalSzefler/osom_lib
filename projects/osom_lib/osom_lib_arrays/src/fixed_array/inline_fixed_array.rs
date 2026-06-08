#![allow(clippy::cast_possible_truncation)]

use core::{
    borrow::{Borrow, BorrowMut},
    hash::Hash,
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
};

use osom_lib_primitives::length::Length;
use osom_lib_reprc::traits::ReprC;
use osom_lib_try_clone::TryClone;

use crate::{
    errors::{ArrayError, ArrayIsEmptyError, ArrayTryCloneError},
    traits::{ImmutableArray, MutableArray},
};

/// A fixed-capacity array. This type is similar to [`DynamicArray`][`crate::dynamic_array::DynamicArray`],
/// except its capacity is fixed at compile time, and doesn't change at runtime. The "Inline" prefix
/// indicates that the data is stored inside the struct itself, meaning its size depends on `TSIZE`.
///
/// In particular this type does not require an allocator.
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct InlineFixedArray<const TSIZE: usize, T: Sized> {
    length: Length,
    inner: MaybeUninit<[T; TSIZE]>,
    _phantom: PhantomData<T>,
}

unsafe impl<const TSIZE: usize, T: Send + Sized> Send for InlineFixedArray<TSIZE, T> {}
unsafe impl<const TSIZE: usize, T: Sync + Sized> Sync for InlineFixedArray<TSIZE, T> {}

unsafe impl<const TSIZE: usize, T: ReprC + Sized> ReprC for InlineFixedArray<TSIZE, T> {
    const CHECK: () = const {
        osom_lib_reprc::hidden::is_reprc::<T>();
        osom_lib_reprc::hidden::is_reprc::<PhantomData<T>>();
        osom_lib_reprc::hidden::is_reprc::<Length>();
        osom_lib_reprc::hidden::is_reprc::<MaybeUninit<[T; TSIZE]>>();
    };
}

impl<const TSIZE: usize, T: Sized> InlineFixedArray<TSIZE, T> {
    /// Creates a new, empty [`InlineFixedArray`].
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

    /// Returns the length of the [`InlineFixedArray`].
    #[inline(always)]
    pub const fn length(&self) -> Length {
        self.length
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.length.as_u32() == 0
    }

    /// Returns the capacity of the [`InlineFixedArray`]. This is `TSIZE`
    /// as [`Length`].
    #[inline(always)]
    pub const fn capacity(&self) -> Length {
        unsafe { Length::new_unchecked(TSIZE as u32) }
    }

    /// Returns the [`InlineFixedArray`] as immutable slice.
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

    /// Returns the [`InlineFixedArray`] as mutable slice.
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

impl<const TSIZE: usize, T: Sized> Default for InlineFixedArray<TSIZE, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const TSIZE: usize, T: Sized> ImmutableArray<T> for InlineFixedArray<TSIZE, T> {
    #[inline(always)]
    fn length(&self) -> Length {
        self.length()
    }

    #[inline(always)]
    fn capacity(&self) -> Length {
        self.capacity()
    }
}

impl<const TSIZE: usize, T: Sized> MutableArray<T> for InlineFixedArray<TSIZE, T> {
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

    fn try_push_slice(&mut self, slice: &[T]) -> Result<(), ArrayTryCloneError>
    where
        T: TryClone,
    {
        let len = self.length.as_usize();
        let tsize = slice.len();
        if len + tsize > TSIZE {
            return Err(ArrayError::LengthLimitExceeded.into());
        }

        unsafe {
            let mut dst = self.inner.as_mut_ptr().cast::<T>().add(len);
            for item in slice {
                let clone = item.try_clone().map_err(|_| ArrayTryCloneError::ItemCloningError)?;
                dst.write(clone);
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
}

impl<const TSIZE: usize, T: Sized + TryClone> Clone for InlineFixedArray<TSIZE, T> {
    fn clone(&self) -> Self {
        self.try_clone().expect("Failed to clone inline fixed array")
    }
}

impl<const TSIZE: usize, T: Sized + TryClone> TryClone for InlineFixedArray<TSIZE, T> {
    type Error = ArrayTryCloneError;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        let mut new_instance = Self::new();
        new_instance.try_push_slice(self.as_slice_const())?;
        Ok(new_instance)
    }
}

impl<const TSIZE: usize, T: Sized + PartialEq, Rhs: AsRef<[T]>> PartialEq<Rhs> for InlineFixedArray<TSIZE, T> {
    fn eq(&self, other: &Rhs) -> bool {
        self.as_slice_const() == other.as_ref()
    }
}

impl<const TSIZE: usize, T: Sized + Eq> Eq for InlineFixedArray<TSIZE, T> {}

impl<const TSIZE: usize, T: Sized + Hash> Hash for InlineFixedArray<TSIZE, T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_slice_const().hash(state);
    }
}

impl<const TSIZE: usize, T: Sized> AsRef<[T]> for InlineFixedArray<TSIZE, T> {
    fn as_ref(&self) -> &[T] {
        self.as_slice_const()
    }
}

impl<const TSIZE: usize, T: Sized> AsMut<[T]> for InlineFixedArray<TSIZE, T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_slice_mut_const()
    }
}

impl<const TSIZE: usize, T: Sized> Borrow<[T]> for InlineFixedArray<TSIZE, T> {
    fn borrow(&self) -> &[T] {
        self.as_slice_const()
    }
}

impl<const TSIZE: usize, T: Sized> BorrowMut<[T]> for InlineFixedArray<TSIZE, T> {
    fn borrow_mut(&mut self) -> &mut [T] {
        self.as_slice_mut_const()
    }
}

impl<const TSIZE: usize, T: Sized> Drop for InlineFixedArray<TSIZE, T> {
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
