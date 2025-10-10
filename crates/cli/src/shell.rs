use std::io::{stdin, Result, Write};

use storage::{MemoryStorage, Value};
use table::{Column, Table};

pub struct Shell<'a> {
    table: Table<MemoryStorage>,
    writer: &'a mut dyn Write,
}

impl <'a> Shell<'a> {

    pub fn new(table: Table<MemoryStorage>, writer: &'a mut dyn Write) -> Self {
        Self { table, writer }
    }

    pub fn run(&mut self) -> Result<()> {
        writeln!(self.writer, "Starting CLI REPL... (type .exit to quit)")?;

        loop {
            write!(self.writer,"kirin> ")?;
            self.writer.flush()?;

            // Read
            let mut line = String::new();
            stdin().read_line(&mut line)?;

            match line.trim() {
                ".exit" => break,

                cmd if cmd.starts_with(".") => self.execute_command(cmd)?,

                cmd if cmd.starts_with("SELECT") => self.handle_select(cmd)?,

                cmd if cmd.starts_with("INSERT") => self.handle_insert(cmd)?,

                _ => (),
            }
        }

        writeln!(self.writer, "Exiting Kirin CLI.")?;
        Ok(())
    }

    pub fn execute_command(&mut self, input: &str) -> Result<()> {
        let input = &mut input.trim().split_ascii_whitespace();

        let command = input.next().unwrap_or("");
        let args: Vec<_> = input.collect();

        // Evaluate & Print
        match (command, args) {
            (".help", _) => {
                writeln!(self.writer, "Available commands: .help, .exit, .tables, .backend, .schema")
            },
            (".tables", _) => {
                let table_name = &self.table.name;
                writeln!(self.writer, "Active Tables: {table_name}")
            },
            (".backend", _) => {
                writeln!(self.writer, "Currently using in-memory storage")
            },
            (".schema", args) => {
                // Check arguments exist
                let Some(table_name) = args.first() else {
                    return writeln!(self.writer, "Expected format: .schema {{table}}")
                };

                // Check table specified
                let Some(table_ref) = Some(&self.table) else {
                    return writeln!(self.writer, "Table ({table_name}) not found")
                };

                let column_text = table_ref.columns.iter()
                    .map(|Column{ name, col_type }| format!("{name} <{col_type}>"))
                    .fold(String::from("|"), |acc, x| format!("{acc} {x} |"));
                writeln!(self.writer, "{column_text}")

            },
            _ => {
                writeln!(self.writer, "Unknown command, type .help")
            },
        }
    }

    pub fn handle_select(&mut self, input: &str) -> Result<()> {

        // Expected format:
        // "SELECT * FROM <table>"

        let input = input.trim();
    
        let input_upper = input.to_uppercase();
        if !input_upper.starts_with("SELECT") {
            return writeln!(self.writer, "Malformed SELECT command")
        }

        // Very naive split (stub only)
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() < 4 {
            return writeln!(self.writer, "Expected syntax: SELECT * FROM <table>")
        }

        // Check for FROM keyword
        if parts.get(2).unwrap().trim() != "FROM" {
            return writeln!(self.writer, "Missing FROM clause")
        }

        // Check table name matches
        let table_name = *parts.get(3).unwrap_or(&"");
        if table_name != self.table.name {
            return writeln!(self.writer, "Table '{}' not found", table_name)
        }

        // Print table information
        let row_count = self.table.iter().count();
        writeln!(self.writer, "({row_count} rows)")?;

        // Print schema
        let column_schema = self.table.columns.iter()
            .map(|Column{ name, col_type }| format!("{name} <{col_type}>"))
            .fold(String::from("|"), |acc, x| format!("{acc} {x} |"));
        writeln!(self.writer, "{column_schema}")?;

        // Print spacer
        let row_spacer = self.table.columns.iter()
            .map(|_| String::from(" --- "))
            .fold(String::from("|"), |acc, x| format!("{acc} {x} |"));
        writeln!(self.writer, "{row_spacer}")?;

        // Print all rows        
        for row in self.table.iter() {
            let row_str = row.values.iter().fold(String::from("|"), |acc, x| format!("{acc} {x} |"));
            writeln!(self.writer, "{row_str}")?
        }

        Ok(())
    }


    pub fn handle_insert(&mut self, input: &str) -> Result<()> {

        // Expected format:
        // INSERT INTO users VALUES ('Alice', 170.5)

        let input = input.trim();

        if !input.to_uppercase().starts_with("INSERT INTO") {
            return writeln!(self.writer, "Malformed INSERT command")
        }

        // Very naive split (stub only)
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() < 4 {
            return writeln!(self.writer, "Expected syntax: INSERT INTO <table> VALUES (...)")
        }

        // We are assuming it is in the correct order
        let table_name = parts[2];
        if self.table.name != table_name {
            return writeln!(self.writer, "Table '{table_name}' not found")
        }

        // Convert input *eagerly* into typed values
        let values_str = input.split("VALUES").nth(1).unwrap_or("").trim().trim_start_matches('(').trim_end_matches(')');
        if values_str.is_empty() {
            return writeln!(self.writer, "No values provided")
        }

        let values: Vec<Value> = values_str.split(",").map(|s| eagerly_convert_to_value(s.trim())).collect();


        if let Some(row_id) = self.table.insert(values) {
            return writeln!(self.writer, "Inserted row with id {row_id}")
        };

        writeln!(self.writer, "Insert failed: type mismatch or column count mismatch")?;

        Ok(())
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