use osom_lib_alloc::traits::Allocator;
use osom_lib_arrays::traits::MutableArray as _;
use osom_lib_primitives::length::Length;
use osom_lib_try_clone::TryClone;
use serde::{Serialize, de::{self, DeserializeSeed, Visitor}};

use crate::cvr::{CVRArray, serde::CVRDeserializeContext};

use super::make_seed_struct;
use super::serde_core::CVRSeed;

impl<TAllocator: Allocator> Serialize for CVRArray<TAllocator> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner_ref().as_ref().serialize(serializer)
    }
}

make_seed_struct!(CVRArraySeed);

struct CVRArrayVisitor<'a, TAllocator: Allocator + TryClone> {
    context: &'a mut CVRDeserializeContext<TAllocator>,
}

impl<'de, TAllocator: Allocator + TryClone> Visitor<'de> for CVRArrayVisitor<'_, TAllocator> {
    type Value = CVRArray<TAllocator>;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("a sequence of CVR values")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let capacity = seq.size_hint().unwrap_or(0);
        let capacity = Length::try_from_usize(capacity).map_err(de::Error::custom)?;
        let allocator = self.context.allocator().try_clone()
            .map_err(|_| de::Error::custom("Failed to clone allocator"))?;
        let mut result = Self::Value::with_capacity_and_allocator(capacity, allocator)
            .map_err(de::Error::custom)?;

        while let Some(item) = seq.next_element_seed(CVRSeed { context: self.context })? {
            result.inner_mut().try_push(item).map_err(de::Error::custom)?;
        }

        Ok(result)
    }
}

impl<'de, TAllocator: Allocator + TryClone> DeserializeSeed<'de> for CVRArraySeed<'_, TAllocator> {
    type Value = CVRArray<TAllocator>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>
    {
        deserializer.deserialize_seq(CVRArrayVisitor { context: self.context })
    }
}
