use std::{fmt::{Debug, Display}, time::Instant};


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

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Text(str) => Display::fmt(str, f),
            Value::Bool(bool) => Display::fmt(bool, f),
            Value::Int(int) => Display::fmt(int, f),
            Value::Float(float) => Display::fmt(float, f),
            Value::DateTime(_time) => f.write_str("Date Time"),
        }
    }

}