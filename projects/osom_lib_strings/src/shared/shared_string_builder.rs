use osom_lib_alloc::traits::Allocator;
use osom_lib_arc::carc_array::CArcArrayBuilder;
use osom_lib_primitives::length::Length;
use osom_lib_reprc::macros::reprc;

use super::{SharedString, SharedStringError};

/// The builder for [`SharedString`].
///
/// It allows direct writing to the underlying buffer, before the data
/// gets sealed as shared and immutable.
///
/// # Examples
///
/// ```rust
/// # cfg_select! {
/// #    feature="std" => {
/// use osom_lib_strings::shared::SharedStringError;
/// use osom_lib_strings::shared::std::StdSharedStringBuilder;
///
/// fn main() -> Result<(), SharedStringError> {
///     let mut builder = StdSharedStringBuilder::new()?;
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
pub struct SharedStringBuilder<TAllocator: Allocator> {
    internal: CArcArrayBuilder<u8, TAllocator>,
}

impl<TAllocator: Allocator> SharedStringBuilder<TAllocator> {
    /// Creates a new [`SharedStringBuilder`]. This allocates an initial buffer
    /// under the hood.
    ///
    /// # Errors
    ///
    /// For details see [`SharedStringError`].
    #[inline]
    pub fn new() -> Result<Self, SharedStringError>
    where
        TAllocator: Default,
    {
        Self::with_capacity_and_allocator(Length::ZERO, TAllocator::default())
    }

    /// Creates a new [`SharedStringBuilder`] with given capacity. This allocates an initial buffer
    /// under the hood.
    ///
    /// # Errors
    ///
    /// For details see [`SharedStringError`].
    #[inline]
    pub fn with_capacity(capacity: Length) -> Result<Self, SharedStringError>
    where
        TAllocator: Default,
    {
        Self::with_capacity_and_allocator(capacity, TAllocator::default())
    }

    /// Creates a new [`SharedStringBuilder`] with given allocator. This allocates an initial buffer
    /// under the hood.
    ///
    /// # Errors
    ///
    /// For details see [`SharedStringError`].
    #[inline]
    pub fn with_allocator(allocator: TAllocator) -> Result<Self, SharedStringError> {
        Self::with_capacity_and_allocator(Length::ZERO, allocator)
    }

    /// Creates a new [`SharedStringBuilder`] with given capacity and allocator.
    /// This allocates an initial buffer under the hood.
    ///
    /// # Errors
    ///
    /// For details see [`SharedStringError`].
    #[inline]
    pub fn with_capacity_and_allocator(capacity: Length, allocator: TAllocator) -> Result<Self, SharedStringError> {
        // NOTE: we allocate capacity+1 to make room for the last \0 byte, for C-string compatibility.
        let internal = CArcArrayBuilder::with_capacity_and_allocator(capacity + 1, allocator)?;
        Ok(Self { internal })
    }

    /// Pushes a new string slice to [`SharedStringBuilder`]. This copies the passed string to the underlying
    /// buffer. It will also resize the underlying buffer on demand.
    ///
    /// # Errors
    ///
    /// For details see [`SharedStringError`].
    #[inline]
    pub fn try_push(&mut self, text: &str) -> Result<(), SharedStringError> {
        self.internal
            .try_push_slice(text.as_bytes())
            .map_err(SharedStringError::from)
    }

    /// Shrinks the underlying buffer to match the length of the string exactly.
    /// This will likely reallocate the underlying buffer.
    ///
    /// # Errors
    ///
    /// For details see [`SharedStringError`].
    pub fn shrink_to_fit(&mut self) -> Result<(), SharedStringError> {
        self.internal.shrink_to_fit().map_err(SharedStringError::from)
    }

    /// Builds a new instance of [`SharedString`] out of the builder.
    ///
    /// # Errors
    ///
    /// For details see [`SharedStringError`].
    pub fn build(self) -> Result<SharedString<TAllocator>, SharedStringError> {
        let mut internal = unsafe { (&raw const self.internal).read() };
        core::mem::forget(self);
        internal.try_push_slice(b"\0")?;
        internal.shrink_to_fit()?;
        let result = SharedString::from_internal(internal.build());
        Ok(result)
    }
}
