use std::io::{self, Write};

use storage::{MemoryStorage, Storage, Value};
use table::{Column, Table};

pub struct Shell {
    table: Table<MemoryStorage>
}

impl Shell {

    pub fn new(table: Table<MemoryStorage>) -> Self {
        Self { table }
    }

    pub fn run(&mut self) {
        println!("Starting CLI REPL... (type .exit to quit)");

        loop {
            print!("kirin> ");
            io::stdout().flush().unwrap();

            // Read
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();

            match self.execute(&line) {
                Ok(_) => (),
                Err(_) => break,
            }
        }

        println!("Exiting Kirin CLI.");
    }

    fn execute(&mut self, input: &str) -> Result<(), ()> {
        match input.trim() {
            ".exit" => return Err(()),

            cmd if cmd.starts_with(".") => self.execute_command(input),

            cmd if cmd.starts_with("SELECT") => self.handle_select(input),

            cmd if cmd.starts_with("INSERT") => self.handle_insert(input),

            _ => ()
        }
        Ok(())
    }

    pub fn execute_command(&mut self, input: &str) {
        let input = &mut input.trim().split_ascii_whitespace();

        let command = input.next().unwrap_or("");
        let args: Vec<_> = input.collect();

        // Evaluate & Print
        match (command, args) {
            (".help", _) => help(),
            (".tables", _) => active_tables(&self.table),
            (".backend", _) => active_backend(),
            (".schema", args) => schema(&self.table, args),
            _ => println!("Unknown command, type .help"),
        };
    }

    pub fn handle_select(&mut self, input: &str) {

        // Expected format:
        // "SELECT * FROM <table>"

        let input = input.trim();
    
        let input_upper = input.to_uppercase();
        if !input_upper.starts_with("SELECT") {
            println!("Malformed SELECT command");
            return;
        }

        // Very naive split (stub only)
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() < 4 {
            println!("Expected syntax: SELECT * FROM <table>");
            return;
        }

        // Check for FROM keyword
        if parts.get(2).unwrap().trim() != "FROM" {
            println!("Missing FROM clause");
            return;
        }

        // Check table name matches
        let table_name = *parts.get(3).unwrap_or(&"");
        if table_name != self.table.name {
            println!("Table '{}' not found", table_name);
            return;
        }

        // Print table information
        let row_count = self.table.iter().count();
        println!("({row_count} rows)");

        // Print schema
        let column_schema = self.table.columns.iter()
            .map(|Column{ name, col_type }| format!("{name} <{col_type}>"))
            .fold(String::from("|"), |acc, x| format!("{acc} {x} |"));
        println!("{column_schema}");

        // Print spacer
        let row_spacer = self.table.columns.iter()
            .map(|_| String::from(" --- "))
            .fold(String::from("|"), |acc, x| format!("{acc} {x} |"));
        println!("{row_spacer}");

        // Print all rows        
        for row in self.table.iter() {
            let row_str = row.values.iter().fold(String::from("|"), |acc, x| format!("{acc} {x} |"));
            println!("{row_str}")
        }

    }


    pub fn handle_insert(&mut self, input: &str) {

        // Expected format:
        // INSERT INTO users VALUES ('Alice', 170.5)

        let input = input.trim();

        if !input.to_uppercase().starts_with("INSERT INTO") {
            println!("Malformed INSERT command");
            return;
        }

        // Very naive split (stub only)
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() < 4 {
            println!("Expected syntax: INSERT INTO <table> VALUES (...)");
            return;
        }

        // We are assuming it is in the correct order
        let table_name = parts[2];
        if self.table.name != table_name {
            println!("Table '{table_name}' not found");
            return;
        }

        // Convert input *eagerly* into typed values
        let values_str = input.split("VALUES").nth(1).unwrap_or("").trim().trim_start_matches('(').trim_end_matches(')');
        if values_str.is_empty() {
            println!("No values provided");
            return;
        }

        let values: Vec<Value> = values_str.split(",").map(|s| eagerly_convert_to_value(s.trim())).collect();


        if let Some(row_id) = self.table.insert(values) {
            println!("Inserted row with id {row_id}");
            return;
        };

        println!("Insert failed: type mismatch or column count mismatch");
    }

}

/// This is obviously a bad way of doing this.
/// TODO: Literally anything else
fn eagerly_convert_to_value(input: &str) -> Value {
    if let Ok(integer) = input.parse::<i64>() {
        return Value::Int(integer);

    } else if let Ok(float) = input.parse::<f64>() {
        return Value::Float(float);

    } else {
        return Value::Text(input.to_string())
    }
}

fn help() {
    println!("Available commands: .help, .exit, .tables, .backend, .schema");
}

fn active_tables<S: Storage>(table: &Table<S>) {
    let table_name = &table.name;
    println!("Active Tables: {table_name}")
}

fn active_backend() {
    println!("Currently using in-memory storage")
}

fn schema<S: Storage>(table: &Table<S>, args: Vec<&str>) {
    // Check arguments exist
    let Some(table_name) = args.first() else {
        println!("Expected format: .schema {{table}}");
        return;
    };
    // Check table specified
    let Some(table_ref) = Some(&table) else {
        println!("Table ({table_name}) not found");
        return;
    };
    let column_text = table_ref.columns.iter()
        .map(|Column{ name, col_type }| format!("{name} <{col_type}>"))
        .fold(String::from("|"), |acc, x| format!("{acc} {x} |"));
    println!("{column_text}")

}

