/// Token module
/// Defines the Token struct used by the lexer.

use crate::source::SourcePos;

/// Different kinds of tokens.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TokenKind {
    Nop,
    Comment,
    EOS,
    Number,
    String,
    Word,
    Print,
}

/// Token structure
#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Option<String>,
    pub pos: SourcePos,
    pub josi: Option<String>,
}
impl Token {
    /// Create a new token
    pub fn new(kind: TokenKind, value: Option<String>, pos: SourcePos) -> Self {
        Token {
            kind,
            value,
            pos: pos,
            josi: None,
        }
    }
    pub fn new_nop(pos: SourcePos) -> Self {
        Token {
            kind: TokenKind::Nop,
            value: None,
            pos: pos,
            josi: None,
        }
    }
    pub fn value_is(&self, s: &str) -> bool {
        if let Some(ref val) = self.value {
            val == s
        } else {
            false
        }
    }
}
