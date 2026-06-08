use osom_lib_alloc::traits::Allocator;
use osom_lib_hash_tables::traits::{
    ImmutableHashTable as _,
    MutableHashTable as _,
};
use osom_lib_strings::immutable::ImmutableString;
use osom_lib_try_clone::TryClone;
use serde::Serialize;
use serde::de::{DeserializeSeed, Error, Visitor};

use crate::cvr::CVRString;

use super::make_seed_struct;
use super::context::CVRDeserializeContext;

make_seed_struct!(CVRStringSeed);

impl<TAllocator: Allocator> Serialize for CVRString<TAllocator> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_str().serialize(serializer)
    }
}

struct CVRStringVisitor<'a, TAllocator: Allocator + TryClone> {
    context: &'a mut CVRDeserializeContext<TAllocator>,
}

impl<TAllocator: Allocator + TryClone> CVRStringVisitor<'_, TAllocator> {
    fn build_cvr_string(&mut self, value: &str) -> Result<CVRString<TAllocator>, &'static str> {
        if let Some(cvr_string) = self.context.string_cache_mut()
            .get_key_value(value)
        {
            let clone = cvr_string.key.try_clone().map_err(|_| "Failed to clone ImmutableString")?;
            let cvr_string = CVRString::from(clone);
            return Ok(cvr_string);
        }

        let allocator = self.context.allocator().try_clone()
            .map_err(|_| "Failed to clone allocator")?;
        let immutable_string = ImmutableString::from_str_slice_and_allocator(value, allocator)
            .map_err(|_| "Failed to create ImmutableString")?;
        let cvr_string = CVRString::from(immutable_string.clone());
        self.context.string_cache_mut().try_insert(immutable_string, ())
            .map_err(|_| "Failed to insert CVRString into cache")?;
        Ok(cvr_string)
    }
}

impl<'de, TAllocator: Allocator + TryClone> Visitor<'de> for CVRStringVisitor<'_, TAllocator> {
    type Value = CVRString<TAllocator>;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E>(mut self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.build_cvr_string(v).map_err(E::custom)
    }

    fn visit_borrowed_str<E>(mut self, v: &'de str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.build_cvr_string(v).map_err(E::custom)
    }

    #[cfg(feature = "std")]
    fn visit_string<E>(mut self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.build_cvr_string(&v).map_err(E::custom)
    }
}

impl<'de, TAllocator: Allocator + TryClone> DeserializeSeed<'de> for CVRStringSeed<'_, TAllocator> {
    type Value = CVRString<TAllocator>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(CVRStringVisitor { context: self.context })
    }
}
