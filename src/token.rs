/// Token module
/// Defines the Token struct used by the lexer.

use crate::source::SourcePos;

/// Different kinds of tokens.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TokenKind {
    Nop,
    Comment,
    Number,
    String,
    Print,
}

/// Token structure
#[derive(Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Option<String>,
    pub pos: SourcePos,
}
impl Token {
    /// Create a new token
    pub fn new(kind: TokenKind, value: Option<String>, pos: SourcePos) -> Self {
        Token {
            kind,
            value,
            pos: pos,
        }
    }
}
