/// Virtual Machine module
use crate::bytecode::{NakoSystem, ByteCodeKind};

/// Run the VM with the given VM system
pub fn run(sys: &mut NakoSystem) -> bool {
    let mut pc: usize = 0;
    let code_len = sys.codes.len();

    while pc < code_len {
        let code = &sys.codes[pc];
        match code.kind {
            ByteCodeKind::Nop => {
                // Do nothing
            },
            ByteCodeKind::PushString => {
                let const_index = code.arg1;
                if const_index < sys.const_list.len() {
                    let value = sys.const_list[const_index].clone();
                    sys.stack.push(value);
                } else {
                    sys.error(&format!("Invalid constant index: {}", const_index));
                    return false;
                }
            },
            ByteCodeKind::Print => {
                if let Some(value) = sys.stack.pop() {
                    println!("[PRINT]{:?}", value);
                    sys.println(&value.to_string());
                } else {
                    sys.error("Stack underflow on PRINT operation");
                    return false;
                }
            }
        }
        pc += 1;
    }

    true
}