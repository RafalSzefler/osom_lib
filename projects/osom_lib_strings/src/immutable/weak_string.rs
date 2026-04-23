use core::sync::atomic::{Ordering, fence};

use osom_lib_alloc::traits::Allocator;
use osom_lib_reprc::macros::reprc;

use crate::immutable::{ImmutableString, internal_string::InternalString};

#[reprc]
#[repr(transparent)]
#[derive(Debug)]
pub struct WeakImmutableString<TAllocator: Allocator> {
    internal: InternalString<TAllocator>,
}

impl<TAllocator: Allocator> WeakImmutableString<TAllocator> {
    #[inline(always)]
    pub(crate) fn from_internal(internal: InternalString<TAllocator>) -> Self {
        Self { internal }
    }

    #[inline(always)]
    pub fn strong_count(&self) -> u32 {
        self.internal.strong().load(Ordering::Relaxed)
    }

    #[inline(always)]
    pub fn weak_count(&self) -> u32 {
        self.internal.weak().load(Ordering::Relaxed)
    }

    /// Upgrades current weak reference to the strong [`ImmutableString`].
    ///
    /// Returns `None` if this this cannot be done (because there were no strong
    /// references alive). Otherwise returns `Some()` with a strong reference.
    pub fn upgrade(&self) -> Option<ImmutableString<TAllocator>> {
        let strong = self.internal.strong();
        let mut current = strong.load(Ordering::Relaxed);
        loop {
            if current == 0 {
                return None;
            }
            match strong.compare_exchange_weak(current, current + 1, Ordering::Acquire, Ordering::Relaxed) {
                Ok(_) => return Some(ImmutableString::from_internal(self.internal.clone())),
                Err(new) => current = new,
            }
        }
    }

    /// Abandons current weak reference.
    ///
    /// If the internal weak counter is positive it returns false.
    ///
    /// Otherwise it deallocates the underlying memory and returns true.
    #[inline(always)]
    #[must_use]
    pub fn abandon(mut self) -> bool {
        let result = self.internal_abandon();
        core::mem::forget(self);
        result
    }

    fn internal_abandon(&mut self) -> bool {
        let internal = unsafe { core::ptr::read(&raw const self.internal) };
        let prev = internal.weak().fetch_sub(1, Ordering::Release);
        if prev > 1 {
            return false;
        }

        // Synchronize with all prior Release decrements before deallocating.
        fence(Ordering::Acquire);
        internal.deallocate();
        true
    }
}

impl<TAllocator: Allocator> Drop for WeakImmutableString<TAllocator> {
    fn drop(&mut self) {
        let _ = self.internal_abandon();
    }
}

impl<TAllocator: Allocator> Clone for WeakImmutableString<TAllocator> {
    fn clone(&self) -> Self {
        let internal_clone = self.internal.clone();
        internal_clone.weak().fetch_add(1, Ordering::Relaxed);
        Self {
            internal: internal_clone,
        }
    }
}
