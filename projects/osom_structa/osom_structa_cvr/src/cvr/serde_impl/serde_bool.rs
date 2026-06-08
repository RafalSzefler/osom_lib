use serde::{Deserialize, Serialize};

use crate::cvr::CVRBool;

impl Serialize for CVRBool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bool(self.inner())
    }
}

impl<'de> Deserialize<'de> for CVRBool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = bool::deserialize(deserializer)?;
        Ok(Self::new(value))
    }
}
