use crate::source::Source;

/// lexer module
use crate::token::{Token, TokenKind};
use crate::char_type::{is_hiragana, is_kanji, is_katakana};

static JOSI2: [&str; 5] = ["から", "まで", "から", "には", "とは"];
static JOSI1: [char; 8] = ['と', 'は', 'が', 'を', 'に', 'で', 'へ', 'の'];

// Lexer implementation
pub fn lex(src: &mut Source) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    while let Some(ch) = src.peek() {
        // println!("ch: {:?}", ch);
        match ch {
            ' ' | '\t' | '\r' => { src.next(); }, // skip whitespace
            '#' => lex_comment(src, &mut tokens),
            '0'..='9' => lex_number(src, &mut tokens),
            'a'..='z' | 'A'..='Z' | '_' => lex_alphabetic_word(src, &mut tokens),
            '"' => lex_string(src, &mut tokens, '"', '"'),
            '「' => lex_string(src, &mut tokens, '「', '」'),
            '。' | ';' | '\n' => lex_eos(src, &mut tokens, ch),
            '+' | '＋' => tokens.push(get_operator(src, '+', TokenKind::Plus)),
            '-' | '−' => tokens.push(get_operator(src, '-', TokenKind::Minus)),
            '*' | '＊' | '×' => tokens.push(get_operator(src, '*', TokenKind::Mul)),
            '/' | '÷' => tokens.push(get_operator(src, '/', TokenKind::Div)),
            '（' | '(' => tokens.push(get_operator(src, '(', TokenKind::ParenL)),
            '）' | ')' => tokens.push(get_operator(src, ')', TokenKind::ParenR)),
            _ if is_japanese_word(ch) => lex_japanese_word(src, &mut tokens),
            _ => lex_unknown(src, &mut tokens, ch),
        }
    }
    
    tokens
}

fn is_japanese_word(c: char) -> bool {
    is_kanji(c) || is_hiragana(c) || is_katakana(c)
}

fn lex_comment(src: &mut Source, tokens: &mut Vec<Token>) {
    let pos = src.get_position();
    let comment = src.get_token('\n');
    tokens.push(Token::new(TokenKind::Comment, Some(comment), pos));
}

fn lex_number(src: &mut Source, tokens: &mut Vec<Token>) {
    let pos = src.get_position();
    let mut number = String::new();
    while let Some(c) = src.peek() {
        if c.is_digit(10) {
            number.push(c);
            src.next();
        } else {
            break;
        }
    }
    let mut tok = Token::new(TokenKind::Number, Some(number), pos);
    tok.josi = get_josi(src);
    tokens.push(tok);
}

fn lex_alphabetic_word(src: &mut Source, tokens: &mut Vec<Token>) {
    let pos = src.get_position();
    let mut word = String::new();
    while let Some(c) = src.peek() {
        if c.is_alphabetic() || c == '_' {
            word.push(c);
            src.next();
        } else {
            break;
        }
    }
    let mut tok = Token::new(TokenKind::Word, Some(word), pos);
    tok.josi = get_josi(src);
    tokens.push(tok);
}

fn lex_string(src: &mut Source, tokens: &mut Vec<Token>, bos: char, eos: char) {
    let mut tok = get_string_literal(src, bos, eos);
    tok.josi = get_josi(src);
    tokens.push(tok);
}

fn lex_eos(src: &mut Source, tokens: &mut Vec<Token>, ch: char) {
    let pos = src.get_position();
    let symbol = ch.to_string();
    src.next();
    tokens.push(Token::new(TokenKind::EOS, Some(symbol), pos));
}

fn lex_japanese_word(src: &mut Source, tokens: &mut Vec<Token>) {
    let mut tok = match get_word(src) {
        Some(t) => t,
        None => return,
    };
    if tok.value_is("表示") {
        tok.kind = TokenKind::Print;
    }
    tokens.push(tok);
}

fn lex_unknown(src: &mut Source, tokens: &mut Vec<Token>, ch: char) {
    let pos = src.get_position();
    println!("未知の文字: {}", ch);
    let symbol = ch.to_string();
    src.next();
    tokens.push(Token::new(TokenKind::Nop, Some(symbol), pos));
}

fn get_operator(src: &mut Source, op_char: char, kind: TokenKind) -> Token {
    let pos = src.get_position();
    src.next(); // consume operator
    Token::new(kind, Some(op_char.to_string()), pos)
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
    Token::new(TokenKind::Str, Some(literal), pos)
}

/// Check and extract josi (particles)
fn get_josi(src: &mut Source) -> Option<String> {
    let josi_len = is_josi(src);
    if josi_len > 0 {
        return Some(src.get_n(josi_len));
    }
    None
}

/// 助詞チェック - 助詞だったらその長さを返す
fn is_josi(src: &mut Source) -> usize {
    // 2char
    for josi in JOSI2.iter() {
        if src.test_string(josi) {
            return 2;
        }
    }
    // 1char
    if let Some(c) = src.peek() {
        if JOSI1.contains(&c) {
            return 1;
        }
    }
    0
}

fn get_word(src: &mut Source) -> Option<Token> {
    let first_char = src.peek().unwrap_or('\0');
    match first_char {
        _ if is_kanji(first_char) => get_word_kanji(src),
        _ if is_hiragana(first_char) => get_word_hiragana(src),
        _ if is_katakana(first_char) => get_word_katakana(src),
        _ => None,
    }
}

fn get_word_kanji(src: &mut Source) -> Option<Token> {
    let pos = src.get_position();
    let mut word = String::new();
    // 漢字(送り仮名|漢字)+ + 助詞
    while src.has_more() {
        // 漢字
        while let Some(c) = src.peek() {
            if !is_kanji(c) { break; }
            word.push(c);
            src.next();
        }
        // 送り仮名
        while let Some(c) = src.peek() {
            if !is_hiragana(c) { break; }
            if is_josi(src) > 0 {
                break;
            }
            // 送り仮名は省略 --- word.push(c);
            src.next();
        }
        if src.is_kanji() { continue; }
        break;
    }
    if !word.is_empty() {
        let mut tok = Token::new(
            TokenKind::Word, 
            Some(word),
            pos,
        );
        tok.josi = get_josi(src);
        return Some(tok);
    }
    None
}

fn get_word_hiragana(src: &mut Source) -> Option<Token> {
    let pos = src.get_position();
    let mut word = String::new();
    // ひらがな + 助詞
    while let Some(c) = src.peek() {
        if !is_hiragana(c) {
            break;
        }
        if is_josi(src) == 0 {
            word.push(c);
            src.next();
        } else {
            break;
        }
    }
    if !word.is_empty() {
        let mut tok = Token::new(
            TokenKind::Word, 
            Some(word),
            pos,
        );
        tok.josi = get_josi(src);
        return Some(tok);
    }
    None
}

fn get_word_katakana(src: &mut Source) -> Option<Token> {
    let pos = src.get_position();
    let mut word = String::new();
    // カタカナ + 送り仮名 + 助詞
    // カタカナ
    while let Some(c) = src.peek() {
        if !is_katakana(c) { break; }
        word.push(c);
        src.next();
    }
    // 送り仮名
    while let Some(c) = src.peek() {
        if !is_hiragana(c) { break; }
        if is_josi(src) > 0 { break; }
        // 送り仮名は省略 --- word.push(c);
        src.next();
    }
    if !word.is_empty() {
        let mut tok = Token::new(
            TokenKind::Word, 
            Some(word),
            pos,
        );
        tok.josi = get_josi(src);
        return Some(tok);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_word(input: &str, expected_value: &str, expected_josi: Option<&str>) {
        let mut src = Source::new(input);
        let tok = get_word(&mut src).expect("expected word token");
        assert_eq!(tok.kind, TokenKind::Word);
        assert_eq!(tok.value.as_deref(), Some(expected_value));
        assert_eq!(tok.josi.as_deref(), expected_josi);
        assert_eq!((tok.pos.line, tok.pos.column), (0, 0));
    }

    #[test]
    fn kanji_word_with_josi() {
        assert_word("猫は", "猫", Some("は"));
        assert_word("価格の", "価格", Some("の"));
        assert_word("価格から税率を", "価格", Some("から"));
        assert_word("逃げ切るは", "逃切", Some("は"));
    }

    #[test]
    fn kanji_word_with_okurigana_without_josi() {
        assert_word("表示する", "表示", None);
    }

    #[test]
    fn hiragana_word_with_josi() {
        assert_word("あいに行く", "あい", Some("に"));
    }

    #[test]
    fn katakana_word_with_josi() {
        assert_word("カタカナで", "カタカナ", Some("で"));
    }

    #[test]
    fn non_japanese_input_returns_none() {
        let mut src = Source::new("abc");
        assert!(get_word(&mut src).is_none());
    }

    fn assert_lex(input: &str, expected_kinds: Vec<TokenKind>) {
        let mut src = Source::new(input);
        let tokens = lex(&mut src);
        let kinds: Vec<TokenKind> = tokens.iter().map(|t| t.kind).collect();
        assert_eq!(kinds, expected_kinds);
    }

    #[test]
    fn test_lex_number() {
        assert_lex("1と12を", vec![
            TokenKind::Number,
            TokenKind::Number,
        ]);
    }
}


