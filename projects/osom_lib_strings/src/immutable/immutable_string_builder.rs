use osom_lib_alloc::traits::Allocator;
use osom_lib_primitives::length::Length;
use osom_lib_reprc::macros::reprc;

use crate::immutable::{ImmutableString, ImmutableStringError, internal_string::InternalString};

#[reprc]
#[repr(transparent)]
pub struct ImmutableStringBuilder<TAllocator: Allocator> {
    internal: InternalString<TAllocator>,
}

impl<TAllocator: Allocator> ImmutableStringBuilder<TAllocator> {
    /// Creates a new [`ImmutableStringBuilder`]. This allocates an initial buffer
    /// under the hood.
    ///
    /// # Errors
    ///
    /// For details see [`ImmutableStringError`].
    #[inline(always)]
    pub fn new() -> Result<Self, ImmutableStringError> {
        Self::with_capacity_and_allocator(Length::ZERO, TAllocator::default())
    }

    /// Creates a new [`ImmutableStringBuilder`] with given capacity. This allocates an initial buffer
    /// under the hood.
    ///
    /// # Errors
    ///
    /// For details see [`ImmutableStringError`].
    #[inline(always)]
    pub fn with_capacity(capacity: Length) -> Result<Self, ImmutableStringError> {
        Self::with_capacity_and_allocator(capacity, TAllocator::default())
    }

    /// Creates a new [`ImmutableStringBuilder`] with given allocator. This allocates an initial buffer
    /// under the hood.
    ///
    /// # Errors
    ///
    /// For details see [`ImmutableStringError`].
    #[inline(always)]
    pub fn with_allocator(allocator: TAllocator) -> Result<Self, ImmutableStringError> {
        Self::with_capacity_and_allocator(Length::ZERO, allocator)
    }

    /// Creates a new [`ImmutableStringBuilder`] with given capacity and allocator.
    /// This allocates an initial buffer under the hood.
    ///
    /// # Errors
    ///
    /// For details see [`ImmutableStringError`].
    #[inline]
    pub fn with_capacity_and_allocator(capacity: Length, allocator: TAllocator) -> Result<Self, ImmutableStringError> {
        // NOTE: we allocate capacity+1 to make room for the last \0 byte, for C-string compatibility.
        let internal = InternalString::with_allocator_and_capacity(capacity + 1, allocator)?;
        Ok(Self { internal })
    }

    /// Pushes a new string slice to [`ImmutableStringBuilder`]. This copies the passed string to the underlying
    /// buffer. It will also resize the underlying buffer on demand.
    ///
    /// # Errors
    ///
    /// For details see [`ImmutableStringError`].
    #[inline]
    pub fn try_push(&mut self, text: &str) -> Result<(), ImmutableStringError> {
        self.internal.try_push_slice(text.as_bytes())
    }

    /// Shrinks the underlying buffer to match the length of the string exactly.
    /// This will likely reallocate the underlying buffer.
    ///
    /// # Errors
    ///
    /// For details see [`ImmutableStringError`].
    pub fn shrink_to_fit(&mut self) -> Result<(), ImmutableStringError> {
        self.internal.shrink_to_fit()
    }

    /// Builds a new instance of [`ImmutableString`] out of the builder.
    ///
    /// # Errors
    ///
    /// For details see [`ImmutableStringError`].
    pub fn build(self) -> Result<ImmutableString<TAllocator>, ImmutableStringError> {
        let mut internal = self.internal;
        internal.try_push_slice(b"\0")?;
        let result = ImmutableString::from_internal(internal);
        Ok(result)
    }
}
