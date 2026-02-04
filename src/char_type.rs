/// char type module
/// Character type checking functions
pub fn is_hiragana(c: char) -> bool {
    (c >= 'ぁ' && c <= 'ゖ') || (c >= 'ァ' && c <= 'ヺ')
}
pub fn is_katakana(c: char) -> bool {
    c >= 'ァ' && c <= 'ヺ'
}
pub fn is_kanji(c: char) -> bool {
    (c >= '一' && c <= '龥') || (c >= '㐀' && c <= '䶵')
}
pub fn is_japanese(c: char) -> bool {
    is_hiragana(c) || is_katakana(c) || is_kanji(c)
}
pub fn is_alphabet(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
}
pub fn is_number(c: char) -> bool {
    c >= '0' && c <= '9'
}
pub fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\r' || c == '\n'
}
