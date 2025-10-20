use core::{Column, Table, Value, ValueType};

use cli::Shell;
use database::Database;
use storage::MemoryStorage;

    #[test]
    fn handle_select_displays_table_correctly() {

        let columns = vec![
            Column { name: "id".into(), col_type: ValueType::Int },
            Column { name: "name".into(), col_type: ValueType::Text },
        ];

        let mut database: Database = Database::new();
        let mut table = Table::new(columns, MemoryStorage::new());

        table.insert(vec![Value::Int(1), Value::Text("Alice".into())]);
        table.insert(vec![Value::Int(2), Value::Text("Bob".into())]);
        database.add_table("users",table);

        // Create a buffer to capture output
        let mut output = Vec::new();

        {
            // Construct shell referencing the buffer
            let mut shell = Shell::new(database, &mut output);
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
        let columns = vec![
            Column { name: "id".into(), col_type: ValueType::Int },
            Column { name: "name".into(), col_type: ValueType::Text },
        ];

        let mut database: Database = Database::new();
        database.add_table("users", Table::new(columns, MemoryStorage::new()));


        let mut output = Vec::new();

        {
            let mut shell = Shell::new(database, &mut output);
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

    #[test]
    fn malformed_select_shows_error_message() {
        let columns = vec![
            Column { name: "id".into(), col_type: ValueType::Int },
            Column { name: "name".into(), col_type: ValueType::Text },
        ];

        let mut database: Database = Database::new();
        database.add_table("users", Table::new(columns, MemoryStorage::new()));

        let mut output = Vec::new();

        {
            let mut shell = Shell::new(database, &mut output);
            shell.handle_select("SELECT * FRM users").unwrap();
        }

        let printed = String::from_utf8(output).expect("Valid UTF-8");

        // Expect error message to mention correct syntax
        assert!(printed.contains("FROM"));
    }

        #[test]
    fn select_from_missing_table_shows_error_message() {
        let columns = vec![
            Column { name: "id".into(), col_type: ValueType::Int },
            Column { name: "name".into(), col_type: ValueType::Text },
        ];

        let mut database: Database = Database::new();
        database.add_table("users", Table::new(columns, MemoryStorage::new()));
        let mut output = Vec::new();

        {
            let mut shell = Shell::new(database, &mut output);
            shell.handle_select("SELECT * FROM students").unwrap();
        }

        let printed = String::from_utf8(output).expect("Valid UTF-8");

        // Expect error message to mention correct syntax
        assert!(printed.contains("students"));
        assert!(printed.contains("not found"));
    }

