use crate::base::JsonObject;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize, Deserializer};
use serde::de::{self, Visitor};
use std::collections::HashSet;

/// Custom deserializer that handles both boolean and string values
fn deserialize_bool_or_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    struct BoolOrStringVisitor;

    impl<'de> Visitor<'de> for BoolOrStringVisitor {
        type Value = bool;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a boolean or a string")
        }

        fn visit_bool<E>(self, value: bool) -> Result<bool, E>
        where
            E: de::Error,
        {
            Ok(value)
        }

        fn visit_str<E>(self, value: &str) -> Result<bool, E>
        where
            E: de::Error,
        {
            // If it's a string like "mtgo", "paper", "arena", treat as true
            // Empty string or null-like values are false
            Ok(!value.is_empty() && value != "null" && value != "false")
        }

        fn visit_string<E>(self, value: String) -> Result<bool, E>
        where
            E: de::Error,
        {
            self.visit_str(&value)
        }
    }

    deserializer.deserialize_any(BoolOrStringVisitor)
}

/// MTGJSON Singular Card.GameFormats Object
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[pyclass(name = "MtgjsonGameFormatsObject")]
pub struct MtgjsonGameFormatsObject {
    #[pyo3(get, set)]
    #[serde(deserialize_with = "deserialize_bool_or_string")]
    pub paper: bool,
    
    #[pyo3(get, set)]
    #[serde(deserialize_with = "deserialize_bool_or_string")]
    pub mtgo: bool,
    
    #[pyo3(get, set)]
    #[serde(deserialize_with = "deserialize_bool_or_string")]
    pub arena: bool,
    
    #[pyo3(get, set)]
    #[serde(deserialize_with = "deserialize_bool_or_string")]
    pub shandalar: bool,
    
    #[pyo3(get, set)]
    #[serde(deserialize_with = "deserialize_bool_or_string")]
    pub dreamcast: bool,
}

#[pymethods]
impl MtgjsonGameFormatsObject {
    #[new]
    pub fn new() -> Self {
        Self {
            paper: false,
            mtgo: false,
            arena: false,
            shandalar: false,
            dreamcast: false,
        }
    }

    /// Convert to JSON - returns list of available formats
    pub fn to_json(&self) -> PyResult<Vec<String>> {
        let mut formats = Vec::new();
        
        if self.paper {
            formats.push("paper".to_string());
        }
        if self.mtgo {
            formats.push("mtgo".to_string());
        }
        if self.arena {
            formats.push("arena".to_string());
        }
        if self.shandalar {
            formats.push("shandalar".to_string());
        }
        if self.dreamcast {
            formats.push("dreamcast".to_string());
        }
        
        Ok(formats)
    }

    /// Get available formats as a list
    pub fn get_available_formats(&self) -> Vec<String> {
        self.to_json().unwrap_or_default()
    }
}

impl JsonObject for MtgjsonGameFormatsObject {}

impl From<&[&str]> for MtgjsonGameFormatsObject {
    fn from(formats: &[&str]) -> Self {
        let mut game_formats = Self::new();
        
        for format in formats {
            match format.to_lowercase().as_str() {
                "paper" => game_formats.paper = true,
                "mtgo" => game_formats.mtgo = true,
                "arena" => game_formats.arena = true,
                "shandalar" => game_formats.shandalar = true,
                "dreamcast" => game_formats.dreamcast = true,
                _ => {}
            }
        }
        
        game_formats
    }
}