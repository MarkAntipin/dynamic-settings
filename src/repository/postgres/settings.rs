use crate::models::Settings;
use sqlx;
use sqlx::PgPool;

pub async fn pg_add_settings(
    pool: &PgPool,
    settings: &Settings,
) -> Result<Option<String>, sqlx::Error> {
    let query = sqlx::query!(
        r#"
        INSERT INTO settings (
            key,
            value,
            value_type
        )
        VALUES ($1, $2, $3)
        ON CONFLICT (key) DO NOTHING
        RETURNING key
        "#,
        settings.key,
        settings.value,
        settings.value_type.to_string()
    );
    let row = query.fetch_optional(pool).await?;
    Ok(row.map(|row| row.key))
}

pub async fn pg_get_settings_by_key(
    pool: &PgPool,
    key: &str,
) -> Result<Option<Settings>, sqlx::Error> {
    let row = sqlx::query_as!(
        Settings,
        r#"
        SELECT key, value, value_type
        FROM settings
        WHERE key = $1
        "#,
        key,
    )
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn pg_get_settings(pool: &PgPool) -> Result<Vec<Settings>, sqlx::Error> {
    let rows = sqlx::query_as!(
        Settings,
        r#"
        SELECT key, value, value_type
        FROM settings
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
