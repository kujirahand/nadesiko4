/**
 * Nadesiko4 VM code definitions
 */
use crate::value::Value;

/// VM code type
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ByteCodeKind {
    Nop = 0,
    Print = 1,
    PushString = 2,
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

/// VM code list structure
#[derive(Clone, Debug)]
pub struct NakoSystem {
    pub codes: Vec<ByteCode>,
    pub const_list: Vec<Value>,
    pub stack: Vec<Value>,
    pub output: String,
    pub error_msg: String,
}
impl NakoSystem {
    pub fn new() -> Self {
        NakoSystem {
            codes: Vec::new(),
            const_list: Vec::new(),
            stack: Vec::new(),
            output: String::new(),
            error_msg: String::new(),
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
