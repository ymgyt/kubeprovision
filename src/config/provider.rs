use std::str::FromStr;

use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum Provider {
    Aws,
}

impl FromStr for Provider {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_ref() {
            "aws" => Ok(Provider::Aws),
            _ => Err(format!("unsupported provider: {s}")),
        }
    }
}

pub fn deserialize_provider<'de, D>(deserializer: D) -> Result<Provider, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer).and_then(|s| {
        s.parse::<Provider>().map_err(|err| {
            serde::de::Error::invalid_value(serde::de::Unexpected::Str(&s), &err.as_str())
        })
    })
}
