use fjall;

use crate::models::{Settings, SettingsDB};

pub fn db_add_settings(
    db: &SettingsDB,
    settings: &Settings,
) -> Result<Option<String>, fjall::Error> {
    let key = &settings.key;

    let Some(_) = db.partition.get(key)? else {
        let serialized: Vec<u8> = settings.into();
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
) -> Result<Option<Settings>, fjall::Error> {
    let Some(item) = db.partition.get(key)? else {
        return Ok(None);
    };

    let settings: Settings =
        rmp_serde::from_slice(&item).expect("Error deserializing settings from bytes");
    Ok(Some(settings))
}

pub fn db_get_settings(db: &SettingsDB) -> Result<Vec<Settings>, fjall::Error> {
    let settings = db
        .partition
        .iter()
        .map(|item| item.map(Settings::from))
        .collect::<Result<Vec<Settings>, _>>()?;

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
