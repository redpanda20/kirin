mod repl;

use storage::{MemoryStorage};
use repl::start_repl;

fn main() {

    // Opens a new storage
    let storage = MemoryStorage::new();

    println!("Kirin DB initialized with MemoryStorage.");

    // Begin the Read-Evaluate-Print loop
    // (Yes, I forget what REPL means)
    start_repl(storage);
}
