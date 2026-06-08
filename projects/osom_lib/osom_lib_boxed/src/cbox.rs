//! This module defines the [`CBox`] type.
use core::borrow::{Borrow, BorrowMut};
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;

use osom_lib_alloc::traits::Allocator;
use osom_lib_reprc::traits::ReprC;
use osom_lib_try_clone::TryClone;

use crate::errors::CBoxTryCloneError;

use super::errors::CBoxError;
use super::layout::CBoxLayout;

/// The ABI-stable box type. This type is equivalent to the standard `Box` type,
/// except it is suitable for FFI and also accepts any allocator.
#[repr(transparent)]
#[must_use]
#[derive(Debug)]
pub struct CBox<T, TAllocator: Allocator> {
    data: *mut u8,
    _phantom: PhantomData<(T, TAllocator)>,
}

unsafe impl<T: ReprC, TAllocator: Allocator> ReprC for CBox<T, TAllocator> {
    const CHECK: () = const {
        osom_lib_reprc::hidden::is_reprc::<T>();
        osom_lib_reprc::hidden::is_reprc::<TAllocator>();
    };
}

impl<T, TAllocator: Allocator> CBox<T, TAllocator> {
    const LAYOUT: CBoxLayout<T, TAllocator> = CBoxLayout::new();

    /// Creates a new [`CBox`] with the default allocator.
    ///
    /// # Errors
    ///
    /// For details see [`CBoxError`].
    #[inline]
    pub fn new(value: T) -> Result<Self, CBoxError>
    where
        TAllocator: Default,
    {
        Self::with_allocator(value, TAllocator::default())
    }

    /// Creates a new [`CBox`] with a custom allocator.
    ///
    /// # Errors
    ///
    /// For details see [`CBoxError`].
    #[inline]
    pub fn with_allocator(value: T, allocator: TAllocator) -> Result<Self, CBoxError> {
        #[allow(unused_mut)]
        let mut allocator = allocator;
        let ptr = allocator
            .allocate(Self::LAYOUT.total_layout)
            .map_err(|_| CBoxError::AllocationError)?;
        let mut result = Self {
            data: ptr.as_ptr(),
            _phantom: PhantomData,
        };

        unsafe {
            CBox::allocator_ptr_mut(&mut result).write(allocator);
            CBox::data_ptr_mut(&mut result).write(value);
        }

        Ok(result)
    }

    /// Returns a reference to the data stored in the [`CBox`].
    #[inline(always)]
    #[must_use]
    pub const fn data(box_: &Self) -> &T {
        unsafe { &*CBox::data_ptr(box_) }
    }

    /// Returns a mutable reference to the data stored in the [`CBox`].
    #[inline(always)]
    #[must_use]
    pub const fn data_mut(box_: &mut Self) -> &mut T {
        unsafe { &mut *CBox::data_ptr_mut(box_) }
    }

    /// Returns a raw pointer to the underlying data stored in the [`CBox`].
    ///
    /// The caller must ensure that the pointer does not outlive the [`CBox`].
    #[inline(always)]
    #[must_use]
    pub const fn data_ptr(box_: &Self) -> *const T {
        unsafe { box_.data.add(Self::LAYOUT.data_offset).cast::<T>() }
    }

    /// Converts the [`CBox`] into a raw pointer.
    ///
    /// # Notes
    ///
    /// * This method does not touch internal structure of the [`CBox`], `self` is simply forgotten.
    /// * The caller must ensure that the pointer does not outlive the [`CBox`].
    #[inline(always)]
    #[must_use]
    pub const fn into_raw_ptr(box_: Self) -> *mut u8 {
        let ptr = box_.data;
        core::mem::forget(box_);
        ptr
    }

    /// Converts a raw pointer back to a [`CBox`] container.
    ///
    /// # Safety
    ///
    /// * The caller must ensure that the pointer came from the previous
    ///   call to [`CBox::into_raw_ptr`].
    /// * The caller must ensure that the raw pointer won't be used after
    ///   the call.
    ///
    /// Otherwise the behavior is undefined.
    #[inline(always)]
    pub const unsafe fn from_raw_ptr(ptr: *mut u8) -> Self {
        Self {
            data: ptr,
            _phantom: PhantomData,
        }
    }

    /// Returns a raw mutable pointer to the underlying data stored in the [`CBox`].
    ///
    /// The caller must ensure that the pointer does not outlive the [`CBox`].
    #[inline(always)]
    #[must_use]
    pub const fn data_ptr_mut(box_: &mut Self) -> *mut T {
        unsafe { box_.data.add(Self::LAYOUT.data_offset).cast::<T>() }
    }

    /// Unpacks the [`CBox`] and returns the data.
    /// The underlying memory gets deallocated.
    #[inline]
    #[must_use]
    pub fn unpack(mut box_: Self) -> T {
        let result = unsafe { CBox::drop(&mut box_) };
        core::mem::forget(box_);
        result
    }

    #[inline(always)]
    const fn allocator_ref(box_: &Self) -> &TAllocator {
        unsafe { &*CBox::allocator_ptr(box_) }
    }

    #[inline(always)]
    #[must_use]
    const fn allocator_ptr_mut(box_: &mut Self) -> *mut TAllocator {
        unsafe { box_.data.add(Self::LAYOUT.allocator_offset).cast::<TAllocator>() }
    }

    #[inline(always)]
    #[must_use]
    const fn allocator_ptr(box_: &Self) -> *const TAllocator {
        unsafe { box_.data.add(Self::LAYOUT.allocator_offset).cast::<TAllocator>() }
    }

    #[inline]
    unsafe fn drop(box_: &mut Self) -> T {
        unsafe {
            let data = CBox::data_ptr(box_).read();
            let mut allocator = CBox::allocator_ptr(box_).read();
            allocator.deallocate(NonNull::new_unchecked(box_.data), Self::LAYOUT.total_layout);
            data
        }
    }
}

impl<T, TAllocator: Allocator> Drop for CBox<T, TAllocator> {
    fn drop(&mut self) {
        let _ = unsafe { CBox::drop(self) };
    }
}

impl<T, TAllocator: Allocator> AsRef<T> for CBox<T, TAllocator> {
    fn as_ref(&self) -> &T {
        CBox::data(self)
    }
}

impl<T, TAllocator: Allocator> AsMut<T> for CBox<T, TAllocator> {
    fn as_mut(&mut self) -> &mut T {
        CBox::data_mut(self)
    }
}

impl<T, TAllocator: Allocator> Borrow<T> for CBox<T, TAllocator> {
    fn borrow(&self) -> &T {
        CBox::data(self)
    }
}

impl<T, TAllocator: Allocator> BorrowMut<T> for CBox<T, TAllocator> {
    fn borrow_mut(&mut self) -> &mut T {
        CBox::data_mut(self)
    }
}

impl<T, TAllocator> TryClone for CBox<T, TAllocator>
where
    T: TryClone,
    TAllocator: Allocator + TryClone,
{
    type Error = CBoxTryCloneError;

    #[inline]
    fn try_clone(&self) -> Result<Self, Self::Error> {
        let data = CBox::data(self)
            .try_clone()
            .map_err(|_| CBoxTryCloneError::ItemCloningError)?;
        let allocator = CBox::allocator_ref(self)
            .try_clone()
            .map_err(|_| CBoxTryCloneError::AllocatorCloningError)?;
        Self::with_allocator(data, allocator).map_err(CBoxTryCloneError::from)
    }
}

impl<T, TAllocator> Clone for CBox<T, TAllocator>
where
    T: TryClone,
    TAllocator: Allocator + TryClone,
{
    fn clone(&self) -> Self {
        self.try_clone().expect("Couldn't clone CBox")
    }
}

impl<T, TAllocator: Allocator> Deref for CBox<T, TAllocator> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        CBox::data(self)
    }
}

impl<T, TAllocator: Allocator> DerefMut for CBox<T, TAllocator> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        CBox::data_mut(self)
    }
}
