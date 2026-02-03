/// value module
/// 

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    None,
    Number(f64),
    String(String),
    Array(Vec<Value>),
}
