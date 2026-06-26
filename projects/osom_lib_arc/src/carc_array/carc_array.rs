use core::cmp::Ordering as CmpOrdering;
use core::{borrow::Borrow, ops::Deref};

use osom_lib_alloc::traits::Allocator;
use osom_lib_primitives::align::Align;
use osom_lib_primitives::length::Length;
use osom_lib_reprc::traits::ReprC;
use osom_lib_try_clone::TryClone;

use crate::caligned_arc_array::CAlignedArcArray;
use crate::{carc_array::CWeakArray, errors::MaxReferencesExceededError};

/// A smart pointer that can be used to share ownership of a value.
///
/// This struct is functionally similar to [`CArc<[T]>`][crate::carc::CArc]
/// but comes with a builder for iteratively constructing the array.
#[repr(transparent)]
#[must_use]
#[derive(Debug)]
pub struct CArcArray<T, TAllocator: Allocator> {
    internal: CAlignedArcArray<Align<1>, T, TAllocator>,
}

unsafe impl<T: ReprC, TAllocator: Allocator> ReprC for CArcArray<T, TAllocator> {
    const CHECK: () = const {
        osom_lib_reprc::hidden::is_reprc::<T>();
        osom_lib_reprc::hidden::is_reprc::<CWeakArray<T, TAllocator>>();
        osom_lib_reprc::hidden::is_reprc::<CAlignedArcArray<Align<1>, T, TAllocator>>();
    };
}

impl<T, TAllocator: Allocator> CArcArray<T, TAllocator> {
    #[inline]
    pub(super) const fn from_internal(internal: CAlignedArcArray<Align<1>, T, TAllocator>) -> Self {
        Self { internal }
    }

    /// Returns the number of strong references to the [`CArcArray`].
    #[inline(always)]
    #[must_use]
    pub fn strong_count(carc: &Self) -> u32 {
        CAlignedArcArray::strong_count(&carc.internal)
    }

    /// Returns the number of weak references to the [`CArcArray`].
    #[inline(always)]
    #[must_use]
    pub fn weak_count(carc: &Self) -> u32 {
        CAlignedArcArray::weak_count(&carc.internal)
    }

    /// Returns a reference to the underlying slice.
    #[inline]
    #[must_use]
    pub const fn data(carc: &Self) -> &[T] {
        CAlignedArcArray::data(&carc.internal)
    }

    /// Returns the length of the underlying slice.
    #[inline]
    pub const fn length(carc: &Self) -> Length {
        CAlignedArcArray::length(&carc.internal)
    }

    /// Creates a new [`CWeakArray`] reference to the [`CArcArray`].
    ///
    /// # Errors
    ///
    /// If the weak reference count is too high. Cannot exceed [`MAX_REFERENCES`][crate::consts::MAX_REFERENCES].
    pub fn downgrade(carc: &Self) -> Result<CWeakArray<T, TAllocator>, MaxReferencesExceededError> {
        let weak = CAlignedArcArray::downgrade(&carc.internal)?;
        Ok(CWeakArray::from_internal(weak))
    }

    /// Abandons current [`CArcArray`].
    ///
    /// This function returns `None` if the underlying strong reference counter
    /// is still positive. Otherwise it the final [`CWeakArray`]. In particular
    /// this call drops the underlying data, but does not deallocate the memory.
    #[inline]
    #[must_use]
    pub fn abandon(carc: Self) -> Option<CWeakArray<T, TAllocator>> {
        CAlignedArcArray::abandon(carc.internal).map(CWeakArray::from_internal)
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
        let internal = unsafe { core::ptr::read(&raw const carc.internal) };
        core::mem::forget(carc);
        CAlignedArcArray::into_raw_ptr(internal)
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
        let internal = unsafe { CAlignedArcArray::from_raw_ptr(ptr) };
        Self { internal }
    }
}

impl<T, TAllocator: Allocator> AsRef<[T]> for CArcArray<T, TAllocator> {
    fn as_ref(&self) -> &[T] {
        self.internal.as_ref()
    }
}

impl<T, TAllocator: Allocator> Deref for CArcArray<T, TAllocator> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.internal.deref()
    }
}

impl<T, TAllocator: Allocator> Borrow<[T]> for CArcArray<T, TAllocator> {
    fn borrow(&self) -> &[T] {
        self.internal.borrow()
    }
}

impl<T, TAllocator: Allocator> Clone for CArcArray<T, TAllocator> {
    fn clone(&self) -> Self {
        Self {
            internal: self.internal.clone(),
        }
    }
}

impl<T, TAllocator: Allocator> TryClone for CArcArray<T, TAllocator> {
    type Error = MaxReferencesExceededError;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        Ok(Self {
            internal: self.internal.try_clone()?,
        })
    }
}

impl<T: PartialEq, TAllocator: Allocator> PartialEq for CArcArray<T, TAllocator> {
    fn eq(&self, other: &Self) -> bool {
        self.internal == other.internal
    }
}

impl<T: Eq, TAllocator: Allocator> Eq for CArcArray<T, TAllocator> {}

impl<T: core::hash::Hash, TAllocator: Allocator> core::hash::Hash for CArcArray<T, TAllocator> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.internal.hash(state);
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
