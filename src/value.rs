//! value module

/// Defines the Value enum used in the VM stack and constants.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    None,
    Number(f64),
    String(String),
    Array(Vec<Value>),
}
impl Value {
    /// Check if the value is None
    pub fn is_none(&self) -> bool {
        matches!(self, Value::None)
    }
    /// Create a Value from a &str
    pub fn from_str(s: &str) -> Self {
        Value::String(s.to_string())
    }
    /// Create a Value from a String
    pub fn from_string(s: String) -> Self {
        Value::String(s)
    }
    /// Create a Value from a f64 number
    pub fn from_number(n: f64) -> Self {
        Value::Number(n)
    }
    /// Convert the Value to a String representation
    pub fn to_string(&self) -> String {
        match self {
            Value::None => "None".to_string(),
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
            Value::Array(arr) => {
                let mut s = "[".to_string();
                let mut first = true;
                for v in arr {
                    if !first {
                        s.push_str(", ");
                    }
                    s.push_str(&v.to_string());
                    first = false;
                }
                s.push(']');
                s
            }
        }
    }
    /// Convert the Value to a number (f64) if possible
    pub fn to_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            Value::String(s) => s.parse::<f64>().ok(),
            _ => None,
        }
    }
}