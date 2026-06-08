use osom_lib_alloc::traits::Allocator;
use osom_lib_try_clone::TryClone;
use serde::{Deserialize, Serialize, de::{DeserializeSeed, IntoDeserializer, Visitor, value::{MapAccessDeserializer, SeqAccessDeserializer}}};

use crate::cvr::{CVR, CVRBool, CVRFloat, CVRInt, serde::{CVRArraySeed, CVRObjectSeed, CVRStringSeed}};

use super::{make_seed_struct, CVRDeserializeContext};

make_seed_struct!(CVRSeed);

impl<TAllocator: Allocator> Serialize for CVR<TAllocator> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Null => serializer.serialize_unit(),
            Self::Bool(value) => value.serialize(serializer),
            Self::Int(value) => value.serialize(serializer),
            Self::String(value) => value.serialize(serializer),
            Self::Float(value) => value.serialize(serializer),
            Self::Array(value) => value.serialize(serializer),
            Self::Object(value) => value.serialize(serializer),
        }
    }
}

struct CVRVisitor<'a, TAllocator: Allocator + TryClone> {
    context: &'a mut CVRDeserializeContext<TAllocator>,
}

impl<'de, TAllocator: Allocator + TryClone> Visitor<'de> for CVRVisitor<'_, TAllocator> {
    type Value = CVR<TAllocator>;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("a CVR value")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let result = CVRBool::deserialize(v.into_deserializer())?;
        Ok(CVR::Bool(result))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CVR::Null)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CVR::Null)
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let string_seed = CVRStringSeed { context: self.context };
        let result = string_seed.deserialize(v.into_deserializer())?;
        Ok(CVR::String(result))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let string_seed = CVRStringSeed { context: self.context };
        let result = string_seed.deserialize(v.into_deserializer())?;
        Ok(CVR::String(result))
    }

    #[cfg(feature = "std")]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let string_seed = CVRStringSeed { context: self.context };
        let result = string_seed.deserialize(v.into_deserializer())?;
        Ok(CVR::String(result))
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let array_seed = CVRArraySeed { context: self.context };
        let result = array_seed.deserialize(SeqAccessDeserializer::new(seq))?;
        Ok(CVR::Array(result))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let object_seed = CVRObjectSeed { context: self.context };
        let result = object_seed.deserialize(MapAccessDeserializer::new(map))?;
        Ok(CVR::Object(result))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let result = CVRFloat::deserialize(v.into_deserializer())?;
        Ok(CVR::Float(result))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let result = CVRFloat::deserialize(f64::from(v).into_deserializer())?;
        Ok(CVR::Float(result))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let result = CVRInt::deserialize(i128::from(v).into_deserializer())?;
        Ok(CVR::Int(result))
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let result = CVRInt::deserialize(i128::from(v).into_deserializer())?;
        Ok(CVR::Int(result))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let result = CVRInt::deserialize(i128::from(v).into_deserializer())?;
        Ok(CVR::Int(result))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let result = CVRInt::deserialize(i128::from(v).into_deserializer())?;
        Ok(CVR::Int(result))
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let Ok(value) = i128::try_from(v) else {
            return Err(E::custom("u128 value is too large"));
        };
        let result = CVRInt::deserialize(value.into_deserializer())?;
        Ok(CVR::Int(result))
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let result = CVRInt::deserialize(i128::from(v).into_deserializer())?;
        Ok(CVR::Int(result))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let result = CVRInt::deserialize(i128::from(v).into_deserializer())?;
        Ok(CVR::Int(result))
    }


    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let result = CVRInt::deserialize(i128::from(v).into_deserializer())?;
        Ok(CVR::Int(result))
    }
    
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let result = CVRInt::deserialize(i128::from(v).into_deserializer())?;
        Ok(CVR::Int(result))
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let result = CVRInt::deserialize(v.into_deserializer())?;
        Ok(CVR::Int(result))
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let seed = CVRSeed { context: self.context };
        seed.deserialize(deserializer)
    }
}

impl<'de, TAllocator: Allocator + TryClone> DeserializeSeed<'de> for CVRSeed<'_, TAllocator> {
    type Value = CVR<TAllocator>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        deserializer.deserialize_any(CVRVisitor { context: self.context })
    }
}
