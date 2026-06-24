//! Holds the definition of [`CArc`].
use core::{
    borrow::Borrow,
    cmp::Ordering as CmpOrdering,
    hash::Hash,
    ops::Deref,
    sync::atomic::{Ordering, fence},
};

use osom_lib_alloc::traits::Allocator;
use osom_lib_reprc::traits::ReprC;
use osom_lib_try_clone::TryClone;

use crate::{
    consts::MAX_REFERENCES,
    errors::{CArcError, MaxReferencesExceededError},
};

use super::{internal::InternalArc, weak::CWeak};

/// A smart pointer that can be used to share ownership of a value.
///
/// This struct is equivalent to the standard `Arc` type,
/// but is `#[repr(C)]` and thus safe to use across the ffi boundaries.
#[repr(transparent)]
#[must_use]
#[derive(Debug)]
pub struct CArc<T, TAllocator: Allocator> {
    internal: InternalArc<T, TAllocator>,
}

unsafe impl<T: ReprC, TAllocator: Allocator> ReprC for CArc<T, TAllocator> {
    const CHECK: () = const {
        osom_lib_reprc::hidden::is_reprc::<T>();
        osom_lib_reprc::hidden::is_reprc::<CWeak<T, TAllocator>>();
        osom_lib_reprc::hidden::is_reprc::<InternalArc<T, TAllocator>>();
    };
}

/// The result of abandoning a [`CArc`]. Holds the underlying data and
/// a weak reference to the [`CArc`].
#[must_use]
#[repr(C)]
#[derive(Debug)]
pub struct AbandonResult<T, TAllocator: Allocator> {
    /// The underlying data.
    pub data: T,

    /// The weak reference to the final strong [`CArc`] reference.
    pub weak: CWeak<T, TAllocator>,
}

unsafe impl<T: ReprC, TAllocator: Allocator> ReprC for AbandonResult<T, TAllocator> {
    const CHECK: () = const {
        osom_lib_reprc::hidden::is_reprc::<T>();
        osom_lib_reprc::hidden::is_reprc::<CWeak<T, TAllocator>>();
    };
}

impl<T, TAllocator: Allocator> CArc<T, TAllocator> {
    /// Creates a new [`CArc`] with the default allocator.
    ///
    /// # Errors
    ///
    /// For details see [`CArcError`].
    #[inline]
    pub fn new(value: T) -> Result<Self, CArcError>
    where
        TAllocator: Default,
    {
        let internal = InternalArc::new(value)?;
        Ok(Self::from_internal(internal))
    }

    /// Creates a new [`CArc`] with a custom allocator.
    ///
    /// # Errors
    ///
    /// For details see [`CArcError`].
    #[inline]
    pub fn with_allocator(value: T, allocator: TAllocator) -> Result<Self, CArcError> {
        let internal = InternalArc::with_allocator(value, allocator)?;
        Ok(Self::from_internal(internal))
    }

    /// Returns the number of strong references to the [`CArc`].
    #[inline(always)]
    #[must_use]
    pub fn strong_count(carc: &Self) -> u32 {
        carc.internal.strong().load(Ordering::Relaxed)
    }

    /// Returns the number of weak references to the [`CArc`].
    #[inline(always)]
    #[must_use]
    pub fn weak_count(carc: &Self) -> u32 {
        carc.internal.weak().load(Ordering::Relaxed)
    }

    /// Returns a reference to the underlying value.
    #[inline]
    #[must_use]
    pub const fn data(carc: &Self) -> &T {
        carc.internal.data()
    }

    /// Creates a new [`CWeak`] reference to the [`CArc`].
    ///
    /// # Errors
    ///
    /// If the weak reference count is too high. Cannot exceed [`MAX_REFERENCES`].
    pub fn downgrade(carc: &Self) -> Result<CWeak<T, TAllocator>, MaxReferencesExceededError> {
        let internal_clone = carc.internal.clone();
        let prev_value = internal_clone.weak().fetch_add(1, Ordering::Relaxed);
        if prev_value >= MAX_REFERENCES {
            internal_clone.weak().fetch_sub(1, Ordering::Relaxed);
            return Err(MaxReferencesExceededError);
        }
        Ok(CWeak::from_internal(internal_clone))
    }

    /// Abandons current [`CArc`].
    ///
    /// This function returns `None` if the underlying strong reference counter
    /// is still positive. Otherwise it returns the underlying data and
    /// a weak reference to the [`CArc`]. In particular the data will be dropped
    /// by the caller if that was the last strong reference.
    #[inline]
    #[must_use]
    pub fn abandon(mut carc: Self) -> Option<AbandonResult<T, TAllocator>> {
        let result = unsafe { CArc::internal_abandon(&mut carc) };
        core::mem::forget(carc);
        result
    }

    /// Converts the [`CArc`] into a raw pointer.
    ///
    /// # Notes
    ///
    /// * This method does not touch internal reference counters, `self` is simply forgotten.
    /// * The caller must ensure that the pointer does not outlive the [`CArc`].
    #[inline(always)]
    #[must_use]
    pub const fn into_raw_ptr(carc: Self) -> *mut u8 {
        let ptr = carc.internal.raw_ptr();
        core::mem::forget(carc);
        ptr
    }

    /// Converts a raw pointer back to a [`CArc`].
    ///
    /// # Safety
    ///
    /// * The caller must ensure that the pointer came from the previous
    ///   call to [`CArc::into_raw_ptr`].
    /// * The caller must ensure that the raw pointer won't be used after
    ///   the call.
    ///
    /// Otherwise the behavior is undefined.
    #[inline(always)]
    pub const unsafe fn from_raw_ptr(ptr: *mut u8) -> Self {
        let internal = InternalArc::from_raw_ptr(ptr);
        Self { internal }
    }

    #[inline(always)]
    pub(super) fn from_internal(internal: InternalArc<T, TAllocator>) -> Self {
        Self { internal }
    }

    unsafe fn internal_abandon(carc: &mut Self) -> Option<AbandonResult<T, TAllocator>> {
        let internal = unsafe { core::ptr::read(&raw const carc.internal) };
        let prev = internal.strong().fetch_sub(1, Ordering::Release);
        if prev > 1 {
            return None;
        }

        let data = unsafe { internal.read_data() };

        // Synchronize with all prior Release decrements before deallocating.
        fence(Ordering::Acquire);

        Some(AbandonResult {
            data,
            weak: CWeak::from_internal(internal),
        })
    }
}

impl<T, TAllocator: Allocator> Drop for CArc<T, TAllocator> {
    fn drop(&mut self) {
        let _ = unsafe { CArc::internal_abandon(self) };
    }
}

impl<T, TAllocator: Allocator> AsRef<T> for CArc<T, TAllocator> {
    fn as_ref(&self) -> &T {
        CArc::data(self)
    }
}

impl<T, TAllocator: Allocator> Deref for CArc<T, TAllocator> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        CArc::data(self)
    }
}

impl<T, TAllocator: Allocator> Borrow<T> for CArc<T, TAllocator> {
    fn borrow(&self) -> &T {
        CArc::data(self)
    }
}

impl<T, TAllocator: Allocator> Clone for CArc<T, TAllocator> {
    fn clone(&self) -> Self {
        self.try_clone().expect("CArc strong reference count is too high.")
    }
}

impl<T, TAllocator: Allocator> TryClone for CArc<T, TAllocator> {
    type Error = MaxReferencesExceededError;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        let internal_clone = self.internal.clone();
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

impl<T: PartialEq, TAllocator: Allocator> PartialEq for CArc<T, TAllocator> {
    fn eq(&self, other: &Self) -> bool {
        self.internal.raw_equals(&other.internal) || CArc::data(self) == CArc::data(other)
    }
}

impl<T: Eq, TAllocator: Allocator> Eq for CArc<T, TAllocator> {}

impl<T: Hash, TAllocator: Allocator> Hash for CArc<T, TAllocator> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        CArc::data(self).hash(state);
    }
}

impl<T: PartialOrd, TAllocator: Allocator> PartialOrd for CArc<T, TAllocator> {
    fn partial_cmp(&self, other: &Self) -> Option<CmpOrdering> {
        CArc::data(self).partial_cmp(CArc::data(other))
    }
}

impl<T: Ord, TAllocator: Allocator> Ord for CArc<T, TAllocator> {
    fn cmp(&self, other: &Self) -> CmpOrdering {
        CArc::data(self).cmp(CArc::data(other))
    }
}
