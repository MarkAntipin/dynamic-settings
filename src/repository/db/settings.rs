use fjall;

use crate::models::{SettingsDBRow, SettingsDB};

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

    let settings: SettingsDBRow =
        rmp_serde::from_slice(&item).expect("Error deserializing settings from bytes");
    Ok(Some(settings))
}

pub fn db_get_settings(db: &SettingsDB) -> Result<Vec<SettingsDBRow>, fjall::Error> {
    let settings = db
        .partition
        .iter()
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
