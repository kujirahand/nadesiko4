/// Token module
/// Defines the Token struct used by the lexer.

use std::fmt;

use crate::source::SourcePos;

/// Different kinds of tokens.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TokenKind {
    Nop,
    Comment,
    EOS,
    Number,
    Str,
    Word,
    Print,
    Plus,
    Minus,
    Mul,
    Div,
    ParenL,
    ParenR,
}
impl TokenKind {
    pub fn is_operator(&self) -> bool {
        matches!(self,
            TokenKind::Plus | TokenKind::Minus |
            TokenKind::Mul | TokenKind::Div
        )
    }
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
    /// Create a new token with josi
    pub fn new_arg(kind: TokenKind, value: &str, josi: &str, pos: SourcePos) -> Self {
        Token {
            kind,
            value: Some(value.to_string()),
            pos,
            josi: Some(josi.to_string()),
        }
    }
    /// Create a new token
    pub fn new(kind: TokenKind, value: Option<String>, pos: SourcePos) -> Self {
        Self { kind, value, pos, josi: None }
    }
    pub fn new_nop(pos: SourcePos) -> Self {
        Self::new(TokenKind::Nop, None, pos)
    }
    pub fn value_is(&self, s: &str) -> bool {
        if let Some(ref val) = self.value {
            val == s
        } else {
            false
        }
    }
}
/// Human-friendly display of tokens for logging/debug output.
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.value.as_deref().unwrap_or("").replace("\n", "¶");
        let pos = self.pos;
        if let Some(josi) = &self.josi {
            write!(f, "{:?}({}){}@{}:{}", self.kind, value, josi, pos.line, pos.column)
        } else {
            write!(f, "{:?}({})@{}:{}", self.kind, value, pos.line, pos.column)
        }
    }
}
