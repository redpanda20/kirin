use core::{Column, Table, Value, ValueType};

use storage::MemoryStorage;


#[test]
fn insert_and_retrieve_row() {
    let columns = vec![
        Column { name: "id".into(), col_type: ValueType::Int },
        Column { name: "name".into(), col_type: ValueType::Text },
    ];
    let mut table = Table::new(columns, MemoryStorage::new());

    let inserted = table.insert(vec![Value::Int(1), Value::Text("Alice".into())]);
    assert!(inserted.is_some(), "Insert should succeed");

    let row_id = inserted.unwrap();
    let row = table.get(row_id).expect("Row should exist");

    assert_eq!(row.values.len(), 2);
    assert_eq!(row.values[0], Value::Int(1));
    assert_eq!(row.values[1], Value::Text("Alice".into()));
}

#[test]
fn insert_fails_on_column_mismatch() {
    let columns = vec![
        Column { name: "id".into(), col_type: ValueType::Int },
        Column { name: "name".into(), col_type: ValueType::Text },
    ];
    let mut table = Table::new(columns, MemoryStorage::new());

    // Only one value instead of two
    let result = table.insert(vec![Value::Int(42)]);
    assert!(result.is_none(), "Insert should fail on column count mismatch");
}

#[test]
fn insert_fails_on_type_mismatch() {
    let columns = vec![
        Column { name: "id".into(), col_type: ValueType::Int },
        Column { name: "name".into(), col_type: ValueType::Text },
    ];
    let mut table = Table::new(columns, MemoryStorage::new());

    // id should be Int, but given Text
    let result = table.insert(vec![
        Value::Text("Not an Int".into()),
        Value::Text("Alice".into())
    ]);
    assert!(result.is_none(), "Insert should fail on type mismatch");
}

#[test]
fn iterate_returns_all_rows() {
    let columns = vec![
        Column { name: "id".into(), col_type: ValueType::Int },
        Column { name: "name".into(), col_type: ValueType::Text },
    ];
    let mut table = Table::new(columns, MemoryStorage::new());

    table.insert(vec![Value::Int(1), Value::Text("Alice".into())]);
    table.insert(vec![Value::Int(2), Value::Text("Bob".into())]);

    let rows: Vec<_> = table.iter().collect();
    assert_eq!(rows.len(), 2);

    let names: Vec<Value> = rows.iter().map(|row| row.values[1].clone()).collect();
    assert!(names.contains(&Value::Text("Alice".into())));
    assert!(names.contains(&Value::Text("Bob".into())));

}
