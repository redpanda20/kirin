
use core::{Column, Table, Value, ValueType};
use std::io::stdout;

use cli::Shell;
use database::Database;
use storage::{MemoryStorage};

fn main() {

    // Opens a new database
    let mut database = Database::new();

    database.add_table("default",
        Table::new(
        vec![
            Column { name: "Name".into(), col_type: ValueType::Text },
            Column { name: "Height".into(), col_type: ValueType::Float }
        ],
        MemoryStorage::new()
    ));

    let table = database.get_table("default").unwrap();

    table.insert(vec![Value::Text("Alice".into()), Value::Float(170.5)]);
    table.insert(vec![Value::Text("Bob".into()), Value::Float(183.2)]);

    println!("Kirin DB initialized with MemoryStorage.");

    let _ = Shell::new(database, &mut stdout()).run();
}
