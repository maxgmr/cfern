use crate::compiler::token::{Token, get_next_token};

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
}
