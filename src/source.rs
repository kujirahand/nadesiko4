/// source code character cursor module
use crate::char_type;

#[derive(Copy, Clone, Debug)]
pub struct SourcePos {
    pub line: usize,
    pub column: usize,
}
impl SourcePos {
    pub fn new(line: usize, column: usize) -> Self {
        SourcePos { line, column }
    }
    pub fn to_touple(&self) -> (usize, usize) {
        (self.line, self.column)
    }
    pub fn zero() -> Self {
        SourcePos { line: 0, column: 0 }
    }
}

#[derive(Clone)]
pub struct Source {
    pub source: Vec<char>,
    start_pos: SourcePos,
    index: usize,
    pos: SourcePos,
}

impl Source {
    /// Create a new CharCursor
    pub fn new(source_str: &str) -> Self {
        Source {
            source: source_str.chars().collect(),
            index: 0,
            pos: SourcePos::zero(),
            start_pos: SourcePos::zero(),
        }
    }
    /// Create a new CharCursor with a given starting position
    pub fn new_with_posision(source_str: &str, start_pos: SourcePos) -> Self {
        Source {
            source: source_str.chars().collect(),
            index: 0,
            pos: SourcePos::new(1, 1),
            start_pos,
        }
    }
    /// Get the current character
    pub fn get_position(&self) -> SourcePos {
        SourcePos::new(
            self.pos.line + self.start_pos.line,
            self.pos.column + self.start_pos.column,
        )
    }
    /// Get the current character
    pub fn get_pos_tuple(&self) -> (usize, usize) {
        let pos = self.get_position();
        (pos.line, pos.column)
    }

    /// Get the current character
    pub fn peek(&self) -> Option<char> {
        if self.index < self.source.len() {
            Some(self.source[self.index])
        } else {
            None
        }
    }
    /// Get the next character and advance the cursor
    pub fn next(&mut self) -> Option<char> {
        if self.index < self.source.len() {
            let ch = self.source[self.index];
            self.index += 1;
            if ch == '\n' {
                self.pos.line += 1;
                self.pos.column = 1;
            } else {
                self.pos.column += 1;
            }
            Some(ch)
        } else {
            None
        }
    }
    /// Advance the cursor by n characters
    pub fn next_n(&mut self, n: usize) {
        for _ in 0..n {
            self.next();
        }
    }
    /// prev the cursor
    pub fn prev(&mut self) {
        if self.index > 0 {
            self.index -= 1;
            let ch = self.source[self.index];
            if ch == '\n' {
                if self.pos.line > 1 {
                    self.pos.line -= 1;
                }
                // Note: column reset is not handled here
            } else {
                if self.pos.column > 1 {
                    self.pos.column -= 1;
                }
            }
        }
    }
    /// Check if the cursor has reached the end of the source
    pub fn is_eof(&self) -> bool {
        self.index >= self.source.len()
    }
    /// Check if the cursor has reached the end of the source
    pub fn has_more(&self) -> bool {
        self.index < self.source.len()
    }
    /// Reset the cursor to the beginning
    pub fn reset(&mut self) {
        self.index = 0;
        self.pos = SourcePos::zero();
    }
    /// Get the length of the source
    pub fn len(&self) -> usize {
        self.source.len()
    }
    /// Get the token
    pub fn get_token(&mut self, end_of_token: char) -> String {
        let mut token = String::new();
        while let Some(ch) = self.peek() {
            if ch == end_of_token {
                // Consume the delimiter but keep position at the end of the token
                self.next();
                if self.pos.column > 0 {
                    self.pos.column -= 1;
                }
                break;
            }
            token.push(ch);
            self.next();
        }
        token
    }
    /// Get the substring from current to the given length
    pub fn peek_n(&self, length: usize) -> String {
        let mut substring = String::new();
        let end_index = usize::min(self.index + length, self.source.len());
        for i in self.index..end_index {
            substring.push(self.source[i]);
        }
        substring
    }
    /// Get the substring from current to the given length
    pub fn get_n(&mut self, length: usize) -> String {
        let mut remain = length;
        let mut substring = String::new();
        while let Some(ch) = self.next() {
            substring.push(ch);
            remain -= 1;
            if remain <= 0 { break; }
        }
        substring
    }
    /// Skip whitespace characters
    pub fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.next();
            } else {
                break;
            }
        }
    }
    /// Test character at current position
    pub fn test_char(&self, test_char: char) -> bool {
        if let Some(ch) = self.peek() {
            ch == test_char
        } else {
            false
        }
    }
    /// Test string at current position
    pub fn test_string(&self, test_str: &str) -> bool {
        let test_chars: Vec<char> = test_str.chars().collect();
        for (i, &test_ch) in test_chars.iter().enumerate() {
            if let Some(ch) = self.source.get(self.index + i) {
                if *ch != test_ch {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    /// Check if current character is kanji
    pub fn is_kanji(&self) -> bool {
        if let Some(c) = self.peek() {
            char_type::is_kanji(c)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Source};

    #[test]
    fn next_and_positions_progress() {
        let mut cur = Source::new("ab");
        assert_eq!(cur.get_pos_tuple(), (0, 0));
        assert_eq!(cur.peek(), Some('a'));

        assert_eq!(cur.next(), Some('a'));
        assert_eq!(cur.get_pos_tuple(), (0, 1));

        assert_eq!(cur.next(), Some('b'));
        assert_eq!(cur.get_pos_tuple(), (0, 2));
        assert!(cur.is_eof());
    }

    #[test]
    fn newline_updates_line_and_column() {
        let mut cur = Source::new("a\nb");
        cur.next();
        assert_eq!(cur.get_pos_tuple(), (0, 1));

        cur.next(); // '\n'
        assert_eq!(cur.get_pos_tuple(), (1, 1));

        cur.next();
        assert_eq!(cur.get_pos_tuple(), (1, 2));
    }

    #[test]
    fn prev_moves_backwards() {
        let mut cur = Source::new("ab");
        cur.next();
        cur.next();
        assert_eq!(cur.get_pos_tuple(), (0, 2));

        cur.prev();
        assert_eq!(cur.get_pos_tuple(), (0, 1));
        assert_eq!(cur.peek(), Some('b'));

        cur.prev();
        assert_eq!(cur.get_pos_tuple(), (0, 1));
        assert_eq!(cur.peek(), Some('a'));
    }

    #[test]
    fn get_token_stops_at_delimiter() {
        let mut cur = Source::new("hello,world");
        let tok = cur.get_token(',');
        assert_eq!(tok, "hello");
        assert_eq!(cur.get_pos_tuple(), (0, 5));
        assert_eq!(cur.peek(), Some('w'));
    }

    #[test]
    fn get_substring_clamps_length() {
        let mut cur = Source::new("abc");
        assert_eq!(cur.peek_n(2), "ab");
        cur.next();
        assert_eq!(cur.peek_n(5), "bc");
    }

    #[test]
    fn get_substring_clamps_length_multibyte() {
        let mut cur = Source::new("あいう");
        assert_eq!(cur.peek_n(2), "あい");
        cur.next();
        assert_eq!(cur.peek_n(5), "いう");
    }

    #[test]
    fn get_n_test() {
        let mut cur = Source::new("あいうえお");
        assert_eq!(cur.get_n(2), "あい");
        assert_eq!(cur.get_n(5), "うえお");
    }

    #[test]
    fn get_n_and_prev_test() {
        let mut cur = Source::new("あいうえお");
        assert_eq!(cur.get_n(1), "あ");
        assert_eq!(cur.get_n(2), "いう");
        cur.prev();
        assert_eq!(cur.get_n(2), "うえ");
    }

    #[test]
    fn get_substring_with_zero_length() {
        let cur = Source::new("abc");
        assert_eq!(cur.peek_n(0), "");
    }

    #[test]
    fn get_substring_exact_remaining() {
        let mut cur = Source::new("abc");
        cur.next(); // at 'b'
        assert_eq!(cur.peek_n(2), "bc");
    }

    #[test]
    fn get_substring_from_end() {
        let mut cur = Source::new("abc");
        cur.next();
        cur.next();
        cur.next(); // at EOF
        assert_eq!(cur.peek_n(1), "");
    }

    #[test]
    fn get_substring_with_japanese() {
        let cur = Source::new("あいうえお");
        assert_eq!(cur.peek_n(2), "あい");
        assert_eq!(cur.peek_n(5), "あいうえお");
    }

    #[test]
    fn get_substring_empty_source() {
        let cur = Source::new("");
        assert_eq!(cur.peek_n(5), "");
    }

    #[test]
    fn skip_whitespace_advances_to_content() {
        let mut cur = Source::new("  \tabc");
        cur.skip_whitespace();
        assert_eq!(cur.get_pos_tuple(), (0, 3));
        assert_eq!(cur.peek(), Some('a'));
    }

    #[test]
    fn reset_moves_back_to_start() {
        let mut cur = Source::new("abc");
        cur.next();
        cur.next();
        cur.reset();
        assert_eq!(cur.get_pos_tuple(), (0, 0));
        assert_eq!(cur.peek(), Some('a'));
    }
}

