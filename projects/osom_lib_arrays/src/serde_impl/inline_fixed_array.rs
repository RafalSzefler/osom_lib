use core::marker::PhantomData;

use serde::{
    Deserialize, Serialize,
    de::{self, Visitor},
};

use crate::fixed_array::InlineFixedArray;
use crate::traits::MutableArray;

impl<const CAPACITY: usize, T: Serialize> Serialize for InlineFixedArray<CAPACITY, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}

struct InlineFixedArrayVisitor<const CAPACITY: usize, T> {
    _phantom: PhantomData<T>,
}

impl<const CAPACITY: usize, T> InlineFixedArrayVisitor<CAPACITY, T> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self { _phantom: PhantomData }
    }
}

impl<'de, const CAPACITY: usize, T: Deserialize<'de>> Visitor<'de> for InlineFixedArrayVisitor<CAPACITY, T> {
    type Value = InlineFixedArray<CAPACITY, T>;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("a sequence of deserializable values")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut result = Self::Value::new();
        while let Some(item) = seq.next_element()? {
            result.try_push(item).map_err(de::Error::custom)?;
        }

        Ok(result)
    }
}

impl<'de, const CAPACITY: usize, T: Deserialize<'de>> Deserialize<'de> for InlineFixedArray<CAPACITY, T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(InlineFixedArrayVisitor::new())
    }
}
