use serde::{Deserialize, Serialize, de};

use osom_lib_reprc::traits::ReprC;

use super::coption::COption;
use super::kvp::KVP;
use super::length::Length;
use super::power_of_two::{PowerOfTwo32, PowerOfTwo64};

impl<TValue: ReprC + Serialize> Serialize for COption<TValue> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().into_option().serialize(serializer)
    }
}

impl<'de, TValue: ReprC + Deserialize<'de>> Deserialize<'de> for COption<TValue> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let option = Option::<TValue>::deserialize(deserializer)?;
        Ok(COption::from_option(option))
    }
}

impl Serialize for Length {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_u32().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Length {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        Length::try_from_u32(value).map_err(de::Error::custom)
    }
}

impl Serialize for PowerOfTwo32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PowerOfTwo32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        PowerOfTwo32::new(value).map_err(de::Error::custom)
    }
}

impl Serialize for PowerOfTwo64 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PowerOfTwo64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u64::deserialize(deserializer)?;
        PowerOfTwo64::new(value).map_err(de::Error::custom)
    }
}

impl<TKey: Serialize, TValue: Serialize> Serialize for KVP<TKey, TValue> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_tuple().serialize(serializer)
    }
}

impl<'de, TKey: Deserialize<'de>, TValue: Deserialize<'de>> Deserialize<'de> for KVP<TKey, TValue> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let tuple = <(TKey, TValue)>::deserialize(deserializer)?;
        Ok(tuple.into())
    }
}
