//! Virtual Machine module
use crate::bytecode::{ByteCode, ByteCodeKind, NakoSystem};

/// Run the VM with the given VM system
pub fn run(sys: &mut NakoSystem) -> bool {
    let mut pc: usize = 0;
    let code_len = sys.codes.len();

    while pc < code_len {
        // Copy out the current instruction to avoid borrowing sys while executing
        let code = sys.codes[pc];
        let result = match code.kind {
            ByteCodeKind::Nop => exec_nop(sys, &code),
            ByteCodeKind::EOS => exec_eos(sys, &code),
            ByteCodeKind::PushConst => exec_push_const(sys, &code),
            ByteCodeKind::PushVariable => exec_push_variable(sys, &code),
            ByteCodeKind::Print => exec_print(sys, &code),
            ByteCodeKind::Add => exec_add(sys, &code),
            ByteCodeKind::Sub => exec_sub(sys, &code),
            ByteCodeKind::Mul => exec_mul(sys, &code),
            ByteCodeKind::Div => exec_div(sys, &code),
            ByteCodeKind::Let => exec_let(sys, &code),
        };
        
        if !result {
            return false;
        }
        
        pc += 1;
    }

    true
}

fn exec_nop(_sys: &mut NakoSystem, _code: &ByteCode) -> bool {
    // Do nothing
    true
}

fn exec_eos(sys: &mut NakoSystem, code: &ByteCode) -> bool {
    sys.src_lineno = code.arg1;
    true
}

fn exec_let(sys: &mut NakoSystem, code: &ByteCode) -> bool {
    let var_index = code.arg1;
    if let Some(value) = sys.stack.pop() {
        if sys.is_debug {
            println!("LET: var_index={}, value={:?}", var_index, value);
        }
        sys.var_table.set_by_index(var_index, value);
    } else {
        sys.error("Stack underflow on LET operation");
        return false;
    }
    true
}

fn exec_push_const(sys: &mut NakoSystem, code: &ByteCode) -> bool {
    if code.arg1 < sys.const_list.len() {
        let value = sys.const_list[code.arg1].clone();
        if sys.is_debug {
            println!("PUSH_CONST: {:?}", value);
        }
        sys.stack.push(value);
        true
    } else {
        sys.error(&format!("Invalid constant index: {}", code.arg1));
        false
    }
}

fn exec_push_variable(sys: &mut NakoSystem, code: &ByteCode) -> bool {
    let var_index = code.arg1;
    if var_index < sys.var_table.len() {
        if let Some(value) = sys.var_table.get_by_index(var_index) {
            sys.stack.push(value.clone());
            return true;
        }
    }
    sys.error(&format!("Invalid constant index: {}", code.arg1));
    false
}

fn exec_print(sys: &mut NakoSystem, _code: &ByteCode) -> bool {
    if let Some(value) = sys.stack.pop() {
        println!("[PRINT]{:?}", value);
        sys.println(&value.to_string());
        true
    } else {
        sys.error("Stack underflow on PRINT operation");
        false
    }
}

fn exec_add(sys: &mut NakoSystem, _code: &ByteCode) -> bool {
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

fn exec_sub(sys: &mut NakoSystem, _code: &ByteCode) -> bool {
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

fn exec_mul(sys: &mut NakoSystem, _code: &ByteCode) -> bool {
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

fn exec_div(sys: &mut NakoSystem, _code: &ByteCode) -> bool {
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