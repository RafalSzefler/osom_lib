use core::{hash::Hash, marker::PhantomData};

use osom_lib_alloc::traits::Allocator;
use osom_lib_primitives::length::Length;
use serde::{
    Deserialize, Serialize,
    de::{self, Visitor},
    ser::SerializeMap,
};

use crate::{
    defaults::DefaultHashTable,
    traits::{ImmutableHashTable, MutableHashTable},
};

impl<TKey, TValue, TAllocator> Serialize for DefaultHashTable<TKey, TValue, TAllocator>
where
    TKey: Eq + Hash + Serialize,
    TValue: Serialize,
    TAllocator: Allocator,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut m = serializer.serialize_map(Some(self.length().as_usize()))?;
        for kvp in self.iter() {
            m.serialize_entry(&kvp.key, &kvp.value)?;
        }
        m.end()
    }
}

struct DefaultHashTableVisitor<TKey, TValue, TAllocator> {
    _phantom: PhantomData<(TKey, TValue, TAllocator)>,
}

impl<TKey, TValue, TAllocator> DefaultHashTableVisitor<TKey, TValue, TAllocator> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self { _phantom: PhantomData }
    }
}

impl<'de, TKey, TValue, TAllocator> Visitor<'de> for DefaultHashTableVisitor<TKey, TValue, TAllocator>
where
    TKey: Eq + Hash + Deserialize<'de>,
    TValue: Deserialize<'de>,
    TAllocator: Allocator + Default,
{
    type Value = DefaultHashTable<TKey, TValue, TAllocator>;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("a (key, value) mapping")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let capacity = map.size_hint().unwrap_or(0);
        let length = Length::try_from_usize(capacity).map_err(de::Error::custom)?;
        let mut result =
            DefaultHashTable::<TKey, TValue, TAllocator>::with_capacity(length).map_err(de::Error::custom)?;
        while let Some((key, value)) = map.next_entry::<TKey, TValue>()? {
            result.try_insert(key, value).map_err(de::Error::custom)?;
        }
        Ok(result)
    }
}

impl<'de, TKey, TValue, TAllocator> Deserialize<'de> for DefaultHashTable<TKey, TValue, TAllocator>
where
    TKey: Eq + Hash + Deserialize<'de>,
    TValue: Deserialize<'de>,
    TAllocator: Allocator + Default,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(DefaultHashTableVisitor::new())
    }
}
