use core::marker::PhantomData;

use serde::{
    Deserialize, Serialize,
    de::{self, Visitor},
};

use osom_lib_alloc::traits::Allocator;
use osom_lib_primitives::length::Length;

use crate::dynamic_array::InlineDynamicArray;
use crate::traits::MutableArray;

impl<const CAPACITY: usize, T: Serialize, TAllocator: Allocator> Serialize
    for InlineDynamicArray<CAPACITY, T, TAllocator>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}

struct InlineDynamicArrayVisitor<const CAPACITY: usize, T, TAllocator> {
    _phantom: PhantomData<(T, TAllocator)>,
}

impl<const CAPACITY: usize, T, TAllocator> InlineDynamicArrayVisitor<CAPACITY, T, TAllocator> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self { _phantom: PhantomData }
    }
}

impl<'de, const CAPACITY: usize, T: Deserialize<'de>, TAllocator: Allocator + Default> Visitor<'de>
    for InlineDynamicArrayVisitor<CAPACITY, T, TAllocator>
{
    type Value = InlineDynamicArray<CAPACITY, T, TAllocator>;

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

impl<'de, const CAPACITY: usize, T: Deserialize<'de>, TAllocator: Allocator + Default> Deserialize<'de>
    for InlineDynamicArray<CAPACITY, T, TAllocator>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(InlineDynamicArrayVisitor::new())
    }
}
