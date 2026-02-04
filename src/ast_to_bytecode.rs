/// ast_to_vmcode module
/// Converts AST nodes to VM code instructions.

use crate::ast::{AstNode, AstKind};
use crate::bytecode::{ByteCodeKind, ByteCode, NakoSystem};
use crate::value::Value;

/// Convert AST to VM code
pub fn ast_to_bytecodes(ast: &AstNode) -> NakoSystem {
    let mut sys = NakoSystem::new();
    read_ast(&mut sys, ast);
    sys
}

/// Read AST nodes recursively and generate VM code
fn read_ast(sys: &mut NakoSystem, node: &AstNode) {
    match node.kind {
        AstKind::Nop | AstKind::Comment => {
            sys.codes.push(ByteCode::new_nop());
        },
        AstKind::Node => {
            read_ast_children(sys, node);
        },
        AstKind::Number => {
            let value = node.value_num.unwrap_or(0.0f64);
            let index = sys.const_list.len();
            sys.const_list.push(Value::from_number(value));
            sys.codes.push(ByteCode::new(
                ByteCodeKind::PushString,
                index,
                0,
                0,
            ));
        },
        AstKind::String => {
            let str_value = node.value_str.clone().unwrap_or("".to_string());
            let str_index = sys.const_list.len();
            sys.const_list.push(Value::from_string(str_value));
            sys.codes.push(ByteCode::new(
                ByteCodeKind::PushString,
                str_index,
                0,
                0,
            ));
        },
        AstKind::Print => {
            read_ast_children(sys, node);
            sys.codes.push(ByteCode::new(ByteCodeKind::Print, 0, 0, 0));
        },
    }
}

/// Read AST children nodes
fn read_ast_children(sys: &mut NakoSystem, node: &AstNode) {
    if let Some(ref children) = node.children {
        for child in children {
            read_ast(sys, child);
        }
    }
}
