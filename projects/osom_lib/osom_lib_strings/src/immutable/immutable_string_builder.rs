use osom_lib_alloc::traits::Allocator;
use osom_lib_arc::carc_array::CArcArrayBuilder;
use osom_lib_primitives::length::Length;
use osom_lib_reprc::macros::reprc;

use crate::immutable::{ImmutableString, ImmutableStringError};

/// The builder for [`ImmutableString`].
///
/// It allows direct writing to the underlying buffer, before the data
/// gets sealed as immutable.
///
/// # Examples
///
/// ```rust
/// # cfg_select! {
/// #    feature="std" => {
/// use osom_lib_strings::immutable::ImmutableStringError;
/// use osom_lib_strings::immutable::std::StdImmutableStringBuilder;
///
/// fn main() -> Result<(), ImmutableStringError> {
///     let mut builder = StdImmutableStringBuilder::new()?;
///     builder.try_push("Name: ")?;
///     builder.try_push("John")?;
///     let string = builder.build()?;
///     assert_eq!(string.as_str(), "Name: John");
///     Ok(())
/// }
/// # },
/// # _ => {}
/// # }
/// ```
#[reprc]
#[repr(transparent)]
#[must_use]
pub struct ImmutableStringBuilder<TAllocator: Allocator> {
    internal: CArcArrayBuilder<u8, TAllocator>,
}

impl<TAllocator: Allocator> ImmutableStringBuilder<TAllocator> {
    /// Creates a new [`ImmutableStringBuilder`]. This allocates an initial buffer
    /// under the hood.
    ///
    /// # Errors
    ///
    /// For details see [`ImmutableStringError`].
    #[inline(always)]
    pub fn new() -> Result<Self, ImmutableStringError>
    where
        TAllocator: Default,
    {
        Self::with_capacity_and_allocator(Length::ZERO, TAllocator::default())
    }

    /// Creates a new [`ImmutableStringBuilder`] with given capacity. This allocates an initial buffer
    /// under the hood.
    ///
    /// # Errors
    ///
    /// For details see [`ImmutableStringError`].
    #[inline(always)]
    pub fn with_capacity(capacity: Length) -> Result<Self, ImmutableStringError>
    where
        TAllocator: Default,
    {
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
        let internal = CArcArrayBuilder::with_capacity_and_allocator(capacity + 1, allocator)?;
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
        self.internal
            .try_push_slice(text.as_bytes())
            .map_err(ImmutableStringError::from)
    }

    /// Shrinks the underlying buffer to match the length of the string exactly.
    /// This will likely reallocate the underlying buffer.
    ///
    /// # Errors
    ///
    /// For details see [`ImmutableStringError`].
    pub fn shrink_to_fit(&mut self) -> Result<(), ImmutableStringError> {
        self.internal.shrink_to_fit().map_err(ImmutableStringError::from)
    }

    /// Builds a new instance of [`ImmutableString`] out of the builder.
    ///
    /// # Errors
    ///
    /// For details see [`ImmutableStringError`].
    pub fn build(self) -> Result<ImmutableString<TAllocator>, ImmutableStringError> {
        let mut internal = unsafe { (&raw const self.internal).read() };
        core::mem::forget(self);
        internal.try_push_slice(b"\0")?;
        internal.shrink_to_fit()?;
        let result = ImmutableString::from_internal(internal.build());
        Ok(result)
    }
}
