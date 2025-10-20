use database::Database;
use core::Column;

use std::io::{stdin, Result, Write};

pub struct Shell<'a> {
    db: Database,
    writer: &'a mut dyn Write,
}

impl <'a> Shell<'a> {

    pub fn new(db: Database, writer: &'a mut dyn Write) -> Self {
        Self { db, writer }
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
                let tables = &self.db.get_table_names().fold(String::new(), |acc, s| acc + " " + s);
                writeln!(self.writer, "Active Tables: {tables}")
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
                let Some(table) = self.db.get_table(table_name.to_string()) else {
                    return writeln!(self.writer, "Table ({table_name}) not found")
                };

                let column_text = table.columns.iter()
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
        sql::handler::handle_select(&mut self.db, self.writer, input)
    }


    pub fn handle_insert(&mut self, input: &str) -> Result<()> {
        sql::handler::handle_insert(&mut self.db, self.writer, input)
    }


}
