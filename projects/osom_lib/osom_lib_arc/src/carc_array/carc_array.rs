use core::cmp::Ordering as CmpOrdering;
use core::{
    borrow::Borrow,
    ops::Deref,
    sync::atomic::{Ordering, fence},
};

use osom_lib_alloc::traits::Allocator;
use osom_lib_primitives::length::Length;
use osom_lib_reprc::traits::ReprC;
use osom_lib_try_clone::TryClone;

use crate::{
    carc_array::{CWeakArray, internal::InternalArcArray},
    consts::MAX_REFERENCES,
    errors::MaxReferencesExceededError,
};

/// A smart pointer that can be used to share ownership of a value.
///
/// This struct is functionally similar to [`CArc<[T]>`][crate::carc::CArc]
/// but comes with a builder for iteratively constructing the array.
#[repr(transparent)]
#[must_use]
#[derive(Debug)]
pub struct CArcArray<T, TAllocator: Allocator> {
    internal: InternalArcArray<T, TAllocator>,
}

unsafe impl<T: ReprC, TAllocator: Allocator> ReprC for CArcArray<T, TAllocator> {
    const CHECK: () = const {
        osom_lib_reprc::hidden::is_reprc::<T>();
        osom_lib_reprc::hidden::is_reprc::<CWeakArray<T, TAllocator>>();
        osom_lib_reprc::hidden::is_reprc::<InternalArcArray<T, TAllocator>>();
    };
}

impl<T, TAllocator: Allocator> CArcArray<T, TAllocator> {
    #[inline]
    pub(super) const fn from_internal(internal: InternalArcArray<T, TAllocator>) -> Self {
        Self { internal }
    }

    /// Returns the number of strong references to the [`CArcArray`].
    #[inline(always)]
    #[must_use]
    pub fn strong_count(carc: &Self) -> u32 {
        carc.internal.strong().load(Ordering::Relaxed)
    }

    /// Returns the number of weak references to the [`CArcArray`].
    #[inline(always)]
    #[must_use]
    pub fn weak_count(carc: &Self) -> u32 {
        carc.internal.weak().load(Ordering::Relaxed)
    }

    /// Returns a reference to the underlying slice.
    #[inline]
    #[must_use]
    pub const fn data(carc: &Self) -> &[T] {
        carc.internal.data_slice()
    }

    /// Returns the length of the underlying slice.
    #[inline]
    pub const fn length(carc: &Self) -> Length {
        carc.internal.size()
    }

    /// Creates a new [`CWeakArray`] reference to the [`CArcArray`].
    ///
    /// # Errors
    ///
    /// If the weak reference count is too high. Cannot exceed [`MAX_REFERENCES`].
    pub fn downgrade(carc: &Self) -> Result<CWeakArray<T, TAllocator>, MaxReferencesExceededError> {
        let internal_clone = carc.internal.raw_clone();
        let prev_value = internal_clone.weak().fetch_add(1, Ordering::Relaxed);
        if prev_value >= MAX_REFERENCES {
            internal_clone.weak().fetch_sub(1, Ordering::Relaxed);
            return Err(MaxReferencesExceededError);
        }
        Ok(CWeakArray::from_internal(internal_clone))
    }

    /// Abandons current [`CArcArray`].
    ///
    /// This function returns `None` if the underlying strong reference counter
    /// is still positive. Otherwise it the final [`CWeakArray`]. In particular
    /// this call drops the underlying data, but does not deallocate the memory.
    #[inline]
    #[must_use]
    pub fn abandon(mut carc: Self) -> Option<CWeakArray<T, TAllocator>> {
        let result = unsafe { CArcArray::internal_abandon(&mut carc) };
        core::mem::forget(carc);
        result
    }

    /// Converts the [`CArcArray`] into a raw pointer.
    ///
    /// # Notes
    ///
    /// * This method does not touch internal reference counters, `self` is simply forgotten.
    /// * The caller must ensure that the pointer does not outlive the [`CArcArray`].
    #[inline(always)]
    #[must_use]
    pub const fn into_raw_ptr(carc: Self) -> *mut u8 {
        let ptr = carc.internal.raw_ptr();
        core::mem::forget(carc);
        ptr
    }

    /// Converts a raw pointer back to a [`CArcArray`].
    ///
    /// # Safety
    ///
    /// * The caller must ensure that the pointer came from the previous
    ///   call to [`CArcArray::into_raw_ptr`].
    /// * The caller must ensure that the raw pointer won't be used after
    ///   the call.
    ///
    /// Otherwise the behavior is undefined.
    #[inline(always)]
    pub const unsafe fn from_raw_ptr(ptr: *mut u8) -> Self {
        let internal = InternalArcArray::from_raw_ptr(ptr);
        Self { internal }
    }

    unsafe fn internal_abandon(carc: &mut Self) -> Option<CWeakArray<T, TAllocator>> {
        let mut internal = unsafe { core::ptr::read(&raw const carc.internal) };
        let prev = internal.strong().fetch_sub(1, Ordering::Release);
        if prev > 1 {
            return None;
        }

        if core::mem::needs_drop::<T>() {
            for item in internal.data_slice_mut() {
                unsafe { core::ptr::drop_in_place(item) };
            }
        }

        // Synchronize with all prior Release decrements before deallocating.
        fence(Ordering::Acquire);

        Some(CWeakArray::from_internal(internal))
    }
}

impl<T, TAllocator: Allocator> Drop for CArcArray<T, TAllocator> {
    fn drop(&mut self) {
        let _ = unsafe { CArcArray::internal_abandon(self) };
    }
}

impl<T, TAllocator: Allocator> AsRef<[T]> for CArcArray<T, TAllocator> {
    fn as_ref(&self) -> &[T] {
        self.internal.data_slice()
    }
}

impl<T, TAllocator: Allocator> Deref for CArcArray<T, TAllocator> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.internal.data_slice()
    }
}

impl<T, TAllocator: Allocator> Borrow<[T]> for CArcArray<T, TAllocator> {
    fn borrow(&self) -> &[T] {
        self.internal.data_slice()
    }
}

impl<T, TAllocator: Allocator> Clone for CArcArray<T, TAllocator> {
    fn clone(&self) -> Self {
        self.try_clone()
            .expect("CArcArray strong reference count is too high. Cannot exceed {MAX_REFERENCES}")
    }
}

impl<T, TAllocator: Allocator> TryClone for CArcArray<T, TAllocator> {
    type Error = MaxReferencesExceededError;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        let internal_clone = self.internal.raw_clone();
        let prev_value = internal_clone.strong().fetch_add(1, Ordering::Relaxed);
        if prev_value >= MAX_REFERENCES {
            internal_clone.strong().fetch_sub(1, Ordering::Relaxed);
            return Err(MaxReferencesExceededError);
        }
        Ok(Self {
            internal: internal_clone,
        })
    }
}

impl<T: PartialEq, TAllocator: Allocator> PartialEq for CArcArray<T, TAllocator> {
    fn eq(&self, other: &Self) -> bool {
        if self.internal.raw_equals(&other.internal) {
            return true;
        }
        self.as_ref() == other.as_ref()
    }
}

impl<T: Eq, TAllocator: Allocator> Eq for CArcArray<T, TAllocator> {}

impl<T: core::hash::Hash, TAllocator: Allocator> core::hash::Hash for CArcArray<T, TAllocator> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}

impl<T: PartialOrd, TAllocator: Allocator> PartialOrd for CArcArray<T, TAllocator> {
    fn partial_cmp(&self, other: &Self) -> Option<CmpOrdering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

impl<T: Ord, TAllocator: Allocator> Ord for CArcArray<T, TAllocator> {
    fn cmp(&self, other: &Self) -> CmpOrdering {
        self.as_ref().cmp(other.as_ref())
    }
}
