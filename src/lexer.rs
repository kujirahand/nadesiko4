use crate::source::Source;

/// lexer module
use crate::token::{Token, TokenKind};

// Lexer implementation
pub fn lex(src: &mut Source) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    while let Some(ch) = src.peek() {
        let pos = src.get_position();
        match ch {
            ' ' | '\t' | '\r' | '\n' => {
                src.next();
            }
            '#' => {
                // Comment
                let comment = src.get_token('\n');
                tokens.push(Token::new(TokenKind::Comment, Some(comment), pos));
            }
            '0'..='9' => {
                // Number
                let mut number = String::new();
                while let Some(c) = src.peek() {
                    if c.is_digit(10) {
                        number.push(c);
                        src.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::new(TokenKind::Number, Some(number), pos));
            }
            '"' => {
                // String literal
                let str_tok = get_string_literal(src, '"', '"');
                tokens.push(str_tok);
            }
            '「' => {
                // String literal
                let str_tok = get_string_literal(src, '「', '」');
                tokens.push(str_tok);
            }
            _ => {
                // Other symbols
                let symbol = ch.to_string();
                src.next();
                tokens.push(Token::new(TokenKind::Nop, Some(symbol), pos));
            }
        }
    }
    
    tokens
}

/// Helper function to extract string literals
fn get_string_literal(src: &mut Source, bos: char, eos: char) -> Token {
    let mut literal = String::new();
    if let Some(c) = src.peek() {
        if c == bos {
            src.next(); // consume opening quote
        }
    }

    while let Some(next_ch) = src.next() {
        if next_ch == eos {
            break;
        }
        if next_ch == '\\' {
            // Handle escape sequences
            if let Some(esc_ch) = src.next() {
                match esc_ch {
                    'n' => literal.push('\n'),
                    't' => literal.push('\t'),
                    '\\' => literal.push('\\'),
                    '"' => literal.push('"'),
                    _ => literal.push(esc_ch),
                }
                continue;
            }
        }
        literal.push(next_ch);
    }
    Token::new(TokenKind::String, Some(literal), src.get_position())
}
