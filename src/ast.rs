/// AST module
/// Defines the Abstract Syntax Tree (AST) structure.

use crate::source::SourcePos;

#[derive(Copy, Clone, Debug)]
pub enum AstKind {
    Nop,
    Comment,
    Node,
    Number,
    String,
    Print,
}

#[derive(Clone, Debug)]
pub struct AstNode {
    pub kind: AstKind,
    pub value_str: Option<String>,
    pub value_num: Option<f64>,
    pub children: Option<Vec<AstNode>>,
    pub pos: SourcePos,
}
impl AstNode {
    pub fn new_nop() -> Self {
        AstNode {
            kind: AstKind::Nop,
            value_str: None,
            value_num: None,
            children: None,
            pos: SourcePos::zero(),
        }
    }
    pub fn new(kind: AstKind) -> Self {
        AstNode {
            kind,
            value_str: None,
            value_num: None,
            children: None,
            pos: SourcePos::zero(),
        }
    }
    pub fn new_pos(kind: AstKind, pos: SourcePos) -> Self {
        AstNode {
            kind,
            value_str: None,
            value_num: None,
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
        println!("{}AstNode: kind={:?}, value_str={:?}, value_num={:?}, pos=({:?})",
            indent_str,
            self.kind,
            self.value_str,
            self.value_num,
            self.pos,
        );
        if let Some(ref children) = self.children {
            for child in children {
                child.print_tree(indent + 1);
            }
        }
    }
}
