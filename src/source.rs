/// source code character cursor module

#[derive(Copy, Clone)]
pub struct SourcePos {
    pub line: usize,
    pub column: usize,
}

#[derive(Clone)]
pub struct Source {
    pub source: Vec<char>,
    start_pos: SourcePos,
    index: usize,
    pos: SourcePos,
}
impl SourcePos {
    pub fn new(line: usize, column: usize) -> Self {
        SourcePos { line, column }
    }
    pub fn zero() -> Self {
        SourcePos { line: 0, column: 0 }
    }
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
        while let Some(ch) = self.next() {
            if ch == end_of_token {
                break;
            }
            token.push(ch);
        }
        token
    }
    /// Get the substring from current to the given length
    pub fn get_substring(&self, length: usize) -> String {
        let mut substring = String::new();
        let end_index = usize::min(self.index + length, self.source.len());
        for i in self.index..end_index {
            substring.push(self.source[i]);
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
}

#[cfg(test)]
mod tests {
    use super::{Source, SourcePos};

    fn pos_tuple(pos: SourcePos) -> (usize, usize) {
        (pos.line, pos.column)
    }

    #[test]
    fn next_and_positions_progress() {
        let mut cur = Source::new("ab");
        assert_eq!(pos_tuple(cur.get_position()), (0, 0));
        assert_eq!(cur.peek(), Some('a'));

        assert_eq!(cur.next(), Some('a'));
        assert_eq!(pos_tuple(cur.get_position()), (0, 1));

        assert_eq!(cur.next(), Some('b'));
        assert_eq!(pos_tuple(cur.get_position()), (0, 2));
        assert!(cur.is_eof());
    }

    #[test]
    fn newline_updates_line_and_column() {
        let mut cur = Source::new("a\nb");
        cur.next();
        assert_eq!(pos_tuple(cur.get_position()), (0, 1));

        cur.next(); // '\n'
        assert_eq!(pos_tuple(cur.get_position()), (1, 1));

        cur.next();
        assert_eq!(pos_tuple(cur.get_position()), (1, 2));
    }

    #[test]
    fn prev_moves_backwards() {
        let mut cur = Source::new("ab");
        cur.next();
        cur.next();
        assert_eq!(pos_tuple(cur.get_position()), (0, 2));

        cur.prev();
        assert_eq!(pos_tuple(cur.get_position()), (0, 1));
        assert_eq!(cur.peek(), Some('b'));

        cur.prev();
        assert_eq!(pos_tuple(cur.get_position()), (0, 1));
        assert_eq!(cur.peek(), Some('a'));
    }

    #[test]
    fn get_token_stops_at_delimiter() {
        let mut cur = Source::new("hello,world");
        let tok = cur.get_token(',');
        assert_eq!(tok, "hello");
        assert_eq!(pos_tuple(cur.get_position()), (0, 5));
        assert_eq!(cur.peek(), Some('w'));
    }

    #[test]
    fn get_substring_clamps_length() {
        let mut cur = Source::new("abc");
        assert_eq!(cur.get_substring(2), "ab");
        cur.next();
        assert_eq!(cur.get_substring(5), "bc");
    }

    #[test]
    fn skip_whitespace_advances_to_content() {
        let mut cur = Source::new("  \tabc");
        cur.skip_whitespace();
        assert_eq!(pos_tuple(cur.get_position()), (0, 3));
        assert_eq!(cur.peek(), Some('a'));
    }

    #[test]
    fn reset_moves_back_to_start() {
        let mut cur = Source::new("abc");
        cur.next();
        cur.next();
        cur.reset();
        assert_eq!(pos_tuple(cur.get_position()), (0, 0));
        assert_eq!(cur.peek(), Some('a'));
    }
}

