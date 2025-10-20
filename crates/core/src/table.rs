use std::iter::zip;

use crate::{Column, Row, RowId, Storage, Value, ValueType};

pub struct Table<S: Storage> {
    storage: S,
    pub columns: Vec<Column>,
}

impl<S: Storage> Table<S> {
    pub fn new(columns: Vec<Column>, storage: S) -> Self {
        Self {
            columns,
            storage,
        }
    }

    /// Attempts to insert a logical row.
    /// 
    /// Returns a row id for retrieval
    pub fn insert(&mut self, values: Vec<Value>) -> Option<RowId> {

        // Check if column counts match
        if values.len() != self.columns.len() {
            return None;
        }

        // Check if all column types match
        if !zip(values.iter(), &self.columns).all(|(val, col)| ValueType::from(val) == col.col_type ) {
            return None;
        }

        let row = Row { values };
        Some(self.storage.insert(row))
    }

    /// Attempts to get a single row by row id
    pub fn get(&self, row_id: RowId) -> Option<&Row> {
        self.storage.get(row_id)
    }

    /// Iterate over all rows
    pub fn iter(&self) -> impl Iterator<Item = &Row> {
        self.storage.iter()
    }
}
