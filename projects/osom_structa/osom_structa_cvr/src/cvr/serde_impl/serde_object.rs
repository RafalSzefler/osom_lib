use osom_lib_alloc::traits::Allocator;
use osom_lib_primitives::length::Length;
use osom_lib_try_clone::TryClone;
use serde::{Serialize, de::{self, DeserializeSeed, Visitor}, ser::SerializeMap as _};

use crate::cvr::{CVRObject, serde::CVRDeserializeContext};

use super::make_seed_struct;
use super::serde_core::CVRSeed;
use super::serde_string::CVRStringSeed;

make_seed_struct!(CVRObjectSeed);

impl<TAllocator: Allocator> Serialize for CVRObject<TAllocator> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len().as_usize()))?;

        for (key, value) in self.iter() {
            map.serialize_entry(key, value)?;
        }

        map.end()
    }
}

struct CVRObjectVisitor<'a, TAllocator: Allocator + TryClone> {
    context: &'a mut CVRDeserializeContext<TAllocator>,
}

impl<'de, TAllocator: Allocator + TryClone> Visitor<'de> for CVRObjectVisitor<'_, TAllocator> {
    type Value = CVRObject<TAllocator>;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("a (string, CVR) mapping")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let capacity = map.size_hint().unwrap_or(0);
        let _ = Length::try_from_usize(capacity).map_err(de::Error::custom)?;
        let allocator = self.context.allocator().try_clone()
            .map_err(|_| de::Error::custom("Failed to clone allocator"))?;
        let mut result = Self::Value::with_allocator(allocator);
        loop {
            let cvr_string_seed = CVRStringSeed { context: self.context };
            let key_result = map.next_key_seed(cvr_string_seed)?;
            let Some(key) = key_result else {
                break;
            };
            let cvr_seed = CVRSeed { context: self.context };
            let value = map.next_value_seed(cvr_seed)?;
            result.try_insert(key, value).map_err(de::Error::custom)?;
        }
        Ok(result)
    }
}

impl<'de, TAllocator: Allocator + TryClone> DeserializeSeed<'de> for CVRObjectSeed<'_, TAllocator> {
    type Value = CVRObject<TAllocator>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(CVRObjectVisitor { context: self.context })
    }
}
