use std::fmt::{Debug, Display};

use chrono::{DateTime, Utc};


#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    // Text data types
    Text(String),
    
    // Numeric data types
    Bool(bool),
    Int(i64),
    Float(f64),

    // Date & Time data types
    DateTime(DateTime<Utc>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Text(str) => write!(f, "{str}"),
            Value::Bool(bool) => write!(f, "{bool}"),
            Value::Int(int) => write!(f, "{int}"),
            Value::Float(float) => write!(f, "{float}"),
            Value::DateTime(datetime) => write!(f, "{}", datetime.to_rfc3339()),
        }
    }
}