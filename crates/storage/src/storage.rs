/// Generic storage trait

use crate::Row;

pub type RowId = u64;

pub trait Storage {
    /// Insert a row; returns a RowId for retrieval
    fn insert(&mut self, row: Row) -> RowId;

    /// Get a row by RowId
    fn get(&self, row_id: RowId) -> Option<&Row>;

    /// Delete a row by RowId
    fn delete(&mut self, row_id: RowId) -> bool;

    /// Iterate over all rows
    fn iter(&self) -> Box<dyn Iterator<Item = &Row> + '_>;
}
