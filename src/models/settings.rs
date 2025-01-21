use crate::errors::CustomError;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SettingsValueType {
    Str,
    Int,
    Float,
    Bool,
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub key: String,
    pub value: String,
    #[serde(rename = "type")]
    pub value_type: SettingsValueType,
}

impl fmt::Display for SettingsValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_str = match self {
            SettingsValueType::Str => "str",
            SettingsValueType::Int => "int",
            SettingsValueType::Bool => "bool",
            SettingsValueType::Float => "float",
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
            _ => SettingsValueType::Str,
        }
    }
}

impl Settings {
    pub fn validate(&self) -> Result<(), CustomError> {
        match self.value_type {
            SettingsValueType::Str => Ok(()),
            SettingsValueType::Int => {
                self.value.parse::<i64>().map_err(|_| {
                    CustomError::ValidationError(format!(
                        "Value '{}' is not a valid integer",
                        self.value
                    ))
                })?;
                Ok(())
            }
            SettingsValueType::Bool => {
                self.value.parse::<bool>().map_err(|_| {
                    CustomError::ValidationError(format!(
                        "Value '{}' is not a valid boolean",
                        self.value
                    ))
                })?;
                Ok(())
            }
            SettingsValueType::Float => {
                self.value.parse::<f64>().map_err(|_| {
                    CustomError::ValidationError(format!(
                        "Value '{}' is not a valid float",
                        self.value
                    ))
                })?;
                Ok(())
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MessageResponse {
    pub message: String,
}
