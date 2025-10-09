use std::io::{self, Write};

use storage::Storage;
use table::Table;


pub fn start_repl<S: Storage>(_table: Table<S>) {
    println!("Starting CLI REPL... (type .exit to quit)");
    let mut input = String::new();

    loop {
        print!("kirin> ");
        io::stdout().flush().unwrap();

        // Read
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // Evaluate & Print
        match input {
            ".exit" => {
                println!("Exiting Kirin CLI.");
                break;
            },
            ".help" => {
                println!("Available commands: .help, .exit");
            }
            _ => {
                println!("You typed: '{}'", input);
                println!("(CLI not fully implemented yet)");
            }
        }
    }
}
