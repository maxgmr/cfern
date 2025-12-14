//! Responsible for consuming and tracking position within the preprocessed C source.

const COMMENT_PAIRS: [(&str, &str); 2] = [("//", "\n"), ("/*", "*/")];

/// Consumes source code whilst tracking position.
#[derive(Debug)]
pub struct SourceStream<'a> {
    source: &'a str,
    position: usize,
}
impl<'a> SourceStream<'a> {
    /// Create a new [`SourceStream`].
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            position: 0,
        }
    }

    /// Consume all whitespace and comments at the current position, advancing the position.
    pub fn consume_whitespace_and_comments(&mut self) {
        loop {
            let whitespace_count = self.count_whitespace();
            self.advance(whitespace_count);

            let comment_count = self.count_comment();
            self.advance(comment_count);

            // Stop when there are no more comments or whitespace to consume
            if (whitespace_count + comment_count) == 0 {
                break;
            }
        }
    }

    /// Advance the position by the given number of chars. Will not advance the position past the
    /// end of the source.
    pub fn advance(&mut self, chars: usize) {
        for c in self.remaining().chars().take(chars) {
            self.position += c.len_utf8();
        }
    }

    /// Get the current byte index within the source.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Get the remaining unprocessed source. Return the empty string if the end of the source has
    /// been reached.
    ///
    /// # Panics
    ///
    /// This function panics if the current position is not at a character boundary.
    pub fn remaining(&self) -> &'a str {
        debug_assert!(self.source.is_char_boundary(self.position));

        if self.is_at_end() {
            return "";
        }

        &self.source[self.position..]
    }

    /// Return true if the end of the source has been reached.
    pub fn is_at_end(&self) -> bool {
        self.position >= self.source.len()
    }

    /// Count whitespace bytes from the current position.
    fn count_whitespace(&self) -> usize {
        let mut count = 0;

        for c in self.remaining().chars() {
            if !c.is_whitespace() {
                break;
            }

            count += c.len_utf8();
        }

        count
    }

    /// Count bytes in a comment from the current position. Return 0 if not a comment.
    fn count_comment(&self) -> usize {
        let remaining = self.remaining();

        for &(start, end) in &COMMENT_PAIRS {
            if !remaining.starts_with(start) {
                continue;
            }

            // Return 0 if no end found (i.e. not a comment)
            return self.count_until(end).map_or(0, |count| count + end.len());
        }

        0
    }

    /// Count bytes until the given pattern is found (not including the pattern itself). Return
    /// [`None`] if the given pattern isn't found.
    fn count_until(&self, pattern: &str) -> Option<usize> {
        let mut remaining = self.remaining();
        let mut count = 0;

        while !remaining.is_empty() {
            if remaining.starts_with(pattern) {
                return Some(count);
            }
            let next_char_size = match remaining.chars().next() {
                Some(c) => c.len_utf8(),
                None => {
                    return None;
                }
            };
            count += next_char_size;
            remaining = &remaining[next_char_size..];
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stream_basic_advance() {
        let mut ss = SourceStream::new("Hello, world!");
        assert_eq!(ss.position(), 0);
        assert_eq!(ss.remaining(), "Hello, world!");
        assert!(!ss.is_at_end());

        ss.advance(5);
        assert_eq!(ss.position(), 5);
        assert_eq!(ss.remaining(), ", world!");
        assert!(!ss.is_at_end());

        ss.advance(7);
        assert_eq!(ss.position(), 12);
        assert_eq!(ss.remaining(), "!");
        assert!(!ss.is_at_end());

        ss.advance(1);
        assert_eq!(ss.position(), 13);
        assert_eq!(ss.remaining(), "");
        assert!(ss.is_at_end());
    }

    #[test]
    fn stream_advance_past_end() {
        let mut ss = SourceStream::new("Hello, world!");

        ss.advance(12);
        assert_eq!(ss.position(), 12);
        assert_eq!(ss.remaining(), "!");
        assert!(!ss.is_at_end());

        ss.advance(1);
        assert_eq!(ss.position(), 13);
        assert_eq!(ss.remaining(), "");
        assert!(ss.is_at_end());

        ss.advance(1);
        assert_eq!(ss.position(), 13);
        assert_eq!(ss.remaining(), "");
        assert!(ss.is_at_end());

        ss.advance(5);
        assert_eq!(ss.position(), 13);
        assert_eq!(ss.remaining(), "");
        assert!(ss.is_at_end());
    }

    #[test]
    fn stream_advance_multibyte() {
        let mut ss = SourceStream::new("你好");

        ss.advance(1);
        assert_eq!(ss.position(), 3);
        assert_eq!(ss.remaining(), "好");
        assert!(!ss.is_at_end());

        ss.advance(1);
        assert_eq!(ss.position(), 6);
        assert_eq!(ss.remaining(), "");
        assert!(ss.is_at_end());

        ss.advance(1);
        assert_eq!(ss.position(), 6);
        assert_eq!(ss.remaining(), "");
        assert!(ss.is_at_end());
    }

    #[test]
    fn stream_consume_whitespace() {
        let mut ss = SourceStream::new("   \t\n  hello");

        ss.consume_whitespace_and_comments();
        assert_eq!(ss.position(), 7);
        assert_eq!(ss.remaining(), "hello");
        assert!(!ss.is_at_end());
    }

    #[test]
    fn stream_consume_line_comment() {
        let mut ss = SourceStream::new("// comment\nhello");

        ss.consume_whitespace_and_comments();
        assert_eq!(ss.position(), 11);
        assert_eq!(ss.remaining(), "hello");
        assert!(!ss.is_at_end());
    }

    #[test]
    fn stream_consume_block_comment() {
        let mut ss =
            SourceStream::new("/* \n * This is a block\n * comment\n */\n#include <stdio.h>\n");

        ss.consume_whitespace_and_comments();
        assert_eq!(ss.position(), 38);
        assert_eq!(ss.remaining(), "#include <stdio.h>\n");
        assert!(!ss.is_at_end());
    }

    #[test]
    fn stream_consume_nested_block_comment() {
        // You can't nest block comments in C!
        let mut ss = SourceStream::new("/* \n * Nested\n * /* \n *  * comment\n *  */\n */");

        ss.consume_whitespace_and_comments();
        assert_eq!(ss.position(), 43);
        assert_eq!(ss.remaining(), "*/");
        assert!(!ss.is_at_end());
    }

    #[test]
    fn stream_block_comment_no_end() {
        let mut ss = SourceStream::new("/* \n * not a comment\n * \n");

        ss.consume_whitespace_and_comments();
        assert_eq!(ss.position(), 0);
        assert_eq!(ss.remaining(), "/* \n * not a comment\n * \n");
        assert!(!ss.is_at_end());
    }
}
