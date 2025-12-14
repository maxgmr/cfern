//! This module is responsible for lexing preprocessed C source code into [`Token`]s.

use crate::token::Token;

mod matchers;
mod stream;

use matchers::{ConstantMatcher, IdentifierMatcher, KeywordMatcher, SymbolMatcher, TokenMatcher};
use stream::SourceStream;

/// Lex a string of preprocessed C code into a list of [`Token`]s.
///
/// # Errors
///
/// This function returns a [`LexingError`] if any part of the given C code doesn't match a valid
/// [`Token`].
pub fn lex(data: &str) -> Result<Vec<Token<'_>>, LexingError> {
    let mut stream = SourceStream::new(data);
    let mut tokens = Vec::new();

    // Create matchers in priority order
    let matchers: Vec<Box<dyn TokenMatcher>> = vec![
        Box::new(KeywordMatcher),
        Box::new(IdentifierMatcher),
        Box::new(ConstantMatcher),
        Box::new(SymbolMatcher),
    ];

    while !stream.is_at_end() {
        stream.consume_whitespace_and_comments();

        if stream.is_at_end() {
            break;
        }

        let start_position = stream.position();
        let mut matched = false;

        // Try matching with each token type in priority order
        for matcher in &matchers {
            if let Some(token) = matcher.try_match(stream.remaining(), start_position) {
                stream.advance(token.len());
                tokens.push(token);
                matched = true;
                break;
            }
        }

        if !matched {
            return Err(LexingError(stream.position()));
        }
    }

    Ok(tokens)
}

/// An error which can occur if the provided sequence doesn't match a valid [`Token`]. Contains the
/// index within the provided sequence where the error occurred.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct LexingError(usize);
impl LexingError {
    /// Get the index where this [`LexingError`] occurred.
    #[must_use]
    pub fn index(&self) -> usize {
        self.0
    }
}
impl std::fmt::Display for LexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "no valid token at index {}", self.0)
    }
}
impl std::error::Error for LexingError {}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::token::{Keyword, Symbol, TokenKind};

    #[test]
    fn lex_basic_program() {
        let data = "int main(void) { return 2; }";
        let expected = [
            Token::new(TokenKind::Keyword(Keyword::Int), 0),
            Token::new(TokenKind::Identifier("main"), 4),
            Token::new(TokenKind::Symbol(Symbol::OpenParenthesis), 8),
            Token::new(TokenKind::Keyword(Keyword::Void), 9),
            Token::new(TokenKind::Symbol(Symbol::CloseParenthesis), 13),
            Token::new(TokenKind::Symbol(Symbol::OpenBrace), 15),
            Token::new(TokenKind::Keyword(Keyword::Return), 17),
            Token::new(TokenKind::Constant("2"), 24),
            Token::new(TokenKind::Symbol(Symbol::Semicolon), 25),
            Token::new(TokenKind::Symbol(Symbol::CloseBrace), 27),
        ];
        let tokens = lex(data).unwrap();
        assert_eq!(tokens, expected);
    }
}
