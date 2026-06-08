use serde::{Deserialize, Serialize, de::Visitor};

use crate::cvr::CVRInt;

impl Serialize for CVRInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i128(self.inner())
    }
}

struct CVRIntVisitor;

impl Visitor<'_> for CVRIntVisitor {
    type Value = CVRInt;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("an integer")
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CVRInt::new(i128::from(v)))
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CVRInt::new(i128::from(v)))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CVRInt::new(i128::from(v)))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CVRInt::new(i128::from(v)))
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let Ok(value) = i128::try_from(v) else {
            return Err(E::custom("u128 value is too large, doesn't fit into CVRInt backed by i128"));
        };
        Ok(CVRInt::new(value))
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CVRInt::new(i128::from(v)))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CVRInt::new(i128::from(v)))
    }


    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CVRInt::new(i128::from(v)))
    }
    
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CVRInt::new(i128::from(v)))
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CVRInt::new(v))
    }
}

impl<'de> Deserialize<'de> for CVRInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_i128(CVRIntVisitor)
    }
}
