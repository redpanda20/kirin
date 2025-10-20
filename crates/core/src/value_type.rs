use std::fmt::Display;

use crate::Value;

#[derive(Debug, PartialEq)]
pub enum ValueType {
    // Text data types
    Text,
    
    // Numeric data types
    Bool,
    Int,
    Float,

    // Date & Time data types
    DateTime,
}

impl From<&Value> for ValueType {
    fn from(value: &Value) -> Self {
        match value {
            Value::Text(_) => ValueType::Text,
            Value::Bool(_) => ValueType::Bool,
            Value::Int(_) => ValueType::Int,
            Value::Float(_) => ValueType::Float,
            Value::DateTime(_) => ValueType::DateTime,
        }
    }
}

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            ValueType::Text => "Text",
            ValueType::Bool => "Boolean",
            ValueType::Int => "Integer",
            ValueType::Float => "Float",
            ValueType::DateTime => "Date/Time",
        };
        f.write_str(text)
    }
}