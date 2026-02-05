/**
 * Nadesiko4 VM code definitions
 */
use crate::value::Value;
use std::collections::HashMap;

/// VM code type
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ByteCodeKind {
    Nop = 0,
    EOS,
    Print,
    PushConst,
    PushVariable,
    Add,
    Sub,
    Mul,
    Div,
    Let,
}

/// VM code structure
#[derive(Clone, Copy, Debug)]
pub struct ByteCode {
    pub kind: ByteCodeKind,
    pub arg1: usize,
    pub arg2: usize,
    pub arg3: usize,
}
impl ByteCode {
    pub fn new(kind: ByteCodeKind, arg1: usize, arg2: usize, arg3: usize) -> Self {
        ByteCode {
            kind,
            arg1,
            arg2,
            arg3,
        }
    }
    pub fn new_code(kind: ByteCodeKind) -> Self {
        ByteCode::new(kind, 0, 0, 0)
    }
    pub fn new_nop() -> Self {
        ByteCode::new_code(ByteCodeKind::Nop)
    }
}

/// Nako Variable structure
#[derive(Clone, Debug)]
pub struct NakoVar {
    pub name: String,
    pub value: Value,
}
/// Nako Variable Table
#[derive(Clone, Debug)]
pub struct NakoVarTable {
    pub vars: Vec<NakoVar>,
    pub name_map: HashMap<String, usize>,
}
impl NakoVarTable {
    /// Create a new variable table
    pub fn new() -> Self {
        NakoVarTable {
            vars: Vec::new(),
            name_map: HashMap::new(),
        }
    }
    /// Length of variable table
    pub fn len(&self) -> usize {
        self.vars.len()
    }
    pub fn set_by_index(&mut self, index: usize, value: Value) {
        // out of range ... expand
        while index >= self.vars.len() {
            self.vars.push(NakoVar {
                name: "".to_string(),
                value: Value::None,
            });
        }
        // already exists
        if index < self.vars.len() {
            self.vars[index].value = value;
        }
    }
    /// Get variable value
    pub fn get_by_index(&self, index: usize) -> Option<&Value> {
        if index < self.vars.len() {
            Some(&self.vars[index].value)
        } else {
            None
        }
    }
    /// Get variable index, create if not exists
    pub fn get_name_index_create(&mut self, name: &str) -> usize {
        if let Some(&index) = self.name_map.get(name) {
            return index;
        }
        let index = self.vars.len();
        self.vars.push(NakoVar {
            name: name.to_string(),
            value: Value::None,
        });
        self.name_map.insert(name.to_string(), index);
        return index;
    }
    /// Get variable index
    pub fn get_name_index(&self, name: &str) -> Option<usize> {
        self.name_map.get(name).cloned()
    }
}

/// VM code list structure
#[derive(Clone, Debug)]
pub struct NakoSystem {
    pub is_debug: bool,
    pub codes: Vec<ByteCode>,
    pub const_list: Vec<Value>,
    pub stack: Vec<Value>,
    pub var_table: NakoVarTable,
    pub output: String,
    pub error_msg: String,
    pub src_lineno: usize,
}
impl NakoSystem {
    pub fn new() -> Self {
        NakoSystem {
            is_debug: false,
            codes: Vec::new(),
            const_list: Vec::new(),
            var_table: NakoVarTable::new(),
            stack: Vec::new(),
            output: String::new(),
            error_msg: String::new(),
            src_lineno: 0,
        }
    }
    pub fn print(&mut self, msg: &str) {
        self.output.push_str(msg);
    }
    pub fn println(&mut self, msg: &str) {
        self.print(msg);
        self.print("\n");
    }
    pub fn error(&mut self, msg: &str) {
        self.error_msg.push_str(msg);
        self.error_msg.push('\n');
    }
}
