#![cfg(feature = "std")]
#![allow(clippy::default_constructed_unit_structs)]
use std::collections::HashSet;

use osom_lib_alloc::{std_allocator::StdAllocator, traits::Allocator};
use osom_lib_try_clone::TryClone;

use crate::immutable::{ImmutableString, serde::StringCache};

/// The default string cache backed by the standard [`HashSet`].
/// 
/// Note: this struct is not `repr(C)`.
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[must_use]
pub struct StdStringCache<TAllocator: Allocator + TryClone> {
    allocator: TAllocator,
    cache: HashSet<ImmutableString<TAllocator>>,
}

impl<TAllocator: Allocator + TryClone> StdStringCache<TAllocator> {
    /// Creates a new [`StdStringCache`] with the given allocator.
    #[inline]
    pub fn with_allocator(allocator: TAllocator) -> Self {
        Self {
            allocator,
            cache: HashSet::new(),
        }
    }
}

impl StdStringCache<StdAllocator> {
    /// Creates a new [`StdStringCache`] with the default [`StdAllocator`].
    #[inline(always)]
    pub fn new() -> Self {
        Self::with_allocator(StdAllocator::default())
    }
}

impl Default for StdStringCache<StdAllocator> {
    fn default() -> Self {
        Self::new()
    }
}

impl<TAllocator: Allocator + TryClone> StringCache for StdStringCache<TAllocator> {
    type TAllocator = TAllocator;

    fn get_and_cache(&mut self, value: &str) -> Result<ImmutableString<Self::TAllocator>, super::CacheError> {
        if let Some(imm) = self.cache.get(value) {
            let clone = imm.try_clone().map_err(|_| super::CacheError::new("Failed to clone ImmutableString"))?;
            return Ok(clone);
        }

        let allocator_clone = self.allocator.try_clone().map_err(|_| super::CacheError::new("Failed to clone Allocator"))?;
        let immutable_string = ImmutableString::from_str_slice_and_allocator(value, allocator_clone)
            .map_err(|_| super::CacheError::new("Failed to create ImmutableString"))?;
        self.cache.insert(immutable_string.clone());
        Ok(immutable_string)
    }
}
