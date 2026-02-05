/// char type module
/// Character type checking functions 
/// 各種文字の分類判定を行うヘルパー関数群

/// ひらがなかどうかを判定する
pub fn is_hiragana(c: char) -> bool {
    // ひらがな判定
    c >= 'ぁ' && c <= 'ゖ'
}
/// カタカナかどうかを判定する
pub fn is_katakana(c: char) -> bool {
    // カタカナ判定
    c >= 'ァ' && c <= 'ヺ'
}
/// 漢字かどうかを判定する
pub fn is_kanji(c: char) -> bool {
    // 漢字判定（CJK統合漢字と拡張Aの一部）
    (c >= '一' && c <= '龥') || (c >= '㐀' && c <= '䶵')
}
/// 日本語の文字（ひらがな・カタカナ・漢字）かを判定する
pub fn is_japanese(c: char) -> bool {
    // 日本語文字の総合判定（ひらがな／カタカナ／漢字）
    is_hiragana(c) || is_katakana(c) || is_kanji(c)
}
/// 英字(A-Z, a-z)かどうかを判定する
pub fn is_alphabet(c: char) -> bool {
    // 英字判定（A-Z, a-z）
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
}
/// 数字(0-9)かどうかを判定する
pub fn is_number(c: char) -> bool {
    // 数字判定（0-9）
    c >= '0' && c <= '9'
}
/// 空白文字（スペース・タブ・CR・LF）かを判定する
pub fn is_whitespace(c: char) -> bool {
    // 空白判定（スペース、タブ、CR、LF）
    c == ' ' || c == '\t' || c == '\r' || c == '\n'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hiragana_checks() {
        assert!(is_hiragana('あ'));
        assert!(!is_hiragana('ア'));
        assert!(!is_hiragana('a'));
    }

    #[test]
    fn katakana_checks() {
        assert!(is_katakana('ア'));
        assert!(!is_katakana('あ'));
    }

    #[test]
    fn kanji_checks() {
        assert!(is_kanji('漢'));
        assert!(!is_kanji('あ'));
    }

    #[test]
    fn japanese_checks() {
        assert!(is_japanese('あ'));
        assert!(is_japanese('ア'));
        assert!(is_japanese('漢'));
        assert!(!is_japanese('a'));
    }

    #[test]
    fn alphabet_checks() {
        assert!(is_alphabet('a'));
        assert!(is_alphabet('Z'));
        assert!(!is_alphabet('あ'));
    }

    #[test]
    fn number_checks() {
        assert!(is_number('0'));
        assert!(is_number('9'));
        assert!(!is_number('a'));
    }

    #[test]
    fn whitespace_checks() {
        assert!(is_whitespace(' '));
        assert!(is_whitespace('\t'));
        assert!(is_whitespace('\n'));
        assert!(!is_whitespace('a'));
    }
}
