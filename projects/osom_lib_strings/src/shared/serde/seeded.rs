#![allow(clippy::elidable_lifetime_names)]

use osom_lib_alloc::traits::Allocator;
use serde::de;

use crate::shared::SharedString;

/// Represents a generic cache error.
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Hash)]
#[must_use]
pub struct CacheError {
    message: &'static str,
}

impl CacheError {
    #[inline(always)]
    pub const fn new(message: &'static str) -> Self {
        Self { message }
    }
}

impl core::fmt::Display for CacheError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "CacheError: {}", self.message)
    }
}

impl AsRef<str> for CacheError {
    fn as_ref(&self) -> &str {
        self.message
    }
}

/// Represents a trait for a cache of immutable strings.
#[must_use]
pub trait StringCache {
    type TAllocator: Allocator;

    /// Gets the immutable string from the cache and caches it if it is not present.
    ///
    /// # Errors
    ///
    /// For details see [`CacheError`].
    fn get_and_cache(&mut self, value: &str) -> Result<SharedString<Self::TAllocator>, CacheError>;
}

/// The main seed for deserializing cached immutable strings.
#[must_use]
pub struct CachedStringSeed<'a, TStringCache: StringCache> {
    cache: &'a mut TStringCache,
}

impl<'a, TStringCache: StringCache> CachedStringSeed<'a, TStringCache> {
    pub fn new(cache: &'a mut TStringCache) -> Self {
        Self { cache }
    }
}

impl<'a, TStringCache: StringCache> de::Visitor<'_> for CachedStringSeed<'a, TStringCache> {
    type Value = SharedString<TStringCache::TAllocator>;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let value = self.cache.get_and_cache(v).map_err(E::custom)?;
        Ok(value)
    }

    #[cfg(feature = "std")]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let value = self.cache.get_and_cache(&v).map_err(E::custom)?;
        Ok(value)
    }

    fn visit_borrowed_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let value = self.cache.get_and_cache(v).map_err(E::custom)?;
        Ok(value)
    }
}

impl<'de, 'a, TStringCache: StringCache> de::DeserializeSeed<'de> for CachedStringSeed<'a, TStringCache> {
    type Value = SharedString<TStringCache::TAllocator>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let visitor = CachedStringSeed::new(self.cache);
        let result = deserializer.deserialize_string(visitor)?;
        Ok(result)
    }
}
