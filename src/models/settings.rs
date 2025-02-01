use std::fmt;

use fjall::{UserKey, UserValue};
use serde::{Deserialize, Serialize};

use crate::errors::CustomError;

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

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub key: String,
    pub value: String,
    #[serde(rename = "type")]
    pub value_type: SettingsValueType,
}

impl From<&Settings> for Vec<u8> {
    fn from(val: &Settings) -> Self {
        rmp_serde::to_vec(&val).expect("Error serializing settings to bytes")
    }
}

impl From<(UserKey, UserValue)> for Settings {
    fn from((key, value): (UserKey, UserValue)) -> Self {
        let key = std::str::from_utf8(&key).unwrap();
        let mut item: Settings =
            rmp_serde::from_slice(&value).expect("Error deserializing settings from bytes");
        key.clone_into(&mut item.key);
        item
    }
}

impl fmt::Display for Settings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}, {}", self.key, self.value, self.value_type)
    }
}

impl Settings {
    pub fn validate(&self) -> Result<(), CustomError> {
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
