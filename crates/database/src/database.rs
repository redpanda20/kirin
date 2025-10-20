use storage::MemoryStorage;
use core::Table;
use std::collections::HashMap;

pub struct Database {
    tables: HashMap<String, Table<MemoryStorage>>
}

impl Database {

    pub fn new() -> Self {
        Self { tables: HashMap::new() }
    }

    pub fn add_table(&mut self, name: impl Into<String>, table: Table<MemoryStorage>) {
        self.tables.insert(name.into(), table);
    }

    pub fn get_table(&mut self, name: impl Into<String>) -> Option<&mut Table<MemoryStorage>> {
        self.tables.get_mut(&name.into())
    }

    pub fn get_table_names(&self) -> impl Iterator<Item = &String>{
        self.tables.keys()
    }

}