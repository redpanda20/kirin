pub mod storage;
pub mod memory;
pub mod disk;

pub use storage::Storage;
pub use memory::MemoryStorage;
pub use disk::DiskStorage;

#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    pub values: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Text(String),

    // ...
}