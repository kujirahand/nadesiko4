/// ast_to_vmcode module
/// Converts AST nodes to VM code instructions.

use crate::ast::{AstNode, AstKind};
use crate::vmcode::{VmcodeKind, Vmcode, VmArg, VmSystem};

/// Convert AST to VM code
pub fn ast_to_vmcode(ast: &AstNode) -> VmSystem {
    let mut sys = VmSystem::new();
    read_ast(&mut sys, ast);
    sys
}

/// Read AST nodes recursively and generate VM code
fn read_ast(sys: &mut VmSystem, node: &AstNode) {
    match node.kind {
        AstKind::Nop => {
            sys.codes.push(Vmcode {
                kind: VmcodeKind::Nop,
                arg: VmArg::I(0),
            });
        },
        AstKind::Node => {
            if let Some(ref children) = node.children {
                for child in children {
                    read_ast(sys, child);
                }
            }
        },
        AstKind::Number => {
            // Handle number nodes if needed
        },
        AstKind::String => {
            let value = node.value_str.clone().unwrap_or_default();
            let str_index = sys.str_list.len();
            sys.str_list.push(value);
            sys.codes.push(Vmcode {
                kind: VmcodeKind::PushString,
                arg: VmArg::P(str_index),
            });
        },
        AstKind::Print => {
            sys.codes.push(Vmcode {
                kind: VmcodeKind::Print,
                arg: VmArg::P(0),
            });
        },
        _ => {}
    }
}
