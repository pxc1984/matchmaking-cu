use std::fmt::{Display, Formatter};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug)]
pub struct Epoch {
    pub content: String,
}

// helper module to handle serialization and deserialization
mod string_epoch {
    use super::Epoch;
    use serde::{self, Deserialize, Serialize, Deserializer, Serializer};

    #[derive(Deserialize, Serialize)]
    #[serde(remote = "Epoch")]
    pub struct EpochDef {
        pub content: String,
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Epoch, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Epoch { content: s })
    }

    pub fn serialize<S>(epoch: &Epoch, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        epoch.content.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Epoch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        string_epoch::deserialize(deserializer)
    }
}

impl Serialize for Epoch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        string_epoch::serialize(self, serializer)
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