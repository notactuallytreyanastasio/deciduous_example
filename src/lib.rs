use wasm_bindgen::prelude::*;

/// Count the number of lines in the given text.
/// A line is defined by the presence of a newline character.
/// This matches the behavior of `wc -l`.
#[wasm_bindgen]
pub fn count_lines(text: &str) -> usize {
    text.chars().filter(|&c| c == '\n').count()
}

/// Count the number of words in the given text.
/// Words are separated by whitespace (spaces, tabs, newlines).
/// This matches the behavior of `wc -w`.
#[wasm_bindgen]
pub fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

/// Count the number of bytes in the given text.
/// This matches the behavior of `wc -c`.
#[wasm_bindgen]
pub fn count_bytes(text: &str) -> usize {
    text.len()
}

/// Get all counts at once (lines, words, bytes).
/// Returns a JS object with lines, words, and bytes fields.
#[wasm_bindgen]
pub fn count_all(text: &str) -> WcResult {
    WcResult {
        lines: count_lines(text),
        words: count_words(text),
        bytes: count_bytes(text),
    }
}

#[wasm_bindgen]
pub struct WcResult {
    pub lines: usize,
    pub words: usize,
    pub bytes: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_lines_empty() {
        assert_eq!(count_lines(""), 0);
    }

    #[test]
    fn test_count_lines_single_line_no_newline() {
        assert_eq!(count_lines("hello"), 0);
    }

    #[test]
    fn test_count_lines_single_line_with_newline() {
        assert_eq!(count_lines("hello\n"), 1);
    }

    #[test]
    fn test_count_lines_multiple() {
        assert_eq!(count_lines("hello\nworld\n"), 2);
    }

    #[test]
    fn test_count_words_empty() {
        assert_eq!(count_words(""), 0);
    }

    #[test]
    fn test_count_words_single() {
        assert_eq!(count_words("hello"), 1);
    }

    #[test]
    fn test_count_words_multiple() {
        assert_eq!(count_words("hello world"), 2);
    }

    #[test]
    fn test_count_words_with_newlines() {
        assert_eq!(count_words("hello\nworld\nfoo bar"), 4);
    }

    #[test]
    fn test_count_words_extra_whitespace() {
        assert_eq!(count_words("  hello   world  "), 2);
    }

    #[test]
    fn test_count_bytes() {
        assert_eq!(count_bytes("hello"), 5);
    }

    #[test]
    fn test_count_all() {
        let result = count_all("hello\nworld\n");
        assert_eq!(result.lines, 2);
        assert_eq!(result.words, 2);
        assert_eq!(result.bytes, 12);
    }
}
