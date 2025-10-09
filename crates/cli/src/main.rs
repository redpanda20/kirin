mod repl;

use storage::MemoryStorage;
use repl::start_repl;
use table::Table;

fn main() {

    // Opens a new storage
    let storage = MemoryStorage::new();
    
    println!("Kirin DB initialized with MemoryStorage.");

    // Opens a new table
    let table = Table::new("empty", Vec::new(), storage);


    // Begin the Read-Evaluate-Print loop
    // (Yes, I forget what REPL means)
    start_repl(table);
}
