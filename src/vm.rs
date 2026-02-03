/// Virtual Machine module
use crate::vmcode::{VmSystem, VmcodeKind, VmArg};

/// Run the VM with the given VM system
pub fn run(sys: &mut VmSystem) -> bool {
    let mut pc: usize = 0;
    let code_len = sys.codes.len();

    while pc < code_len {
        let code = &sys.codes[pc];
        match code.kind {
            VmcodeKind::Nop => {
                // Do nothing
            },
            VmcodeKind::PushString => {
            },
            VmcodeKind::Print => {
                if let VmArg::P(str_index) = code.arg {
                    if let Some(s) = sys.str_list.get(str_index) {
                        println!("{}", s);
                    }
                }
            }
        }
        pc += 1;
    }

    true
}