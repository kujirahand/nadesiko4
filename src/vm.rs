//! Virtual Machine module
use crate::bytecode::{NakoSystem, ByteCodeKind};

/// Run the VM with the given VM system
pub fn run(sys: &mut NakoSystem) -> bool {
    let mut pc: usize = 0;
    let code_len = sys.codes.len();

    while pc < code_len {
        let code = &sys.codes[pc];
        let result = match code.kind {
            ByteCodeKind::Nop => exec_nop(sys),
            ByteCodeKind::PushString => exec_push_string(sys, code.arg1),
            ByteCodeKind::Print => exec_print(sys),
            ByteCodeKind::Add => exec_add(sys),
            ByteCodeKind::Sub => exec_sub(sys),
            ByteCodeKind::Mul => exec_mul(sys),
            ByteCodeKind::Div => exec_div(sys),
        };
        
        if !result {
            return false;
        }
        
        pc += 1;
    }

    true
}

fn exec_nop(_sys: &mut NakoSystem) -> bool {
    // Do nothing
    true
}

fn exec_push_string(sys: &mut NakoSystem, const_index: usize) -> bool {
    if const_index < sys.const_list.len() {
        let value = sys.const_list[const_index].clone();
        sys.stack.push(value);
        true
    } else {
        sys.error(&format!("Invalid constant index: {}", const_index));
        false
    }
}

fn exec_print(sys: &mut NakoSystem) -> bool {
    if let Some(value) = sys.stack.pop() {
        println!("[PRINT]{:?}", value);
        sys.println(&value.to_string());
        true
    } else {
        sys.error("Stack underflow on PRINT operation");
        false
    }
}

fn exec_add(sys: &mut NakoSystem) -> bool {
    if let (Some(right), Some(left)) = (sys.stack.pop(), sys.stack.pop()) {
        if let (Some(l), Some(r)) = (left.to_number(), right.to_number()) {
            sys.stack.push(crate::value::Value::from_number(l + r));
            true
        } else {
            sys.error("ADD operation requires numeric values");
            false
        }
    } else {
        sys.error("Stack underflow on ADD operation");
        false
    }
}

fn exec_sub(sys: &mut NakoSystem) -> bool {
    if let (Some(right), Some(left)) = (sys.stack.pop(), sys.stack.pop()) {
        if let (Some(l), Some(r)) = (left.to_number(), right.to_number()) {
            sys.stack.push(crate::value::Value::from_number(l - r));
            true
        } else {
            sys.error("SUB operation requires numeric values");
            false
        }
    } else {
        sys.error("Stack underflow on SUB operation");
        false
    }
}

fn exec_mul(sys: &mut NakoSystem) -> bool {
    if let (Some(right), Some(left)) = (sys.stack.pop(), sys.stack.pop()) {
        if let (Some(l), Some(r)) = (left.to_number(), right.to_number()) {
            sys.stack.push(crate::value::Value::from_number(l * r));
            true
        } else {
            sys.error("MUL operation requires numeric values");
            false
        }
    } else {
        sys.error("Stack underflow on MUL operation");
        false
    }
}

fn exec_div(sys: &mut NakoSystem) -> bool {
    if let (Some(right), Some(left)) = (sys.stack.pop(), sys.stack.pop()) {
        if let (Some(l), Some(r)) = (left.to_number(), right.to_number()) {
            if r == 0.0 {
                sys.error("Division by zero");
                return false;
            }
            sys.stack.push(crate::value::Value::from_number(l / r));
            true
        } else {
            sys.error("DIV operation requires numeric values");
            false
        }
    } else {
        sys.error("Stack underflow on DIV operation");
        false
    }
}