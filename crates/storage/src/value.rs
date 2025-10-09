use std::time::Instant;


#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    // Text data types
    Text(String),
    
    // Numeric data types
    Bool(bool),
    Int(i64),
    Float(f64),

    // Date & Time data types
    DateTime(Instant),
}