//! Holds the definition of [`CWeak`].
use core::sync::atomic::{Ordering, fence};

use osom_lib_alloc::traits::Allocator;
use osom_lib_reprc::traits::ReprC;
use osom_lib_try_clone::TryClone;

use crate::{
    consts::MAX_REFERENCES,
    errors::{MaxReferencesExceededError, WeakUpgradeError},
};

use super::{carc::CArc, internal::InternalArc};

/// A weak reference to the underlying [`CArc`].
///
/// This object cannot inspect the underlying value (unless `T` is `Copy`).
/// But it does track weak references, and each weak reference can build
/// a strong reference, assuming any other strong reference is alive.
#[repr(transparent)]
#[must_use]
#[derive(Debug)]
pub struct CWeak<T, TAllocator: Allocator> {
    internal: InternalArc<T, TAllocator>,
}

unsafe impl<T: ReprC, TAllocator: Allocator> ReprC for CWeak<T, TAllocator> {
    const CHECK: () = const {
        osom_lib_reprc::hidden::is_reprc::<T>();
        osom_lib_reprc::hidden::is_reprc::<InternalArc<T, TAllocator>>();
    };
}

impl<T, TAllocator: Allocator> CWeak<T, TAllocator> {
    /// Returns the number of strong references to the [`CWeak`].
    #[inline(always)]
    #[must_use]
    pub fn strong_count(&self) -> u32 {
        self.internal.strong().load(Ordering::Relaxed)
    }

    /// Returns the number of weak references to the [`CWeak`].
    #[inline(always)]
    #[must_use]
    pub fn weak_count(&self) -> u32 {
        self.internal.weak().load(Ordering::Relaxed)
    }

    /// Returns a reference to the underlying data.
    ///
    /// This function is only available if `T` implements `Copy`.
    /// That is because being `Copy` means it is not `Drop`. And
    /// so we don't need strong references to keep the data alive.
    #[inline]
    #[must_use]
    pub fn data(&self) -> &T
    where
        T: Copy,
    {
        self.internal.data()
    }

    /// Upgrades current weak reference to the strong [`CArc`].
    ///
    /// # Errors
    ///
    /// For details see [`WeakUpgradeError`].
    pub fn upgrade(&self) -> Result<CArc<T, TAllocator>, WeakUpgradeError> {
        let strong = self.internal.strong();
        let mut current = strong.load(Ordering::Relaxed);

        loop {
            if current == 0 {
                return Err(WeakUpgradeError::NoStrongReferencesAlive);
            }

            if current >= MAX_REFERENCES {
                return Err(WeakUpgradeError::MaxReferencesExceeded);
            }

            match strong.compare_exchange_weak(current, current + 1, Ordering::Acquire, Ordering::Relaxed) {
                Ok(_) => return Ok(CArc::from_internal(self.internal.clone())),
                Err(new) => current = new,
            }
        }
    }

    /// Abandons current weak reference.
    ///
    /// If the internal weak counter is positive it returns false.
    ///
    /// Otherwise it deallocates the underlying memory and returns true.
    /// In particular only single (the last) [`CWeak`] returns true
    /// by calling this.
    #[inline(always)]
    #[must_use]
    pub fn abandon(mut self) -> bool {
        let result = self.internal_abandon();
        core::mem::forget(self);
        result
    }

    #[inline]
    pub(super) fn from_internal(internal: InternalArc<T, TAllocator>) -> Self {
        Self { internal }
    }

    fn internal_abandon(&mut self) -> bool {
        let internal = unsafe { core::ptr::read(&raw const self.internal) };
        let prev = internal.weak().fetch_sub(1, Ordering::Release);
        if prev > 1 {
            return false;
        }

        // Synchronize with all prior Release decrements before deallocating.
        fence(Ordering::Acquire);
        unsafe { internal.deallocate_memory() };
        true
    }
}

impl<T, TAllocator: Allocator> Drop for CWeak<T, TAllocator> {
    fn drop(&mut self) {
        let _ = self.internal_abandon();
    }
}

impl<T, TAllocator: Allocator> Clone for CWeak<T, TAllocator> {
    fn clone(&self) -> Self {
        self.try_clone()
            .expect("CWeak weak reference count is too high. Cannot exceed {MAX_REFERENCES}")
    }
}

impl<T, TAllocator: Allocator> TryClone for CWeak<T, TAllocator> {
    type Error = MaxReferencesExceededError;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        let internal_clone = self.internal.clone();
        let prev_value = internal_clone.weak().fetch_add(1, Ordering::Relaxed);
        if prev_value >= MAX_REFERENCES {
            internal_clone.weak().fetch_sub(1, Ordering::Relaxed);
            return Err(MaxReferencesExceededError);
        }
        Ok(Self {
            internal: internal_clone,
        })
    }
}
