use chrono::{DateTime, Utc};
use fjall::{Keyspace, PartitionHandle, UserKey, UserValue};
use serde::{Deserialize, Serialize};

use crate::enums::SettingsValueType;
use crate::models::SettingsRequest;


pub struct SettingsDB {
    #[allow(unused)]
    pub keyspace: Keyspace,

    pub partition: PartitionHandle,
}

#[derive(Serialize, Deserialize)]
pub struct SettingsDBRow {
    pub key: String,
    pub value: String,
    #[serde(rename = "type")]
    pub value_type: SettingsValueType,
    pub created_at: DateTime<Utc>,
}

impl From<&SettingsDBRow> for Vec<u8> {
    fn from(val: &SettingsDBRow) -> Self {
        rmp_serde::to_vec(&val).expect("Error serializing settings to bytes")
    }
}

impl From<(UserKey, UserValue)> for SettingsDBRow {
    fn from((key, value): (UserKey, UserValue)) -> Self {
        let key = std::str::from_utf8(&key).unwrap();
        let mut item: SettingsDBRow =
            rmp_serde::from_slice(&value).expect("Error deserializing settings from bytes");
        key.clone_into(&mut item.key);
        item
    }
}

impl From<SettingsRequest> for SettingsDBRow {
    fn from(request: SettingsRequest) -> Self {
        Self {
            key: request.key,
            value: request.value,
            value_type: request.value_type,
            created_at: Utc::now()
        }
    }
}
