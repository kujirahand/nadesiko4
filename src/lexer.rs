use crate::source::Source;

/// lexer module
use crate::token::{Token, TokenKind};
use crate::char_type::{is_hiragana, is_japanese, is_kanji};

static JOSI1: [char; 6] = ['と', 'は', 'が', 'を', 'に', 'で'];

// Lexer implementation
pub fn lex(src: &mut Source) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    while let Some(ch) = src.peek() {
        println!("ch: {:?}", ch);
        let pos = src.get_position();
        match ch {
            ' ' | '\t' | '\r' => { // whitespace
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
            'a'..='z' | 'A'..='Z' => {
                // Alphabetic word
                let mut word = String::new();
                while let Some(c) = src.peek() {
                    if c.is_alphabetic() {
                        word.push(c);
                        src.next();
                    } else {
                        break;
                    }
                }
                let mut tok = Token::new(TokenKind::Word, Some(word), pos);
                tok.josi = get_josi(src, true);
                tokens.push(tok);
            }
            '"' => {
                let mut tok = get_string_literal(src, '"', '"');
                tok.josi = get_josi(src, true);
                tokens.push(tok);
            }
            '「' => {
                let mut tok = get_string_literal(src, '「', '」');
                tok.josi = get_josi(src, true);
                tokens.push(tok);
            }
            '。' | ';' | '\n' => {
                // Statement terminator
                let symbol = ch.to_string();
                src.next();
                tokens.push(Token::new(TokenKind::EOS, Some(symbol), pos));
            }
            _ if is_kanji(ch) => {
                let tok = match get_word(src) {
                    Some(mut t) => {
                        if t.value_is("表示") {
                            t.kind = TokenKind::Print;
                        }
                        t
                    },
                    None => continue,
                };
                tokens.push(tok);
            }
            _ => {
                // Other symbols
                println!("未知の文字: {}", ch);
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
    let pos = src.get_position();
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
    Token::new(TokenKind::String, Some(literal), pos)
}

/// Check and extract josi (particles)
fn get_josi(src: &mut Source, is_test: bool) -> Option<String> {
    let mut josi = String::new();
    if let Some(c) = src.peek() {
        if JOSI1.contains(&c) {
            josi.push(c);
            if is_test {
                src.next();
                return Some(josi);
            }
        }
    }
    return None;
}


fn get_word(src: &mut Source) -> Option<Token> {
    //　日本語？
    let pos = src.get_position();
    let mut first_char = '\0';
    if let Some(c) = src.peek() {
        first_char = c;
        if !is_japanese(c) {
            return None;
        }
    }
    let mut word = String::new();
    let mut josi: Option<String> = None;
    if is_kanji(first_char) {
        // 漢字 + 送り仮名 + 助詞
        while let Some(c) = src.peek() {
            if is_kanji(c) {
                word.push(c);
                src.next();
                continue;
            }
            // 助詞?
            if is_hiragana(c) {
                let josi2 = get_josi(src, true);
                if josi2.is_none() {
                    word.push(c);
                    src.next();
                    continue;
                }
                josi = josi2;
            }
            break;
        }
        // 助詞あり
        let mut tok = Token::new(
            TokenKind::Word, 
            Some(word),
            pos,
        );
        tok.josi = josi;
        return Some(tok);
    } else if is_hiragana(first_char) {
        // ひらがな + 助詞
        while let Some(c) = src.next() {
            // 助詞?
            let josi = get_josi(src, true);
            if josi.is_none() {
                word.push(c);
                continue;
            }
            // 助詞あり
            let mut tok = Token::new(
                TokenKind::Word, 
                Some(word),
                pos,
            );
            tok.josi = josi;
            return Some(tok);
        }
    }
    None
}
