use core::marker::PhantomData;

use serde::{
    Deserialize, Serialize,
    de::{self, Visitor},
};

use osom_lib_alloc::traits::Allocator;
use osom_lib_primitives::length::Length;

use crate::dynamic_array::DynamicArray;
use crate::traits::MutableArray;

impl<T: Serialize, TAllocator: Allocator> Serialize for DynamicArray<T, TAllocator> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}

struct DynamicArrayVisitor<T, TAllocator> {
    _phantom: PhantomData<(T, TAllocator)>,
}

impl<T, TAllocator> DynamicArrayVisitor<T, TAllocator> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self { _phantom: PhantomData }
    }
}

impl<'de, T: Deserialize<'de>, TAllocator: Allocator + Default> Visitor<'de> for DynamicArrayVisitor<T, TAllocator> {
    type Value = DynamicArray<T, TAllocator>;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("a sequence of deserializable values")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let capacity = seq.size_hint().unwrap_or(0);
        let capacity = Length::try_from_usize(capacity).map_err(de::Error::custom)?;
        let mut result = Self::Value::with_capacity(capacity).map_err(de::Error::custom)?;
        while let Some(item) = seq.next_element()? {
            result.try_push(item).map_err(de::Error::custom)?;
        }

        Ok(result)
    }
}

impl<'de, T: Deserialize<'de>, TAllocator: Allocator + Default> Deserialize<'de> for DynamicArray<T, TAllocator> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(DynamicArrayVisitor::new())
    }
}
