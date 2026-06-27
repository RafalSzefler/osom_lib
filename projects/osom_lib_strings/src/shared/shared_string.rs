#![allow(clippy::cast_possible_truncation)]

use core::{borrow::Borrow, fmt::Display, hash::Hash};

use osom_lib_alloc::traits::Allocator;
use osom_lib_arc::carc_array::CArcArray;
use osom_lib_primitives::length::Length;
use osom_lib_reprc::macros::reprc;
use osom_lib_try_clone::TryClone;

use super::{MaxReferencesExceededError, SharedStringError, WeakSharedString};

/// This struct is a smart pointer around a string. Internally keeps reference counters
/// for both strong and weak references. Cloning the struct is therefore cheap.
///
/// NOTE: the shared string internally keeps one byte more than the specified data. This last
/// byte is always set to `\0` and thus the string is suitable to use as a C-style string,
/// by calling [`SharedString::as_c_str()`][SharedString::as_c_str] method.
#[reprc]
#[repr(transparent)]
#[derive(Debug)]
#[must_use]
pub struct SharedString<TAllocator: Allocator> {
    internal: CArcArray<u8, TAllocator>,
}

impl<TAllocator: Allocator> SharedString<TAllocator> {
    #[inline(always)]
    pub(crate) fn from_internal(internal: CArcArray<u8, TAllocator>) -> Self {
        Self { internal }
    }

    /// Creates a new, empty [`SharedString`].
    ///
    /// This function allocates under the hood, since all shared strings are backed by smart pointer.
    ///
    /// # Errors
    ///
    /// For details see [`SharedStringError`].
    #[inline]
    pub fn empty() -> Result<Self, SharedStringError>
    where
        TAllocator: Default,
    {
        Self::empty_with_allocator(TAllocator::default())
    }

    /// Creates a new [`SharedString`] out of passed string slice.
    ///
    /// This function allocates under the hood, since all shared strings are backed by smart pointer.
    /// It also copies the input into the output's buffer.
    ///
    /// # Errors
    ///
    /// For details see [`SharedStringError`].
    #[inline]
    pub fn from_str_slice(text: &str) -> Result<Self, SharedStringError>
    where
        TAllocator: Default,
    {
        Self::from_str_slice_and_allocator(text, TAllocator::default())
    }

    /// Creates a new [`SharedString`] out of passed string slice and allocator.
    ///
    /// This function allocates under the hood, since all shared strings are backed by smart pointer.
    /// It also copies the input into the output's buffer.
    ///
    /// # Errors
    ///
    /// For details see [`SharedStringError`].
    #[inline]
    pub fn from_str_slice_and_allocator(text: &str, allocator: TAllocator) -> Result<Self, SharedStringError> {
        let text_len = Length::try_from_usize(text.len())?;
        let mut builder = super::SharedStringBuilder::with_capacity_and_allocator(text_len, allocator)?;
        builder.try_push(text)?;
        let result = builder.build()?;
        Ok(result)
    }

    /// Creates a new, empty [`SharedString`] with a given allocator.
    ///
    /// This function allocates under the hood, since all shared strings are backed by smart pointer.
    ///
    /// # Errors
    ///
    /// For details see [`SharedStringError`].
    #[inline(always)]
    pub fn empty_with_allocator(allocator: TAllocator) -> Result<Self, SharedStringError> {
        Self::from_str_slice_and_allocator("", allocator)
    }

    #[inline(always)]
    #[must_use]
    pub fn strong_count(&self) -> u32 {
        CArcArray::strong_count(&self.internal)
    }

    #[inline(always)]
    #[must_use]
    pub fn weak_count(&self) -> u32 {
        CArcArray::weak_count(&self.internal)
    }

    /// Compares the underlying raw pointers of the two strings.
    ///
    /// Useful for determining whether the two strings have the same
    /// allocation, not only data.
    #[inline]
    #[must_use]
    pub fn ptr_eq(&self, other: &Self) -> bool {
        core::ptr::eq(CArcArray::data(&self.internal), CArcArray::data(&other.internal))
    }

    /// Returns the underlying string.
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        unsafe {
            let slice = self.internal.as_ref();
            core::str::from_utf8_unchecked(&slice[..slice.len() - 1])
        }
    }

    /// Returns the underlying string as C-string.
    ///
    /// Meaning the string has an additional 0 at the end of the buffer. In particular,
    /// the C-string returned by this method has length +1 compared to [`SharedString::as_str`]
    /// call.
    #[inline]
    #[must_use]
    pub fn as_c_str(&self) -> &str {
        unsafe {
            let slice = self.internal.as_ref();
            core::str::from_utf8_unchecked(slice)
        }
    }

    /// The length of the string.
    #[inline]
    pub fn length(&self) -> Length {
        unsafe {
            let slice = self.internal.as_ref();
            Length::new_unchecked((slice.len() - 1) as u32)
        }
    }

    /// Creates a new [`WeakSharedString`] out of current.
    ///
    /// # Errors
    ///
    /// Returns [`MaxReferencesExceededError`] if the weak reference count is too high.
    /// Cannot exceed [`osom_lib_arc::consts::MAX_REFERENCES`].
    #[inline]
    pub fn downgrade(&self) -> Result<WeakSharedString<TAllocator>, MaxReferencesExceededError> {
        let weak = CArcArray::downgrade(&self.internal)?;
        Ok(WeakSharedString::from_internal(weak))
    }

    /// Abandons current [`SharedString`].
    ///
    /// This function returns None if the underlying strong reference counter
    /// is still positive. Otherwise it returns the final [`WeakSharedString`]
    /// reference.
    #[inline]
    #[must_use]
    pub fn abandon(self) -> Option<WeakSharedString<TAllocator>> {
        let result = CArcArray::<u8, TAllocator>::abandon(self.internal)?;
        Some(WeakSharedString::from_internal(result))
    }
}

impl<TAllocator: Allocator> TryClone for SharedString<TAllocator> {
    type Error = MaxReferencesExceededError;

    #[inline]
    fn try_clone(&self) -> Result<Self, Self::Error> {
        let internal = self.internal.try_clone()?;
        Ok(Self { internal })
    }
}

impl<TAllocator: Allocator> Clone for SharedString<TAllocator> {
    #[inline]
    fn clone(&self) -> Self {
        self.try_clone()
            .expect("ImmutableString strong reference count is too high.")
    }
}

impl<TAllocator: Allocator> AsRef<str> for SharedString<TAllocator> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<TAllocator: Allocator> Borrow<str> for SharedString<TAllocator> {
    #[inline]
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl<TAllocator: Allocator> Hash for SharedString<TAllocator> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

impl<TAllocator: Allocator> PartialEq for SharedString<TAllocator> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.internal == other.internal
    }
}

impl<TAllocator: Allocator> Eq for SharedString<TAllocator> {}

impl<TAllocator: Allocator> Display for SharedString<TAllocator> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<TAllocator: Allocator> From<&str> for SharedString<TAllocator>
where
    TAllocator: Default,
{
    #[inline]
    fn from(value: &str) -> Self {
        Self::from_str_slice(value).expect("Failed to create SharedString from &str")
    }
}
