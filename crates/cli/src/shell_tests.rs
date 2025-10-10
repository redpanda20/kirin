#[cfg(test)]
mod tests {
    use storage::{MemoryStorage, Value};
    use table::{Table, Column, ValueType};

    use crate::shell::Shell;

    #[test]
    fn handle_select_displays_table_correctly() {
        // Prepare in-memory table
        let storage = MemoryStorage::new();
        let columns = vec![
            Column { name: "id".into(), col_type: ValueType::Int },
            Column { name: "name".into(), col_type: ValueType::Text },
        ];
        let mut table = Table::new("users", columns, storage);

        table.insert(vec![Value::Int(1), Value::Text("Alice".into())]);
        table.insert(vec![Value::Int(2), Value::Text("Bob".into())]);

        // Create a buffer to capture output
        let mut output = Vec::new();

        {
            // Construct shell referencing the buffer
            let mut shell = Shell::new(table, &mut output);
            shell.handle_select("SELECT * FROM users").unwrap();
        }

        let printed = String::from_utf8(output).expect("Valid UTF-8");

        // Check expected fragments
        assert!(printed.contains("(2 rows)"));
        assert!(printed.contains("id <Integer>"));
        assert!(printed.contains("name <Text>"));
        assert!(printed.contains("Alice"));
        assert!(printed.contains("Bob"));
    }

    #[test]
    fn handle_select_on_empty_table_shows_zero_rows() {
        let storage = MemoryStorage::new();
        let columns = vec![
            Column { name: "id".into(), col_type: ValueType::Int },
            Column { name: "name".into(), col_type: ValueType::Text },
        ];
        let table = Table::new("users", columns, storage);

        let mut output = Vec::new();

        {
            let mut shell = Shell::new(table, &mut output);
            shell.handle_select("SELECT * FROM users").unwrap();
        }

        let printed = String::from_utf8(output).unwrap();

        // Check row count and schema
        assert!(printed.contains("(0 rows)"));
        assert!(printed.contains("id <Integer>"));
        assert!(printed.contains("name <Text>"));

        // No data rows expected
        assert!(!printed.contains("| 1 |"));
    }
}
