use core::{Row, Value, Storage};

use storage::MemoryStorage;


#[test]
fn memory_storage_insert_and_fetch() {
    // Create a new in-memory storage
    let mut store = MemoryStorage::new();

    // Create a row
    let row = Row {
        values: vec![Value::Int(42), Value::Text("Alice".to_string())],
    };

    // Insert the row into storage
    let row_id = store.insert(row.clone());

    // Fetch the row by ID
    let fetched = store.get(row_id).expect("Row should exist");

    // Assert the fetched row matches inserted row
    assert_eq!(fetched.values.len(), 2);

    match &fetched.values[0] {
        Value::Int(n) => assert_eq!(*n, 42),
        _ => panic!("Expected first column to be Int"),
    }

    match &fetched.values[1] {
        Value::Text(s) => assert_eq!(s, "Alice"),
        _ => panic!("Expected second column to be Text"),
    }

}


#[test]
fn memory_storage_insert_and_iter() {
    // Create a new in-memory storage
    let mut store = MemoryStorage::new();

        // Create a row
    let row = Row {
        values: vec![Value::Int(42), Value::Text("Alice".to_string())],
    };

    store.insert(row);

    // Iteration test
    let all_rows: Vec<_> = store.iter().collect();
    assert_eq!(all_rows.len(), 1);
}