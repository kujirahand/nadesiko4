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
        AstKind::Nop => read_nop(sys, node),
        AstKind::Comment => read_comment(sys, node),
        AstKind::Node => read_node(sys, node),
        AstKind::Number => read_number(sys, node),
        AstKind::String => read_string(sys, node),
        AstKind::Print => read_print(sys, node),
        AstKind::Plus => read_plus(sys, node),
        AstKind::Minus => read_minus(sys, node),
        AstKind::Mul => read_mul(sys, node),
        AstKind::Div => read_div(sys, node),
        AstKind::EOS => read_eos(sys, node),
    }
}

fn read_nop(sys: &mut NakoSystem, _node: &AstNode) {
    sys.codes.push(ByteCode::new_nop());
}

fn read_comment(sys: &mut NakoSystem, _node: &AstNode) {
    sys.codes.push(ByteCode::new_nop());
}

fn read_node(sys: &mut NakoSystem, node: &AstNode) {
    read_ast_children(sys, node);
}

fn read_number(sys: &mut NakoSystem, node: &AstNode) {
    let value = if let Some(num) = node.value.to_number() {
        num
    } else {
        0.0f64
    };
    let index = sys.const_list.len();
    sys.const_list.push(Value::from_number(value));
    sys.codes.push(ByteCode::new(
        ByteCodeKind::PushString,
        index,
        0,
        0,
    ));
}

fn read_string(sys: &mut NakoSystem, node: &AstNode) {
    let str_value = node.value.to_string();
    let str_index = sys.const_list.len();
    sys.const_list.push(Value::from_string(str_value));
    sys.codes.push(ByteCode::new(
        ByteCodeKind::PushString,
        str_index,
        0,
        0,
    ));
}

fn read_print(sys: &mut NakoSystem, node: &AstNode) {
    read_ast_children(sys, node);
    sys.codes.push(ByteCode::new(ByteCodeKind::Print, 0, 0, 0));
}

fn read_plus(sys: &mut NakoSystem, node: &AstNode) {
    read_ast_children(sys, node);
    sys.codes.push(ByteCode::new(ByteCodeKind::Add, 0, 0, 0));
}

fn read_minus(sys: &mut NakoSystem, node: &AstNode) {
    read_ast_children(sys, node);
    sys.codes.push(ByteCode::new(ByteCodeKind::Sub, 0, 0, 0));
}

fn read_mul(sys: &mut NakoSystem, node: &AstNode) {
    read_ast_children(sys, node);
    sys.codes.push(ByteCode::new(ByteCodeKind::Mul, 0, 0, 0));
}

fn read_div(sys: &mut NakoSystem, node: &AstNode) {
    read_ast_children(sys, node);
    sys.codes.push(ByteCode::new(ByteCodeKind::Div, 0, 0, 0));
}

fn read_eos(sys: &mut NakoSystem, node: &AstNode) {
    sys.codes.push(ByteCode::new(
        ByteCodeKind::EOS,
        node.pos.line,
        0,
        0,
    ));
}

/// Read AST children nodes
fn read_ast_children(sys: &mut NakoSystem, node: &AstNode) {
    if let Some(ref children) = node.children {
        for child in children {
            read_ast(sys, child);
        }
    }
}
