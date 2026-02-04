/// value module
/// 

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    None,
    Number(f64),
    String(String),
    Array(Vec<Value>),
}
impl Value {
    pub fn is_none(&self) -> bool {
        matches!(self, Value::None)
    }
    pub fn from_str(s: &str) -> Self {
        Value::String(s.to_string())
    }
    pub fn from_string(s: String) -> Self {
        Value::String(s)
    }
    pub fn from_number(n: f64) -> Self {
        Value::Number(n)
    }
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
}