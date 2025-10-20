use crate::Value;

pub type RowId = u64;

#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    pub values: Vec<Value>,
}
