use osom_lib_alloc::traits::Allocator;
use serde::{Deserialize, Serialize, de};

use super::cbox::CBox;

impl<T, TAllocator: Allocator> Serialize for CBox<T, TAllocator>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}

impl<'de, T, TAllocator: Allocator + Default> Deserialize<'de> for CBox<T, TAllocator>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = T::deserialize(deserializer)?;
        let arc = CBox::new(value).map_err(de::Error::custom)?;
        Ok(arc)
    }
}
