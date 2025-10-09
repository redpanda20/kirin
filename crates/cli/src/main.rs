
use storage::{MemoryStorage, Value};
use table::{Column, Table};

mod shell;
use crate::shell::Shell;

fn main() {

    // Opens a new storage
    let storage = MemoryStorage::new();
    
    println!("Kirin DB initialized with MemoryStorage.");

    let schema = vec![
        Column { name: "Name".into(), col_type: table::ValueType::Text },
        Column { name: "Height".into(), col_type: table::ValueType::Float },
    ];

    // Opens a new table
    let mut table = Table::new("default", schema, storage);

    table.insert(vec![Value::Text("Alice".into()), Value::Float(170.5)]);

    table.insert(vec![Value::Text("Bob".into()), Value::Float(183.2)]);


    Shell::new(table).run()
}
