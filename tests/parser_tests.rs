use std::fs;

use cfern::{
    compiler::{
        parse,
        parser::{CProgram, Expression, Function, ParseError, Statement},
    },
    lexer::lex,
    token::Symbol,
};

mod common;

#[test]
fn return_2() {
    let data = fs::read_to_string("test_inputs/return_2.c").unwrap();
    let tokens = lex(&data).unwrap();
    assert_eq!(
        parse(&tokens).unwrap(),
        CProgram {
            function: Function {
                name: "main",
                body: Statement::Return(Expression::Constant("2"))
            }
        }
    );
}

#[test]
fn return_2_repeated_keyword() {
    let data = fs::read_to_string("test_inputs/return_2_repeated_keyword.c").unwrap();
    let tokens = lex(&data).unwrap();
    assert_eq!(
        parse(&tokens).unwrap_err(),
        ParseError::ExpectedIdent {
            token_index: 4,
            actual: "keyword \"int\" at index 4".to_string()
        }
    );
}

#[test]
fn return_2_extra_junk() {
    let data = fs::read_to_string("test_inputs/return_2_extra_junk.c").unwrap();
    let tokens = lex(&data).unwrap();
    assert_eq!(
        parse(&tokens).unwrap_err(),
        ParseError::UnexpectedToken { token_index: 115 }
    );
}

#[test]
fn return_2_missing_close_brace() {
    let data = fs::read_to_string("test_inputs/return_2_missing_close_brace.c").unwrap();
    let tokens = lex(&data).unwrap();
    assert_eq!(
        parse(&tokens).unwrap_err(),
        ParseError::UnexpectedEofSymbol {
            expected: Symbol::CloseBrace
        },
    );
}

#[test]
fn return_2_missing_semicolon() {
    let data = fs::read_to_string("test_inputs/return_2_missing_semicolon.c").unwrap();
    let tokens = lex(&data).unwrap();
    assert_eq!(
        parse(&tokens).unwrap_err(),
        ParseError::ExpectedSymbol {
            token_index: 30,
            expected: Symbol::Semicolon,
            actual: "symbol `}` at index 30".to_string(),
        }
    );
}
