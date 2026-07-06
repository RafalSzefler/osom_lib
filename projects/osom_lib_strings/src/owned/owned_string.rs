use core::borrow::Borrow;
use core::hash::Hash;

use osom_lib_alloc::traits::Allocator;
use osom_lib_arrays::dynamic_array::InlineDynamicArray;
use osom_lib_reprc::macros::reprc;
use osom_lib_try_clone::TryClone;

use crate::owned::OwnedStringBuilder;

use super::errors::OwnedStringError;

pub(super) type InnerDynamicArray<TAllocator> = InlineDynamicArray<8, u8, TAllocator>;

/// Represents a string owner. This struct is similar to the `std::string::String` type,
/// with the following differences:
/// - This struct is `repr(C)`, and can be used across FFI boundaries.
/// - This struct is immutable. Extending it is possible, though, by converting it to a [`OwnedStringBuilder`] first.
/// - This struct uses short string optimization. All strings up to 8 bytes are stored inline.
/// - This struct allows for a custom allocator.
#[reprc]
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq)]
#[must_use]
pub struct OwnedString<TAllocator: Allocator> {
    data: InnerDynamicArray<TAllocator>,
}

impl<TAllocator: Allocator> OwnedString<TAllocator> {
    #[inline(always)]
    pub(super) fn from_inner(data: InnerDynamicArray<TAllocator>) -> Self {
        Self { data }
    }
}

impl<TAllocator: Allocator + Default> OwnedString<TAllocator> {
    /// Creates a new [`OwnedString`] from a string slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying allocator fails to allocate memory.
    #[inline]
    pub fn try_from_str(text: &str) -> Result<Self, OwnedStringError> {
        let mut builder = OwnedStringBuilder::new();
        builder.try_push_str(text)?;
        Ok(builder.build())
    }
}

impl<TAllocator: Allocator + TryClone> TryClone for OwnedString<TAllocator> {
    type Error = OwnedStringError;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        let inner = self.data.try_clone().map_err(|_| OwnedStringError::AllocationError)?;
        Ok(Self { data: inner })
    }
}

impl<TAllocator: Allocator + TryClone> Clone for OwnedString<TAllocator> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl<TAllocator: Allocator> From<OwnedStringBuilder<TAllocator>> for OwnedString<TAllocator> {
    fn from(builder: OwnedStringBuilder<TAllocator>) -> Self {
        builder.build()
    }
}

impl<TAllocator: Allocator> From<OwnedString<TAllocator>> for OwnedStringBuilder<TAllocator> {
    fn from(string: OwnedString<TAllocator>) -> Self {
        OwnedStringBuilder::from_inner(string.data)
    }
}

impl<TAllocator: Allocator> AsRef<str> for OwnedString<TAllocator> {
    fn as_ref(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(self.data.as_ref()) }
    }
}

impl<TAllocator: Allocator> Borrow<str> for OwnedString<TAllocator> {
    fn borrow(&self) -> &str {
        self.as_ref()
    }
}

impl<TAllocator: Allocator> Hash for OwnedString<TAllocator> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}

impl<TAllocator: Allocator + Default> From<&str> for OwnedString<TAllocator> {
    fn from(text: &str) -> Self {
        Self::try_from_str(text).unwrap()
    }
}
