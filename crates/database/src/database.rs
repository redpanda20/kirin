use storage::MemoryStorage;
use core::Table;

pub struct Database {
    pub table: Table<MemoryStorage>
}