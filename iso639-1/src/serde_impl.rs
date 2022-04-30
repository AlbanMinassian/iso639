use serde::{de::Error, Deserialize, Serialize};
use std::convert::TryFrom;

use crate::Iso639_1;

impl<'de> Deserialize<'de> for Iso639_1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Iso639_1::try_from(s).map_err(|_| {
            D::Error::invalid_value(serde::de::Unexpected::Str(s), &"iso-639-1 string")
        })
    }
}

impl Serialize for Iso639_1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        str::serialize(self.name(), serializer)
    }
}
