use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SettingsValueType {
    Str,
    Int,
    Float,
    Bool,
    Json
}

impl fmt::Display for SettingsValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_str = match self {
            SettingsValueType::Str => "str",
            SettingsValueType::Int => "int",
            SettingsValueType::Bool => "bool",
            SettingsValueType::Float => "float",
            SettingsValueType::Json => "json",
        };
        write!(f, "{}", as_str)
    }
}

impl From<String> for SettingsValueType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "int" => SettingsValueType::Int,
            "str" => SettingsValueType::Str,
            "bool" => SettingsValueType::Bool,
            "float" => SettingsValueType::Float,
            "json" => SettingsValueType::Json,
            // TODO: can forget to add a new type here
            _ => SettingsValueType::Str,
        }
    }
}
