#![cfg(feature = "std")]
#![allow(clippy::default_constructed_unit_structs)]
use osom_lib_alloc::{std_allocator::StdAllocator, traits::Allocator};
use osom_lib_hash_tables::{defaults::DefaultHashTable, traits::{ImmutableHashTable as _, MutableHashTable as _}};
use osom_lib_try_clone::TryClone;

use crate::shared::{SharedString, serde::StringCache};

/// The default string cache backed by a hash set.
/// 
/// Note: this struct is not `repr(C)`.
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[must_use]
pub struct StdStringCache<TAllocator: Allocator + TryClone> {
    allocator: TAllocator,
    cache: DefaultHashTable<SharedString<TAllocator>, (), TAllocator>,
}

impl<TAllocator: Allocator + TryClone> StdStringCache<TAllocator> {
    /// Creates a new [`StdStringCache`] with the given allocator.
    #[inline]
    pub fn with_allocator(allocator: TAllocator) -> Self
    where TAllocator: Clone,
    {
        let allocator_clone = allocator.clone();
        Self {
            allocator,
            cache: DefaultHashTable::with_allocator(allocator_clone),
        }
    }
}

impl Default for StdStringCache<StdAllocator> {
    fn default() -> Self {
        Self::with_allocator(StdAllocator::default())
    }
}

impl<TAllocator: Allocator + TryClone> StringCache for StdStringCache<TAllocator> {
    type TAllocator = TAllocator;

    fn get_and_cache(&mut self, value: &str) -> Result<SharedString<Self::TAllocator>, super::CacheError> {
        if let Some(imm) = self.cache.get_key_value(value) {
            let clone = imm.key.try_clone().map_err(|_| super::CacheError::new("Failed to clone ImmutableString"))?;
            return Ok(clone);
        }

        let allocator_clone = self.allocator.try_clone().map_err(|_| super::CacheError::new("Failed to clone Allocator"))?;
        let immutable_string = SharedString::from_str_slice_and_allocator(value, allocator_clone)
            .map_err(|_| super::CacheError::new("Failed to create ImmutableString"))?;
        self.cache.insert(immutable_string.clone(), ());
        Ok(immutable_string)
    }
}
