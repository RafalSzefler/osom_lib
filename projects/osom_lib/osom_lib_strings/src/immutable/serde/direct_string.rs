use core::marker::PhantomData;

use osom_lib_alloc::traits::Allocator;
use serde::{
    Deserialize, Serialize,
    de::{self, Visitor},
};

use crate::immutable::ImmutableString;

impl<TAllocator: Allocator> Serialize for ImmutableString<TAllocator> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_str().serialize(serializer)
    }
}

struct ImmutableStringVisitor<TAllocator> {
    _phantom: PhantomData<TAllocator>,
}

impl<TAllocator> ImmutableStringVisitor<TAllocator> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self { _phantom: PhantomData }
    }
}

impl<TAllocator: Allocator + Default> Visitor<'_> for ImmutableStringVisitor<TAllocator> {
    type Value = ImmutableString<TAllocator>;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("a string")
    }

    #[cfg(feature = "std")]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let imm = ImmutableString::from_str_slice(&v).map_err(de::Error::custom)?;
        Ok(imm)
    }

    fn visit_borrowed_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let imm = ImmutableString::from_str_slice(v).map_err(de::Error::custom)?;
        Ok(imm)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let imm = ImmutableString::from_str_slice(v).map_err(de::Error::custom)?;
        Ok(imm)
    }
}

impl<'de, TAllocator: Allocator + Default> Deserialize<'de> for ImmutableString<TAllocator> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(ImmutableStringVisitor::new())
    }
}
