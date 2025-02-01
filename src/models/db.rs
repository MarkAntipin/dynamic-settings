use fjall::{Keyspace, PartitionHandle};

pub struct SettingsDB {
    #[allow(unused)]
    pub keyspace: Keyspace,

    pub partition: PartitionHandle,
}
