use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::enums::SettingsValueType;
use crate::models::CreateSettingsRequest;
use sqlx::sqlite::SqlitePool;


pub struct SettingsDB {
    pub pool: SqlitePool,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct SettingsDBRow {
    pub key: String,
    pub value: String,
    #[serde(rename = "type")]
    pub value_type: SettingsValueType,

    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl From<CreateSettingsRequest> for SettingsDBRow {
    fn from(request: CreateSettingsRequest) -> Self {
        let now = Utc::now();
        Self {
            key: request.key,
            value: request.value,
            value_type: request.value_type,
            created_at: now,
            updated_at: now,
        }
    }
}
