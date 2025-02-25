use fjall;
use chrono::Utc;
use crate::errors::CustomError;
use crate::models::{SettingsDBRow, SettingsDB};
use crate::utils::validate_settings_value;

pub fn db_create_settings(
    db: &SettingsDB,
    settings_row: &SettingsDBRow,
) -> Result<Option<String>, fjall::Error> {
    let key = &settings_row.key;

    let Some(_) = db.partition.get(key)? else {
        let serialized: Vec<u8> = settings_row.into();
        db.partition
            .insert(key, serialized)
            .expect("Failed to insert settings");
        db.keyspace.persist(fjall::PersistMode::SyncAll)?;

        return Ok(Some(key.clone()));
    };
    Ok(None)
}

pub fn db_get_settings_by_key(
    db: &SettingsDB,
    key: &str,
) -> Result<Option<SettingsDBRow>, fjall::Error> {
    let Some(item) = db.partition.get(key)? else {
        return Ok(None);
    };

    let settings: SettingsDBRow = rmp_serde::from_slice(&item)
        .expect("Error deserializing settings from bytes");
    Ok(Some(settings))
}

pub fn db_get_settings(db: &SettingsDB) -> Result<Vec<SettingsDBRow>, fjall::Error> {
    let read_tx = db.keyspace.read_tx();
    let settings = read_tx
        .iter(&db.partition)
        .map(|item| item.map(SettingsDBRow::from))
        .collect::<Result<Vec<SettingsDBRow>, _>>()?;

    Ok(settings)
}

pub fn db_delete_settings_by_keys(
    db: &SettingsDB,
    keys: Vec<String>,
) -> Result<(), fjall::Error> {
    for key in keys {
        db.partition.remove(&key)?;
    }
    db.keyspace.persist(fjall::PersistMode::SyncAll)?;
    Ok(())
}

pub fn db_update_settings_by_key(
    db: &SettingsDB,
    key: &String,
    value: &String,
) -> Result<Option<String>, CustomError> {
    let mut write_tx = db.keyspace.write_tx().durability(
        Some(fjall::PersistMode::SyncAll)
    );

    let Some(item) = write_tx.get(&db.partition, key)? else {
        return Ok(None);
    };
    let settings: SettingsDBRow = rmp_serde::from_slice(&item)
        .expect("Error deserializing settings from bytes");

    validate_settings_value(value.clone(), settings.value_type.clone())?;

    let settings_row = &SettingsDBRow {
        key: settings.key.clone(),
        value: value.to_string(),
        value_type: settings.value_type,
        created_at: settings.created_at,
        updated_at: Utc::now(),
    };
    let serialized: Vec<u8> = settings_row.into();

    write_tx.insert(&db.partition, key, serialized);
    write_tx.commit()?;

    Ok(Some(settings.key))
}
