use sqlx;
use crate::errors::CustomError;
use crate::models::{SettingsDBRow, SettingsDB};
use crate::utils::validate_settings_value;
use sqlx::{SqlitePool, Transaction, Sqlite};
use chrono::Utc;

pub async fn db_create_settings(
    db: &SettingsDB,
    settings_row: &SettingsDBRow,
) -> Result<Option<String>, sqlx::Error> {
    let value_type_str = settings_row.value_type.to_string();

    let result = sqlx::query!(
        r#"
        INSERT INTO settings (key, value, type, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT(key) DO NOTHING
        "#,
        settings_row.key,
        settings_row.value,
        value_type_str,
        settings_row.created_at,
        settings_row.updated_at
    )
    .execute(&db.pool)
    .await?;

    if result.rows_affected() > 0 {
        Ok(Some(settings_row.key.clone()))
    } else {
        Ok(None)
    }
}

pub async fn db_get_settings_by_key(
    db: &SettingsDB,
    key: &str,
) -> Result<Option<SettingsDBRow>, sqlx::Error> {
    let row: Option<SettingsDBRow> = sqlx::query_as!(
        SettingsDBRow,
        r#"
        SELECT
            key as "key!",
            value as "value!",
            type as "value_type!: _",
            created_at as "created_at!: _",
            updated_at as "updated_at!: _"
        FROM settings
        WHERE key = $1
        "#,
        key
    )
    .fetch_optional(&db.pool)
    .await?;

    Ok(row)
}

pub async fn db_get_settings(
    db: &SettingsDB,
    prefix: String,
) -> Result<Vec<SettingsDBRow>, sqlx::Error> {
    let like_pattern = format!("{}%", prefix);

    let rows: Vec<SettingsDBRow> = sqlx::query_as!(
        SettingsDBRow,
        r#"
        SELECT
            key as "key!",
            value as "value!",
            type as "value_type!: _",
            created_at as "created_at!: _",
            updated_at as "updated_at!: _"
        FROM settings
        WHERE key LIKE $1
        "#,
        like_pattern
    )
    .fetch_all(&db.pool)
    .await?;
    Ok(rows)
}

pub async fn db_delete_settings_by_keys(
    db: &SettingsDB,
    keys: Vec<String>,
) -> Result<u64, sqlx::Error> {
    if keys.is_empty() {
        return Ok(0);
    }

    // Build placeholders for the keys e.g., $1, $2, ..., $n
    let placeholders = (1..=keys.len())
        .map(|i| format!("${}", i))
        .collect::<Vec<_>>()
        .join(", ");

    let query = format!("DELETE FROM settings WHERE key IN ({})", placeholders);
    let mut q = sqlx::query(&query);
    for key in &keys {
        q = q.bind(key);
    }
    let result = q.execute(&db.pool).await?;
    Ok(result.rows_affected())
}

pub async fn db_update_settings_by_key(
    pool: &SqlitePool,
    key: &str,
    value: &str,
) -> Result<Option<String>, CustomError> {
    // Begin a transaction
    let mut tx: Transaction<'_, Sqlite> = pool.begin().await.map_err(CustomError::from)?;

    // Fetch current settings row
    let existing = sqlx::query_as!(
        SettingsDBRow,
        r#"
        SELECT
            key as "key!",
            value as "value!",
            type as "value_type!: _",
            created_at as "created_at!: _",
            updated_at as "updated_at!: _"
        FROM settings
        WHERE key = $1
        "#,
        key
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(CustomError::from)?;

    let Some(settings) = existing else {
        return Ok(None);
    };

    // Validate before updating
    validate_settings_value(value.to_string(), settings.value_type.clone())?;

    // Perform update inside the same transaction
    let now = Utc::now();
    let updated = sqlx::query!(
        r#"
        UPDATE settings
        SET value = $1,
            updated_at = $2
        WHERE key = $3
        RETURNING key
        "#,
        value,
        now,
        key
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(CustomError::from)?;

    tx.commit().await.map_err(CustomError::from)?;

    Ok(updated.map(|r| r.key).expect("Key should be present"))
}
