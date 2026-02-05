/// AST module
/// Defines the Abstract Syntax Tree (AST) structure.

use crate::source::SourcePos;
use crate::value::Value;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AstKind {
    Nop,
    Comment,
    Node,
    Number,
    String,
    Print,
    Plus,
    Minus,
    Mul,
    Div,
}

#[derive(Clone, Debug)]
pub struct AstNode {
    pub kind: AstKind,
    pub value: Value,
    pub children: Option<Vec<AstNode>>,
    pub pos: SourcePos,
}
impl AstNode {
    pub fn new_nop() -> Self {
        AstNode {
            kind: AstKind::Nop,
            value: Value::None,
            children: None,
            pos: SourcePos::zero(),
        }
    }
    pub fn new(kind: AstKind) -> Self {
        AstNode {
            kind,
            value: Value::None,
            children: None,
            pos: SourcePos::zero(),
        }
    }
    pub fn new_pos(kind: AstKind, pos: SourcePos) -> Self {
        AstNode {
            kind,
            value: Value::None,
            children: None,
            pos,
        }
    }
    pub fn add_child(&mut self, child: AstNode) {
        if self.children.is_none() {
            self.children = Some(Vec::new());
        }
        if let Some(ref mut children) = self.children {
            children.push(child);
        }
    }
    pub fn print_tree(&self, indent: usize) {
        let indent_str = "  ".repeat(indent);
        println!("{}AstNode: kind={:?}, value={:?}, pos=({:?})",
            indent_str,
            self.kind,
            self.value,
            self.pos,
        );
        if let Some(ref children) = self.children {
            for child in children {
                child.print_tree(indent + 1);
            }
        }
    }
}
