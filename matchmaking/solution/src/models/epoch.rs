use std::fmt::{Display, Formatter};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Visitor};

#[derive(Clone, Debug)]
pub struct Epoch {
    pub content: String,
}

impl Serialize for Epoch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(&self.content)
    }
}

impl<'de> Deserialize<'de> for Epoch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_string(EpochVisitor)
    }
}

impl Display for Epoch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl Epoch {
    pub fn new() -> Epoch {
        Epoch {
            content: "00000000-0000-0000-0000-000000000000".to_string()
        }
    }

    pub fn from(str: &String) -> Epoch {
        Epoch {
            content: str.clone(),
        }
    }
}

struct EpochVisitor;

impl<'de> Visitor<'de> for EpochVisitor {
    type Value = Epoch;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "just a simple string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error
    {
        Ok(Epoch { content: v.parse().expect("Failed to convert &str to String") })
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error
    {
        Ok(Epoch { content: v })
    }
}