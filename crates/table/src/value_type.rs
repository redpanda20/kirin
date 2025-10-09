use storage::Value;

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