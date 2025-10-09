use crate::storage::{Storage, RowId};
use crate::Row;

/// Disk backed storage implementation
pub struct DiskStorage;

impl DiskStorage {
    pub fn open(path: &str) -> Self {
        unimplemented!("Disk storage open not implemeneted yet");
        Self {}
    }
}

impl Storage for DiskStorage {
    fn insert(&mut self, row: Row) -> RowId {
        unimplemented!("Disk-backed insert not implemented yet")
    }

    fn get(&self, row_id: RowId) -> Option<&Row> {
        unimplemented!("Disk-backed get not implemented yet")
    }

    fn delete(&mut self, row_id: RowId) -> bool {
        unimplemented!("Disk-backed delete not implemented yet")
    }

    fn iter(&self) -> Box<dyn Iterator<Item = &Row> + '_> {
        unimplemented!("Disk-backed iter not implemented yet")
    }
}
