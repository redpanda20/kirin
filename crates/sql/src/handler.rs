use core::{Column, Value};
use std::io::{Result, Write};

use database::Database;

type Writer<'a> = &'a mut dyn Write;

pub fn handle_select<'a>(db: &mut Database, writer: Writer, input: &str) -> Result<()> {

    // Expected format:
    // "SELECT * FROM <table>"

    let input = input.trim();

    let input_upper = input.to_uppercase();
    if !input_upper.starts_with("SELECT") {
        return writeln!(writer, "Malformed SELECT command")
    }

    // Very naive split (stub only)
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() < 4 {
        return writeln!(writer, "Expected syntax: SELECT * FROM <table>")
    }

    // Check for FROM keyword
    if parts.get(2).unwrap().trim() != "FROM" {
        return writeln!(writer, "Missing FROM clause")
    }

    // Check table name matches
    let table_name = *parts.get(3).unwrap_or(&"");
    let Some(table) = db.get_table(table_name.to_string()) else {
        return writeln!(writer, "Table '{table_name}' not found")
    };

    // Print table information
    let row_count = table.iter().count();
    writeln!(writer, "({row_count} rows)")?;

    // Print schema
    let column_schema = table.columns.iter()
        .map(|Column{ name, col_type }| format!("{name} <{col_type}>"))
        .fold(String::from("|"), |acc, x| format!("{acc} {x} |"));
    writeln!(writer, "{column_schema}")?;

    // Print spacer
    let row_spacer = table.columns.iter()
        .map(|_| String::from(" --- "))
        .fold(String::from("|"), |acc, x| format!("{acc} {x} |"));
    writeln!(writer, "{row_spacer}")?;

    // Print all rows        
    for row in table.iter() {
        let row_str = row.values.iter()
        .fold(String::from("|"), |acc, x| format!("{acc} {x} |"));
        writeln!(writer, "{row_str}")?
    }

    Ok(())
}

    pub fn handle_insert(db: &mut Database, writer: Writer, input: &str) -> Result<()> {

        // Expected format:
        // INSERT INTO users VALUES ('Alice', 170.5)

        let input = input.trim();

        if !input.to_uppercase().starts_with("INSERT INTO") {
            return writeln!(writer, "Malformed INSERT command")
        }

        // Very naive split (stub only)
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() < 4 {
            return writeln!(writer, "Expected syntax: INSERT INTO <table> VALUES (...)")
        }

        // We are assuming it is in the correct order
        let table_name = parts[2];
        let Some(table) = db.get_table(table_name.to_string()) else {
            return writeln!(writer, "Table '{table_name}' not found")
        };


        // Convert input *eagerly* into typed values
        let values_str = input.split("VALUES").nth(1).unwrap_or("").trim().trim_start_matches('(').trim_end_matches(')');
        if values_str.is_empty() {
            return writeln!(writer, "No values provided")
        }

        let values: Vec<Value> = values_str.split(",").map(|s| eagerly_convert_to_value(s.trim())).collect();


        if let Some(row_id) = table.insert(values) {
            return writeln!(writer, "Inserted row with id {row_id}")
        };

        writeln!(writer, "Insert failed: type mismatch or column count mismatch")?;

        Ok(())
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