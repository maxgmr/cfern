use std::fmt::Display;

use crate::compiler::token::{
    Keyword, Symbol, Token, TokenKind, match_constant, match_identifier, match_keyword,
};

use color_eyre::{
    Section, SectionExt,
    eyre::{OptionExt, eyre},
    owo_colors::OwoColorize,
};
use strum::IntoEnumIterator;

const COMMENT_PAIRS: [(&str, &str); 2] = [("//", "\n"), ("/*", "*/")];

/// Lex a string of valid C code into a list of [`Token`]s.
pub fn lex<'a>(data: &'a str) -> color_eyre::Result<Vec<Token<'a>>> {
    let mut tokens = Vec::new();
    let mut index = 0;

    while let Some(token) = consume_next_token(data, &mut index)? {
        tokens.push(token);
    }

    Ok(tokens)
}

/// Return the next token, moving the index accordingly. Returns [`None`] if done lexing.
/// Returns an error if unable to match token.
fn consume_next_token<'a>(
    data: &'a str,
    index: &mut usize,
) -> color_eyre::Result<Option<Token<'a>>> {
    let mut remaining = get_remaining(data, *index);

    consume_whitespace_comments(&mut remaining, index);

    if remaining.is_empty() {
        return Ok(None);
    }

    let mut token = Token::create_placeholder();

    // Try to match keyword
    token.try_update(remaining, *index, match_keyword, |s| {
        Keyword::try_from(s).ok().map(TokenKind::Keyword)
    });
    // Try to match ident
    token.try_update(remaining, *index, match_identifier, |s| {
        Some(TokenKind::Identifier(s))
    });
    // Try to match const
    token.try_update(remaining, *index, match_constant, |s| {
        Some(TokenKind::Constant(s))
    });

    // If a match has been found, return it
    if !token.is_empty() {
        consume(&mut remaining, index, token.len());
        return Ok(Some(token));
    }

    // Try to match symbols
    for symbol in Symbol::iter() {
        let symbol_str: &'static str = symbol.into();
        if &remaining[..1] == symbol_str {
            let symbol_index = *index;
            *index += 1;
            return Ok(Some(Token::new(TokenKind::Symbol(symbol), symbol_index)));
        }
    }

    // Unable to match with Token
    lexing_error(data, *index, "invalid sequence")
}

fn consume_whitespace_comments(remaining: &mut &str, index: &mut usize) {
    loop {
        let whitespace = count_whitespace(remaining);
        consume(remaining, index, whitespace);
        let comments = count_comment(remaining);
        consume(remaining, index, comments);

        if (whitespace + comments) == 0 {
            break;
        }
    }
}

fn consume(remaining: &mut &str, index: &mut usize, num_bytes: usize) {
    *remaining = &remaining[num_bytes..];
    *index += num_bytes;
}

/// Return an error showing where in the file the error occurred.
fn lexing_error<S: AsRef<str> + Display>(
    data: &str,
    index: usize,
    message: S,
) -> color_eyre::Result<Option<Token<'_>>> {
    let consumed = get_consumed(data, index);
    let remaining = get_remaining(data, index);
    let consumed_lines: Vec<&str> = consumed.split("\n").collect();
    let line_num = consumed_lines.len();
    let consumed_line = consumed_lines.last().ok_or_eyre("no consumed lines")?;
    let col_num = consumed_line.len() + 1;
    let remaining_line: &str = remaining.split("\n").next().unwrap_or("");
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

fn get_consumed(data: &str, index: usize) -> &str {
    &data[..index]
}

fn get_remaining(data: &str, index: usize) -> &str {
    &data[index..]
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

    fn consume_next_token_helper(
        data: &'static str,
        start_index: usize,
        expected_token: Token<'_>,
        expected_end_index: usize,
        expected_remaining: &'static str,
    ) {
        let mut index = start_index;
        assert_eq!(
            consume_next_token(data, &mut index).unwrap(),
            Some(expected_token)
        );
        assert_eq!(index, expected_end_index);
        assert_eq!(get_remaining(data, index), expected_remaining);
    }

    #[test]
    fn lex_basic_c_code() {
        let my_program = "/*
 * This is my test program.
 * I hope it lexes properly!
 */
int main(void) {
    // It just returns 2 for now. Pretty simple.
    return 2; // This is the return value
}";
        let expected = [
            Token::new(TokenKind::Keyword(Keyword::Int), 64),
            Token::new(TokenKind::Identifier("main"), 68),
            Token::new(TokenKind::Symbol(Symbol::OpenParenthesis), 72),
            Token::new(TokenKind::Keyword(Keyword::Void), 73),
            Token::new(TokenKind::Symbol(Symbol::CloseParenthesis), 77),
            Token::new(TokenKind::Symbol(Symbol::OpenBrace), 79),
            Token::new(TokenKind::Keyword(Keyword::Return), 134),
            Token::new(TokenKind::Constant("2"), 141),
            Token::new(TokenKind::Symbol(Symbol::Semicolon), 142),
            Token::new(TokenKind::Symbol(Symbol::CloseBrace), 172),
        ];

        assert_eq!(&lex(my_program).unwrap()[..], expected);
    }

    #[test]
    fn next_token_keyword() {
        consume_next_token_helper(
            "int my_val = 0;",
            0,
            Token::new(TokenKind::Keyword(Keyword::Int), 0),
            3,
            " my_val = 0;",
        );
    }

    #[test]
    fn next_token_ident() {
        consume_next_token_helper(
            "int my_val = 0;",
            3,
            Token::new(TokenKind::Identifier("my_val"), 4),
            10,
            " = 0;",
        );
    }

    #[test]
    fn next_token_const() {
        consume_next_token_helper(
            "int my_val = 0;",
            12,
            Token::new(TokenKind::Constant("0"), 13),
            14,
            ";",
        );
    }

    #[test]
    fn next_token_semicolon() {
        consume_next_token_helper(
            "int my_val = 0;",
            14,
            Token::new(TokenKind::Symbol(Symbol::Semicolon), 14),
            15,
            "",
        );
    }

    #[test]
    fn next_token_empty() {
        let mut index = 0;
        assert_eq!(consume_next_token("", &mut index).unwrap(), None);
    }

    #[test]
    fn next_token_actually_ident() {
        consume_next_token_helper(
            "// Comment ça va?\ninti",
            0,
            Token::new(TokenKind::Identifier("inti"), 19),
            23,
            "",
        );
    }

    #[test]
    fn next_token_no_number_start() {
        let mut index = 0;
        let _ = consume_next_token("123bar", &mut index).unwrap_err();
    }

    #[test]
    fn consume_whitespace_simple() {
        let data = "  \n  hello";
        let mut index = 0;
        let mut data_ref = data;

        consume_whitespace_comments(&mut data_ref, &mut index);
        assert_eq!(data_ref, "hello");
        assert_eq!(index, 5);
    }

    #[test]
    fn consume_whitespace_comments_multiple() {
        let data = "   \t\r\n// comment\n/*\n * Multiline\n */\n  hello";
        let mut index = 0;
        let mut data_ref = data;

        consume_whitespace_comments(&mut data_ref, &mut index);
        assert_eq!(data_ref, "hello");
        assert_eq!(index, 39);
    }

    #[test]
    fn check_consume() {
        let data = "hello";
        let mut index = 0;
        let mut data_ref = data;

        consume(&mut data_ref, &mut index, 1);
        assert_eq!(data_ref, "ello");
        assert_eq!(index, 1);

        consume(&mut data_ref, &mut index, 3);
        assert_eq!(data_ref, "o");
        assert_eq!(index, 4);
    }

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
