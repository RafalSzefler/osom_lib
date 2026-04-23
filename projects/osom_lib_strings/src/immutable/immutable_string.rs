use core::{
    borrow::Borrow,
    hash::Hash,
    sync::atomic::{Ordering, fence},
};

use osom_lib_alloc::traits::Allocator;
use osom_lib_primitives::length::Length;
use osom_lib_reprc::macros::reprc;

use crate::immutable::{ImmutableStringError, WeakImmutableString, internal_string::InternalString};

#[reprc]
#[repr(transparent)]
#[derive(Debug)]
pub struct ImmutableString<TAllocator: Allocator> {
    internal: InternalString<TAllocator>,
}

impl<TAllocator: Allocator> ImmutableString<TAllocator> {
    #[inline(always)]
    pub(crate) fn from_internal(internal: InternalString<TAllocator>) -> Self {
        Self { internal }
    }

    /// Creates a new, empty [`ImmutableString`].
    ///
    /// This function allocates under the hood, since all immutable strings are backed by smart pointer.
    ///
    /// # Errors
    ///
    /// For details see [`ImmutableStringError`].
    #[inline(always)]
    pub fn empty() -> Result<Self, ImmutableStringError> {
        Self::empty_with_allocator(TAllocator::default())
    }

    /// Creates a new, empty [`ImmutableString`] with a given allocator.
    ///
    /// This function allocates under the hood, since all immutable strings are backed by smart pointer.
    ///
    /// # Errors
    ///
    /// For details see [`ImmutableStringError`].
    #[inline(always)]
    pub fn empty_with_allocator(allocator: TAllocator) -> Result<Self, ImmutableStringError> {
        let internal = InternalString::with_allocator_and_capacity(Length::ZERO, allocator)?;
        let result = Self::from_internal(internal);
        Ok(result)
    }

    #[inline(always)]
    pub fn strong_count(&self) -> u32 {
        self.internal.strong().load(Ordering::Relaxed)
    }

    #[inline(always)]
    pub fn weak_count(&self) -> u32 {
        self.internal.weak().load(Ordering::Relaxed)
    }

    /// Returns the underlying string.
    #[inline]
    pub const fn as_str(&self) -> &str {
        unsafe {
            let slice = core::slice::from_raw_parts(self.internal.data_start(), self.internal.length().as_usize() - 1);
            core::str::from_utf8_unchecked(slice)
        }
    }

    /// Returns the underlying string as C-string.
    ///
    /// Meaning the string has an additional 0 at the end of the buffer. In particular,
    /// the C-string returned by this method has length +1 compared to [`ImmutableString::as_str`]
    /// call.
    #[inline]
    pub const fn as_c_str(&self) -> &str {
        unsafe {
            let slice = core::slice::from_raw_parts(self.internal.data_start(), self.internal.length().as_usize());
            core::str::from_utf8_unchecked(slice)
        }
    }

    #[inline]
    pub const fn length(&self) -> Length {
        unsafe { Length::new_unchecked(self.internal.length().as_u32() - 1) }
    }

    /// Creates a new [`WeakImmutableString`] out of current.
    #[inline]
    pub fn downgrade(&self) -> WeakImmutableString<TAllocator> {
        let internal_clone = self.internal.clone();
        internal_clone.weak().fetch_add(1, Ordering::Relaxed);
        WeakImmutableString::from_internal(internal_clone)
    }

    /// Abandons current [`ImmutableString`].
    ///
    /// This function returns None if the underlying strong reference counter
    /// is still positive. Otherwise it returns the final [`WeakImmutableString`]
    /// reference.
    #[inline]
    pub fn abandon(mut self) -> Option<WeakImmutableString<TAllocator>> {
        let result = self.internal_abandon();
        core::mem::forget(self);
        result
    }

    fn internal_abandon(&mut self) -> Option<WeakImmutableString<TAllocator>> {
        let internal = unsafe { core::ptr::read(&raw const self.internal) };
        let prev = internal.strong().fetch_sub(1, Ordering::Release);
        if prev > 1 {
            return None;
        }

        // Synchronize with all prior Release decrements before deallocating.
        fence(Ordering::Acquire);
        Some(WeakImmutableString::from_internal(internal))
    }
}

impl<TAllocator: Allocator> Drop for ImmutableString<TAllocator> {
    fn drop(&mut self) {
        let _ = self.internal_abandon();
    }
}

impl<TAllocator: Allocator> Clone for ImmutableString<TAllocator> {
    fn clone(&self) -> Self {
        let internal_clone = self.internal.clone();
        internal_clone.strong().fetch_add(1, Ordering::Relaxed);
        Self {
            internal: internal_clone,
        }
    }
}

impl<TAllocator: Allocator> AsRef<str> for ImmutableString<TAllocator> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<TAllocator: Allocator> Borrow<str> for ImmutableString<TAllocator> {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl<TAllocator: Allocator> Hash for ImmutableString<TAllocator> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

impl<TAllocator: Allocator, TRight: AsRef<str>> PartialEq<TRight> for ImmutableString<TAllocator> {
    fn eq(&self, other: &TRight) -> bool {
        self.as_str() == other.as_ref()
    }
}

impl<TAllocator: Allocator> Eq for ImmutableString<TAllocator> {}
