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
        AstKind::Variable => read_variable(sys, node),
        AstKind::Print => read_print(sys, node),
        AstKind::Plus => read_plus(sys, node),
        AstKind::Minus => read_minus(sys, node),
        AstKind::Mul => read_mul(sys, node),
        AstKind::Div => read_div(sys, node),
        AstKind::EOS => read_eos(sys, node),
        AstKind::Let => read_let(sys, node),
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
        ByteCodeKind::PushConst,
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
        ByteCodeKind::PushConst,
        str_index,
        0,
        0,
    ));
}

fn read_variable(sys: &mut NakoSystem, node: &AstNode) {
    let var_name = node.value.to_string();
    let var_name_index = sys.var_table.get_name_index_create(&var_name);
    sys.codes.push(ByteCode::new(
        ByteCodeKind::PushVariable,
        var_name_index,
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

fn read_let(sys: &mut NakoSystem, node: &AstNode) {
    // Assuming the first child is the variable name and the second child is the value expression
    if let Some(ref children) = node.children {
        if children.len() == 2 {
            let var_name_node = &children[0];
            let var_name = var_name_node.value.to_string();
            let var_index = sys.var_table.get_name_index_create(&var_name);
            let value_node = &children[1];
            // Process the value expression first
            read_ast(sys, value_node);
            // Store the variable
            sys.codes.push(ByteCode::new(
                ByteCodeKind::Let,
                var_index,
                0,
                0,
            ));
            return;
        }
        sys.error("Invalid Let AST node structure");
    }
}