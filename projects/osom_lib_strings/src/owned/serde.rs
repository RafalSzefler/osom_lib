use std::marker::PhantomData;

use osom_lib_alloc::traits::Allocator;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Visitor};

use super::OwnedString;

struct OwnedStringVisitor<TAllocator: Allocator> {
    _phantom: PhantomData<TAllocator>,
}

impl<'de, TAllocator: Allocator + Default> Visitor<'de> for OwnedStringVisitor<TAllocator> {
    type Value = OwnedString<TAllocator>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        OwnedString::try_from_str(v).map_err(E::custom)
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        OwnedString::try_from_str(&v).map_err(E::custom)
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        OwnedString::try_from_str(v).map_err(E::custom)
    }
}

impl<'de, TAllocator: Allocator + Default> Deserialize<'de> for OwnedString<TAllocator> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(OwnedStringVisitor { _phantom: PhantomData })
    }
}

impl<TAllocator: Allocator> Serialize for OwnedString<TAllocator> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_ref())
    }
}
