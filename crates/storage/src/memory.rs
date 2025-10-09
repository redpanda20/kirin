use std::collections::HashMap;

use crate::{storage::RowId, Row, Storage};

/// In memory storage implementation

pub struct MemoryStorage {
    data: HashMap<RowId, Row>,
    next_id: RowId
}

impl MemoryStorage {
    pub fn new() -> Self {
        let data = HashMap::new();
        let next_id = 0;

        Self {
            data,
            next_id,
        }
    }
}

impl Storage for MemoryStorage {
    fn insert(&mut self, row: crate::Row) -> RowId {
        let id = self.next_id;

        self.data.insert(id, row);
        self.next_id += 1;
        
        id
    }

    fn get(&self, row_id: RowId) -> Option<&crate::Row> {
        self.data.get(&row_id)
    }

    fn delete(&mut self, row_id: RowId) -> bool {
        self.data.remove(&row_id).is_some()
    }

    fn iter(&self) -> Box<dyn Iterator<Item = &crate::Row> + '_> {
        Box::new(self.data.values())
    }
}