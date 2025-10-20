
use core::{Column, Table, Value, ValueType};
use std::io::stdout;

use cli::Shell;
use database::Database;
use storage::{MemoryStorage};

fn main() {

    // Opens a new storage
    let storage = MemoryStorage::new();

    let schema = vec![
        Column { name: "Name".into(), col_type: ValueType::Text },
        Column { name: "Height".into(), col_type: ValueType::Float },
    ];

    // Opens a new table
    let mut table = Table::new("default", schema, storage);

    table.insert(vec![Value::Text("Alice".into()), Value::Float(170.5)]);

    table.insert(vec![Value::Text("Bob".into()), Value::Float(183.2)]);

    // Create database 
    // TODO: Reorder this sensibly
    let database = Database { table };

    println!("Kirin DB initialized with MemoryStorage.");

    let _ = Shell::new(database, &mut stdout()).run();
}
