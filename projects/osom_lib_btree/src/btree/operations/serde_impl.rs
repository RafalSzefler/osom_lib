use core::marker::PhantomData;

use osom_lib_primitives::length::Length;
use serde::de::Visitor;
use serde::ser::SerializeMap as _;
use serde::{Deserialize, Serialize, de};

use crate::btree::{BTree, BTreeConfig};

impl<TKey, TValue, TConfig> Serialize for BTree<TKey, TValue, TConfig>
where
    TKey: Ord + Serialize,
    TValue: Serialize,
    TConfig: BTreeConfig,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len().as_usize()))?;
        for kvp in self.iter() {
            map.serialize_entry(&kvp.key, &kvp.value)?;
        }
        map.end()
    }
}

struct BTreeVisitor<TKey, TValue, TConfig> {
    _phantom: PhantomData<(TKey, TValue, TConfig)>,
}

impl<TKey, TValue, TConfig> BTreeVisitor<TKey, TValue, TConfig>
where
    TKey: Ord,
    TConfig: BTreeConfig,
{
    #[inline(always)]
    pub const fn new() -> Self {
        Self { _phantom: PhantomData }
    }
}

impl<'de, TKey, TValue, TConfig> Visitor<'de> for BTreeVisitor<TKey, TValue, TConfig>
where
    TKey: Ord + Deserialize<'de>,
    TValue: Deserialize<'de>,
    TConfig: BTreeConfig + Default,
{
    type Value = BTree<TKey, TValue, TConfig>;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("a (key, value) mapping")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let capacity = map.size_hint().unwrap_or(0);
        let _ = Length::try_from_usize(capacity).map_err(de::Error::custom)?;

        let mut result = BTree::<TKey, TValue, TConfig>::new();

        while let Some((key, value)) = map.next_entry::<TKey, TValue>()? {
            result.try_insert(key, value).map_err(de::Error::custom)?;
        }
        Ok(result)
    }
}

impl<'de, TKey, TValue, TConfig> Deserialize<'de> for BTree<TKey, TValue, TConfig>
where
    TKey: Ord + Deserialize<'de>,
    TValue: Deserialize<'de>,
    TConfig: BTreeConfig + Default,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(BTreeVisitor::new())
    }
}
