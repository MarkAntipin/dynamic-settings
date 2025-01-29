use crate::errors::CustomError;
use serde::{Deserialize, Serialize};
use std::fmt;

const MAX_KEY_LENGTH: usize = 1024;
const MAX_VALUE_LENGTH: usize = 65536;

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
        println!("Validating settings: {}", self.key.len());

        if self.key.len() > MAX_KEY_LENGTH {
            return Err(CustomError::ValidationError(format!(
                "Key length should be less than {} bytes",
                MAX_KEY_LENGTH
            )));
        }

        if self.value.len() > MAX_VALUE_LENGTH {
            return Err(CustomError::ValidationError(format!(
                "Value length should be less than {} bytes",
                MAX_VALUE_LENGTH
            )));
        }

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
