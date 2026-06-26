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

use crate::{consts::MAX_REFERENCES, errors::MaxReferencesExceededError};

use super::{CAlignedWeakArray, internal::InternalAlignedArcArray};

/// A smart pointer that can be used to share ownership of a value.
///
/// This struct is functionally similar to [`CArc<[T]>`][crate::carc::CArc]
/// but comes with a builder for iteratively constructing the array.
///
/// In addition it accepts `TAlign` generic parameter, which is used to enforce a specific alignment of the
/// internal buffer (note: the alignment won't be lower than the alignment of the item type).
#[repr(transparent)]
#[must_use]
#[derive(Debug)]
pub struct CAlignedArcArray<TAlign, TItem, TAllocator: Allocator> {
    internal: InternalAlignedArcArray<TAlign, TItem, TAllocator>,
}

unsafe impl<TAlign, TItem: ReprC, TAllocator: Allocator> ReprC for CAlignedArcArray<TAlign, TItem, TAllocator> {
    const CHECK: () = const {
        osom_lib_reprc::hidden::is_reprc::<TItem>();
        osom_lib_reprc::hidden::is_reprc::<CAlignedWeakArray<TAlign, TItem, TAllocator>>();
        osom_lib_reprc::hidden::is_reprc::<InternalAlignedArcArray<TAlign, TItem, TAllocator>>();
    };
}

impl<TAlign, TItem, TAllocator: Allocator> CAlignedArcArray<TAlign, TItem, TAllocator> {
    #[inline]
    pub(super) const fn from_internal(internal: InternalAlignedArcArray<TAlign, TItem, TAllocator>) -> Self {
        Self { internal }
    }

    /// Returns the number of strong references to the [`CAlignedArcArray`].
    #[inline(always)]
    #[must_use]
    pub fn strong_count(carc: &Self) -> u32 {
        carc.internal.strong().load(Ordering::Relaxed)
    }

    /// Returns the number of weak references to the [`CAlignedArcArray`].
    #[inline(always)]
    #[must_use]
    pub fn weak_count(carc: &Self) -> u32 {
        carc.internal.weak().load(Ordering::Relaxed)
    }

    /// Returns a reference to the underlying slice.
    #[inline]
    #[must_use]
    pub const fn data(carc: &Self) -> &[TItem] {
        carc.internal.data_slice()
    }

    /// Returns the length of the underlying slice.
    #[inline]
    pub const fn length(carc: &Self) -> Length {
        carc.internal.size()
    }

    /// Creates a new [`CAlignedWeakArray`] reference to the [`CAlignedArcArray`].
    ///
    /// # Errors
    ///
    /// If the weak reference count is too high. Cannot exceed [`MAX_REFERENCES`].
    pub fn downgrade(carc: &Self) -> Result<CAlignedWeakArray<TAlign, TItem, TAllocator>, MaxReferencesExceededError> {
        let internal_clone = carc.internal.raw_clone();
        let prev_value = internal_clone.weak().fetch_add(1, Ordering::Relaxed);
        if prev_value >= MAX_REFERENCES {
            internal_clone.weak().fetch_sub(1, Ordering::Relaxed);
            return Err(MaxReferencesExceededError);
        }
        Ok(CAlignedWeakArray::from_internal(internal_clone))
    }

    /// Abandons current [`CAlignedArcArray`].
    ///
    /// This function returns `None` if the underlying strong reference counter
    /// is still positive. Otherwise it the final [`CAlignedWeakArray`]. In particular
    /// this call drops the underlying data, but does not deallocate the memory.
    #[inline]
    #[must_use]
    pub fn abandon(mut carc: Self) -> Option<CAlignedWeakArray<TAlign, TItem, TAllocator>> {
        let result = unsafe { CAlignedArcArray::internal_abandon(&mut carc) };
        core::mem::forget(carc);
        result
    }

    /// Converts the [`CAlignedArcArray`] into a raw pointer.
    ///
    /// # Notes
    ///
    /// * This method does not touch internal reference counters, `self` is simply forgotten.
    /// * The caller must ensure that the pointer does not outlive the [`CAlignedArcArray`].
    #[inline(always)]
    #[must_use]
    pub const fn into_raw_ptr(carc: Self) -> *mut u8 {
        let ptr = carc.internal.raw_ptr();
        core::mem::forget(carc);
        ptr
    }

    /// Converts a raw pointer back to a [`CAlignedArcArray`].
    ///
    /// # Safety
    ///
    /// * The caller must ensure that the pointer came from the previous
    ///   call to [`CAlignedArcArray::into_raw_ptr`].
    /// * The caller must ensure that the raw pointer won't be used after
    ///   the call.
    ///
    /// Otherwise the behavior is undefined.
    #[inline(always)]
    pub const unsafe fn from_raw_ptr(ptr: *mut u8) -> Self {
        let internal = InternalAlignedArcArray::from_raw_ptr(ptr);
        Self { internal }
    }

    unsafe fn internal_abandon(carc: &mut Self) -> Option<CAlignedWeakArray<TAlign, TItem, TAllocator>> {
        let mut internal = unsafe { core::ptr::read(&raw const carc.internal) };
        let prev = internal.strong().fetch_sub(1, Ordering::Release);
        if prev > 1 {
            return None;
        }

        if core::mem::needs_drop::<TItem>() {
            for item in internal.data_slice_mut() {
                unsafe { core::ptr::drop_in_place(item) };
            }
        }

        // Synchronize with all prior Release decrements before deallocating.
        fence(Ordering::Acquire);

        Some(CAlignedWeakArray::from_internal(internal))
    }
}

impl<TAlign, TItem, TAllocator: Allocator> Drop for CAlignedArcArray<TAlign, TItem, TAllocator> {
    fn drop(&mut self) {
        let _ = unsafe { CAlignedArcArray::internal_abandon(self) };
    }
}

impl<TAlign, TItem, TAllocator: Allocator> AsRef<[TItem]> for CAlignedArcArray<TAlign, TItem, TAllocator> {
    fn as_ref(&self) -> &[TItem] {
        self.internal.data_slice()
    }
}

impl<TAlign, TItem, TAllocator: Allocator> Deref for CAlignedArcArray<TAlign, TItem, TAllocator> {
    type Target = [TItem];

    fn deref(&self) -> &Self::Target {
        self.internal.data_slice()
    }
}

impl<TAlign, TItem, TAllocator: Allocator> Borrow<[TItem]> for CAlignedArcArray<TAlign, TItem, TAllocator> {
    fn borrow(&self) -> &[TItem] {
        self.internal.data_slice()
    }
}

impl<TAlign, TItem, TAllocator: Allocator> Clone for CAlignedArcArray<TAlign, TItem, TAllocator> {
    fn clone(&self) -> Self {
        self.try_clone().expect(
            "CAlignedArcArray strong reference count is too high. Cannot exceed osom_lib_arc::consts::MAX_REFERENCES.",
        )
    }
}

impl<TAlign, TItem, TAllocator: Allocator> TryClone for CAlignedArcArray<TAlign, TItem, TAllocator> {
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

impl<TAlign, TItem: PartialEq, TAllocator: Allocator> PartialEq for CAlignedArcArray<TAlign, TItem, TAllocator> {
    fn eq(&self, other: &Self) -> bool {
        if self.internal.raw_equals(&other.internal) {
            return true;
        }
        self.as_ref() == other.as_ref()
    }
}

impl<TAlign, TItem: Eq, TAllocator: Allocator> Eq for CAlignedArcArray<TAlign, TItem, TAllocator> {}

impl<TAlign, TItem: core::hash::Hash, TAllocator: Allocator> core::hash::Hash
    for CAlignedArcArray<TAlign, TItem, TAllocator>
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}

impl<TAlign, TItem: PartialOrd, TAllocator: Allocator> PartialOrd for CAlignedArcArray<TAlign, TItem, TAllocator> {
    fn partial_cmp(&self, other: &Self) -> Option<CmpOrdering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

impl<TAlign, TItem: Ord, TAllocator: Allocator> Ord for CAlignedArcArray<TAlign, TItem, TAllocator> {
    fn cmp(&self, other: &Self) -> CmpOrdering {
        self.as_ref().cmp(other.as_ref())
    }
}
