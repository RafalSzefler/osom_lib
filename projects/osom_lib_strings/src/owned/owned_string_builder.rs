use osom_lib_alloc::traits::Allocator;
use osom_lib_arrays::{
    dynamic_array::InlineDynamicArray,
    traits::{ImmutableArray, MutableArray},
};
use osom_lib_reprc::macros::reprc;

use super::{InnerDynamicArray, OwnedString, OwnedStringError};

/// Represents a string builder. This struct allows for building [`OwnedString`] instances,
/// which are immutable.
///
/// # Examples
///
/// ```rust
/// # cfg_select! {
/// #    feature="std" => {
/// use osom_lib_strings::std::StdOwnedStringBuilder;
///
/// let mut builder = StdOwnedStringBuilder::new();
/// builder.push_str("Hello, ");
/// builder.push_str("World!");
/// let string = builder.build();
/// assert_eq!(string.as_ref(), "Hello, World!");
/// # },
/// # _ => {}
/// # }
/// ```
#[reprc]
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq)]
#[must_use]
pub struct OwnedStringBuilder<TAllocator: Allocator> {
    data: InnerDynamicArray<TAllocator>,
}

impl<TAllocator: Allocator> OwnedStringBuilder<TAllocator> {
    /// Creates a new [`OwnedStringBuilder`] with the given allocator.
    #[inline]
    pub fn with_allocator(allocator: TAllocator) -> Self {
        Self::from_inner(InlineDynamicArray::with_allocator(allocator))
    }

    /// Appends a string slice to the builder.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying allocator fails to allocate memory.
    #[inline]
    pub fn try_push_str(&mut self, s: &str) -> Result<(), OwnedStringError> {
        self.data.try_push_slice(s.as_bytes()).map_err(Into::into)
    }

    /// Appends a string slice to the builder.
    ///
    /// # Panics
    ///
    /// Panics if the underlying allocator fails to allocate memory.
    #[inline]
    pub fn push_str(&mut self, s: &str) {
        self.try_push_str(s)
            .expect("Failed to push string to OwnedStringBuilder");
    }

    /// Builds a new [`OwnedString`] from the builder.
    #[inline(always)]
    pub fn build(self) -> OwnedString<TAllocator> {
        OwnedString::from_inner(self.data)
    }

    /// Returns a reference to the underlying immutable array.
    #[inline(always)]
    pub fn internal_array(&self) -> &impl ImmutableArray<u8> {
        &self.data
    }

    /// Returns a mutable reference to the underlying mutable array.
    ///
    /// # Safety
    ///
    /// Since this method returns a mutable reference, it is up to the caller to
    /// ensure that the array remains a valid utf-8 string. Otherwise, the behavior is undefined.
    #[inline(always)]
    pub unsafe fn internal_array_mut(&mut self) -> &mut impl MutableArray<u8> {
        &mut self.data
    }

    #[inline(always)]
    pub(super) fn from_inner(data: InnerDynamicArray<TAllocator>) -> Self {
        Self { data }
    }
}

impl<TAllocator: Allocator + Default> OwnedStringBuilder<TAllocator> {
    /// Creates a new [`OwnedStringBuilder`] with the default allocator.
    #[inline]
    pub fn new() -> Self {
        Self::with_allocator(TAllocator::default())
    }
}

impl<TAllocator: Allocator + Default> Default for OwnedStringBuilder<TAllocator> {
    fn default() -> Self {
        Self::new()
    }
}

impl<TAllocator: Allocator> AsRef<str> for OwnedStringBuilder<TAllocator> {
    fn as_ref(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(self.data.as_ref()) }
    }
}

impl<TAllocator: Allocator> AsMut<str> for OwnedStringBuilder<TAllocator> {
    fn as_mut(&mut self) -> &mut str {
        unsafe { core::str::from_utf8_unchecked_mut(self.data.as_mut()) }
    }
}
