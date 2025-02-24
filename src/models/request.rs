use fjall::{UserKey, UserValue};
use serde::{Deserialize, Serialize};

use crate::errors::CustomError;
use crate::enums::SettingsValueType;

const MAX_KEY_LENGTH: usize = 1024;
const MAX_VALUE_LENGTH: usize = 65536;

#[derive(Serialize, Deserialize)]
pub struct SettingsRequest {
    pub key: String,
    pub value: String,
    #[serde(rename = "type")]
    pub value_type: SettingsValueType,
}

impl From<&SettingsRequest> for Vec<u8> {
    fn from(val: &SettingsRequest) -> Self {
        rmp_serde::to_vec(&val).expect("Error serializing settings to bytes")
    }
}

impl From<(UserKey, UserValue)> for SettingsRequest {
    fn from((key, value): (UserKey, UserValue)) -> Self {
        let key = std::str::from_utf8(&key).unwrap();
        let mut item: SettingsRequest =
            rmp_serde::from_slice(&value).expect("Error deserializing settings from bytes");
        key.clone_into(&mut item.key);
        item
    }
}

impl SettingsRequest {
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
            SettingsValueType::Json => {
                serde_json::from_str::<serde_json::Value>(&self.value).map_err(|_| {
                    CustomError::ValidationError(format!(
                        "Value '{}' is not a valid JSON",
                        self.value
                    ))
                })?;
                Ok(())
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DeleteSettingsByKeysRequest {
    pub keys: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ValidateTokenRequest {
    pub token: String,
}
