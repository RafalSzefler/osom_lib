use core::marker::PhantomData;

use serde::{
    Deserialize, Serialize,
    de::{self, Visitor},
};

use osom_lib_alloc::traits::Allocator;
use osom_lib_primitives::length::Length;

use crate::dynamic_array::AlignedDynamicArray;
use crate::traits::MutableArray;

impl<TAlign, TItem: Serialize, TAllocator: Allocator> Serialize for AlignedDynamicArray<TAlign, TItem, TAllocator> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}

struct AlignedDynamicArrayVisitor<TAlign, TItem, TAllocator> {
    _phantom: PhantomData<(TAlign, TItem, TAllocator)>,
}

impl<TAlign, TItem, TAllocator> AlignedDynamicArrayVisitor<TAlign, TItem, TAllocator> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self { _phantom: PhantomData }
    }
}

impl<'de, TAlign, TItem: Deserialize<'de>, TAllocator: Allocator + Default> Visitor<'de>
    for AlignedDynamicArrayVisitor<TAlign, TItem, TAllocator>
{
    type Value = AlignedDynamicArray<TAlign, TItem, TAllocator>;

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

impl<'de, TAlign, TItem: Deserialize<'de>, TAllocator: Allocator + Default> Deserialize<'de>
    for AlignedDynamicArray<TAlign, TItem, TAllocator>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(AlignedDynamicArrayVisitor::new())
    }
}
