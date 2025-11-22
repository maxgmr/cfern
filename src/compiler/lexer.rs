use std::fmt::Display;

use crate::compiler::token::{Token, get_next_token};

use color_eyre::{
    Section, SectionExt,
    eyre::{OptionExt, eyre},
    owo_colors::OwoColorize,
};

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
