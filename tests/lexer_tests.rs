mod common;

use cfern::compiler::{
    lex,
    lexer::LexingError,
    token::{Keyword, Symbol, Token, TokenKind},
};
use common::get_input_string;

#[test]
fn return_2() {
    let data = get_input_string("test_inputs/return_2.c").unwrap();
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
    assert_eq!(lex(&data).unwrap(), &expected[..]);
}

#[test]
fn return_2_bad_token() {
    let data = get_input_string("test_inputs/return_2_bad_token.c").unwrap();
    assert_eq!(
        lex(&data).unwrap_err(),
        LexingError::NoValidToken { index: 4 }
    );
}
