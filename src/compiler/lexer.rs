use std::fmt::Display;

use crate::compiler::token::{Token, get_next_token};

use color_eyre::{
    Section, SectionExt,
    eyre::{OptionExt, eyre},
    owo_colors::OwoColorize,
};

const COMMENT_PAIRS: [(&str, &str); 2] = [("//", "\n"), ("/*", "*/")];

/// Lex a string of valid C code into a list of [`Token`]s.
pub fn lex(_data: &str) -> color_eyre::Result<Vec<Token>> {
    todo!()
}

struct Lexer<'a> {
    index: usize,
    line_num: usize,
    original: &'a str,
    remaining: &'a str,
}
impl<'a> Lexer<'a> {
    fn new(data: &'a str) -> Self {
        Self {
            index: 0,
            line_num: 0,
            original: data,
            remaining: data,
        }
    }

    /// Return an error showing where in the file the error occurred.
    fn lexing_error<S: AsRef<str> + Display>(
        &self,
        message: S,
    ) -> color_eyre::Result<Option<Token<'_>>> {
        let consumed_chars = self.original.len() - self.remaining.len();
        let consumed_lines: Vec<&str> = self.original[..consumed_chars].split("\n").collect();
        let line_num = consumed_lines.len();
        let consumed_line = consumed_lines.last().ok_or_eyre("no consumed lines")?;
        let col_num = consumed_line.len() + 1;
        let remaining_line: &str = self.remaining.split("\n").next().unwrap_or("");
        Err(eyre!("{}", message))
            .with_section(|| format!("{}:{}", line_num.blue(), col_num.blue()).header("Line info:"))
            .with_section(|| {
                format!(
                    "{}{}\n{}{}",
                    consumed_line,
                    remaining_line,
                    (0..(col_num - 1)).map(|_| " ").collect::<String>(),
                    "^ Here".bright_red().bold()
                )
            })
    }
}

fn count_whitespace(data: &str) -> usize {
    count_applicable(data, |c| c.is_whitespace())
}

fn count_comment(data: &str) -> usize {
    for &(start, end) in &COMMENT_PAIRS {
        if !data.starts_with(start) {
            continue;
        }

        return count_to(data, end)
            .map(|count| count + end.len())
            .unwrap_or_default();
    }

    0
}

/// Counts the number of bytes up to the given pattern. Returns [`None`] if no match.
fn count_to(mut data: &str, pattern: &str) -> Option<usize> {
    let mut index = 0;
    while !data.is_empty() {
        if data.starts_with(pattern) {
            return Some(index);
        }
        let next_char_size = data.chars().next().unwrap().len_utf8();
        index += next_char_size;
        data = &data[next_char_size..];
    }

    None
}

// Counts bytes in the given string until a char that doesn't meet the given predicate is found.
fn count_applicable<F>(data: &str, mut predicate: F) -> usize
where
    F: FnMut(char) -> bool,
{
    let mut index = 0;

    for c in data.chars() {
        if !predicate(c) {
            break;
        }
        index += c.len_utf8();
    }

    index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_whitespace_simple() {
        assert_eq!(count_whitespace("   \n\r\t\t"), 7);
    }

    #[test]
    fn count_whitespace_none() {
        assert_eq!(count_whitespace("1  "), 0);
    }

    #[test]
    fn count_whitespace_non_ascii() {
        // U+205F: medium mathematical space
        assert_eq!(count_whitespace("\t\u{205F} "), 5);
    }

    #[test]
    fn count_comment_oneline() {
        let data = "// This is a comment\n//thisisanother\n#include <stdio.h>\n";
        assert_eq!(count_comment(data), 21);
    }

    #[test]
    fn count_comment_online_no_newline() {
        let data = "// This is a comment with no newline";
        assert_eq!(count_comment(data), 0);
    }

    #[test]
    fn count_comment_multiline() {
        let data = "/* \n * This is a multiline\n * comment\n */\n#include <stdio.h>\n";
        assert_eq!(count_comment(data), 41);
    }

    #[test]
    fn count_comment_multiline_nested() {
        let data = "/* \n * Nested\n * /* \n *  * comment\n *  */\n */";
        assert_eq!(count_comment(data), 41);
    }

    #[test]
    fn count_comment_multiline_no_end() {
        let data = "/* \n * not a comment\n * \n";
        assert_eq!(count_comment(data), 0);
    }

    #[test]
    fn count_comment_none() {
        let data = "int hello = 1; // this is the start of the comment";
        assert_eq!(count_comment(data), 0);
    }

    #[test]
    fn count_applicable_simple() {
        assert_eq!(count_applicable("hello there", |c| c.is_alphabetic()), 5);
    }

    #[test]
    fn count_applicable_case() {
        assert_eq!(
            count_applicable("four score and Seven years ago", |c| !c
                .is_ascii_uppercase()),
            15
        );
    }

    #[test]
    fn count_applicable_multibyte() {
        assert_eq!(count_applicable("我是Max", |c| !c.is_ascii()), 6);
    }

    #[test]
    fn count_applicable_end() {
        let data = "Hullo! Nice to meet you";
        assert_eq!(count_applicable(data, |c| c.is_ascii()), data.len());
    }

    #[test]
    fn count_applicable_empty() {
        assert_eq!(count_applicable("", |_| true), 0);
    }

    #[test]
    fn count_to_simple() {
        assert_eq!(count_to("hello there\nnice to meet you!", "to"), Some(17));
    }

    #[test]
    fn count_to_start() {
        assert_eq!(count_to("match immediately", "match"), Some(0));
    }

    #[test]
    fn count_to_multibyte_chars() {
        assert_eq!(
            count_to("你好\n我是马克斯\nhello\nI'm Max", "hello"),
            Some(23)
        );
    }

    #[test]
    fn count_to_multiple_matches() {
        assert_eq!(count_to("i am sam i am", "am"), Some(2));
    }

    #[test]
    fn count_to_no_match() {
        assert_eq!(count_to("om nom nom", "am"), None);
    }

    #[test]
    fn count_to_empty_data() {
        assert_eq!(count_to("", "blah"), None);
    }

    #[test]
    fn count_to_empty_pattern() {
        assert_eq!(count_to("this is a test", ""), Some(0));
    }

    #[test]
    fn count_to_both_empty() {
        assert_eq!(count_to("", ""), None);
    }
}
