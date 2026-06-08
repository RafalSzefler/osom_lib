use serde::{Deserialize, Serialize};

use crate::cvr::CVRFloat;

impl Serialize for CVRFloat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CVRFloat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = f64::deserialize(deserializer)?;
        Ok(Self::new(value))
    }
}
