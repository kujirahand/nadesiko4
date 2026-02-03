/// parser module
use crate::token::{Token, TokenKind};
use crate::ast::{AstNode, AstKind};

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
    root: AstNode,
    stack: Vec<AstNode>,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            index: 0,
            root: AstNode::new_nop(),
            stack: Vec::new(),
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    pub fn next(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.index);
        if tok.is_some() {
            self.index += 1;
        }
        tok
    }
}


/// Parse the list of tokens into an AST.
pub fn parse(tokens: Vec<Token>) -> AstNode {
    let mut parser = Parser::new(tokens);
    while let Some(token) = parser.next() {
        let pos = token.pos;
        match token.kind {
            TokenKind::Nop => {
                let mut node = AstNode::new_pos(AstKind::Nop, pos);
                node.value_str = Some("NOP".to_string());
                parser.root.add_child(node);
            },
            TokenKind::Comment => {
                let mut node = AstNode::new_pos(AstKind::Nop, pos);
                node.value_str = token.value.clone();
                parser.root.add_child(node);
            },
            TokenKind::Number => {
                let mut node = AstNode::new_pos(AstKind::Number, pos);
                if let Some(ref val_str) = token.value {
                    if let Ok(num) = val_str.parse::<f64>() {
                        node.value_num = Some(num);
                    }
                }
                parser.stack.push(node);
            },
            TokenKind::String => {
                let mut node = AstNode::new_pos(AstKind::Nop, pos);
                node.value_str = token.value.clone();
                parser.stack.push(node);
            },
            TokenKind::Print => {
                let mut node = AstNode::new_pos(AstKind::Print, pos);
                if let Some(arg) = parser.stack.pop() {
                    node.add_child(arg);
                } else {
                    node.add_child(AstNode::new_nop());
                }
                parser.root.add_child(node);
            },
            /*
            _ => {
                // For simplicity, we just create a Nop node for each token.
                let mut node = AstNode::new_pos(AstKind::Nop, pos);
                node.value_str = Some(format!("{:?}", token.kind));
                parser.root.add_child(node);
            }
            */
        }
    }
    return parser.root;
}
