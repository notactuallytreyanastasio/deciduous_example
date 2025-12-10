/// Count the number of lines in the given text.
/// A line is defined by the presence of a newline character.
/// This matches the behavior of `wc -l`.
pub fn count_lines(text: &str) -> usize {
    text.chars().filter(|&c| c == '\n').count()
}

/// Count the number of words in the given text.
/// Words are separated by whitespace (spaces, tabs, newlines).
/// This matches the behavior of `wc -w`.
pub fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
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
}
