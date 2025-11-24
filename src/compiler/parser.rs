use crate::compiler::token::{Keyword, Symbol, Token, TokenKind};

/// Converts an array of [`Token`]s into an abstract syntax tree in the form of a [`Program`].
pub fn parse<'a>(tokens: &'a [Token]) -> Result<Program<'a>, ParseError> {
    let mut index = 0;
    let result = parse_program(tokens, &mut index);
    if index < tokens.len() {
        let extra_token = get_next(tokens, &mut index);
        return Err(ParseError::UnexpectedToken {
            token_index: extra_token.index(),
        });
    }
    result
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program<'a> {
    pub function: Function<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function<'a> {
    pub name: &'a str,
    pub body: Statement<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement<'a> {
    Return(Expression<'a>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression<'a> {
    Constant(&'a str),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    ExpectedKeyword {
        token_index: usize,
        expected: Keyword,
        actual: String,
    },
    ExpectedIdent {
        token_index: usize,
        actual: String,
    },
    ExpectedConst {
        token_index: usize,
        actual: String,
    },
    ExpectedSymbol {
        token_index: usize,
        expected: Symbol,
        actual: String,
    },
    UnexpectedEofKeyword {
        expected: Keyword,
    },
    UnexpectedEofSymbol {
        expected: Symbol,
    },
    UnexpectedToken {
        token_index: usize,
    },
}
impl ParseError {
    pub fn index(&self, data: &str) -> usize {
        use ParseError::*;

        match self {
            ExpectedKeyword { token_index, .. } => *token_index,
            ExpectedIdent { token_index, .. } => *token_index,
            ExpectedConst { token_index, .. } => *token_index,
            ExpectedSymbol { token_index, .. } => *token_index,
            UnexpectedEofSymbol { .. } => data.len() - 1,
            UnexpectedEofKeyword { .. } => data.len() - 1,
            UnexpectedToken { token_index } => *token_index,
        }
    }
}
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::ExpectedKeyword {
                token_index: index,
                expected,
                actual,
            } => write!(
                f,
                "expected keyword \"{expected}\" at index {index}, got {actual}"
            ),
            ParseError::ExpectedIdent {
                token_index: index,
                actual,
            } => {
                write!(f, "expected identifier at index {index}, got {actual}")
            }
            ParseError::ExpectedConst {
                token_index: index,
                actual,
            } => {
                write!(f, "expected constant at index {index}, got {actual}")
            }
            ParseError::ExpectedSymbol {
                token_index: index,
                expected,
                actual,
            } => write!(
                f,
                "expected symbol `{expected}` at index {index}, got {actual}",
            ),
            ParseError::UnexpectedEofKeyword { expected } => {
                write!(f, "expected keyword \"{expected}\", reached end of file")
            }
            ParseError::UnexpectedEofSymbol { expected } => {
                write!(f, "expected symbol `{expected}`, reached end of file")
            }
            ParseError::UnexpectedToken { token_index } => {
                write!(f, "unexpected token at index {token_index}")
            }
        }
    }
}
impl std::error::Error for ParseError {}

fn parse_program<'a>(
    tokens: &'a [Token<'a>],
    index: &mut usize,
) -> Result<Program<'a>, ParseError> {
    Ok(Program {
        function: parse_function(tokens, index)?,
    })
}

fn parse_function<'a>(
    tokens: &'a [Token<'a>],
    index: &mut usize,
) -> Result<Function<'a>, ParseError> {
    expect_keyword(tokens, index, Keyword::Int)?;
    let ident = match get_next(tokens, index) {
        Token {
            kind: TokenKind::Identifier(name),
            ..
        } => name,
        token => {
            return Err(ParseError::ExpectedIdent {
                token_index: token.index(),
                actual: format!("{token:?}"),
            });
        }
    };
    expect_symbol(tokens, index, Symbol::OpenParenthesis)?;
    expect_keyword(tokens, index, Keyword::Void)?;
    expect_symbol(tokens, index, Symbol::CloseParenthesis)?;
    expect_symbol(tokens, index, Symbol::OpenBrace)?;

    let statement = parse_statement(tokens, index)?;

    expect_symbol(tokens, index, Symbol::CloseBrace)?;

    Ok(Function {
        name: ident,
        body: statement,
    })
}

fn parse_statement<'a>(
    tokens: &'a [Token<'a>],
    index: &mut usize,
) -> Result<Statement<'a>, ParseError> {
    expect_keyword(tokens, index, Keyword::Return)?;

    let expression = parse_expression(tokens, index)?;

    expect_symbol(tokens, index, Symbol::Semicolon)?;

    Ok(Statement::Return(expression))
}

fn parse_expression<'a>(
    tokens: &'a [Token<'a>],
    index: &mut usize,
) -> Result<Expression<'a>, ParseError> {
    match get_next(tokens, index) {
        Token {
            kind: TokenKind::Constant(s),
            ..
        } => Ok(Expression::Constant(s)),
        token => Err(ParseError::ExpectedConst {
            token_index: token.index(),
            actual: format!("{token:?}"),
        }),
    }
}

fn current<'a>(tokens: &'a [Token<'a>], index: usize) -> Option<&'a Token<'a>> {
    tokens.get(index)
}

fn get_next<'a>(tokens: &'a [Token<'a>], index: &mut usize) -> &'a Token<'a> {
    let token = &tokens[*index];
    *index += 1;
    token
}

fn expect_keyword<'a>(
    tokens: &'a [Token<'a>],
    index: &mut usize,
    keyword: Keyword,
) -> Result<(), ParseError> {
    match current(tokens, *index) {
        Some(Token {
            kind: TokenKind::Keyword(k),
            ..
        }) if *k == keyword => {
            get_next(tokens, index);
            Ok(())
        }
        Some(token) => Err(ParseError::ExpectedKeyword {
            token_index: token.index(),
            expected: keyword,
            actual: format!("{token:?}"),
        }),
        None => Err(ParseError::UnexpectedEofKeyword { expected: keyword }),
    }
}

fn expect_symbol<'a>(
    tokens: &'a [Token<'a>],
    index: &mut usize,
    symbol: Symbol,
) -> Result<(), ParseError> {
    match current(tokens, *index) {
        Some(Token {
            kind: TokenKind::Symbol(s),
            ..
        }) if *s == symbol => {
            get_next(tokens, index);
            Ok(())
        }
        Some(token) => Err(ParseError::ExpectedSymbol {
            token_index: token.index(),
            expected: symbol,
            actual: format!("{token:?}"),
        }),
        None => Err(ParseError::UnexpectedEofSymbol { expected: symbol }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple() {
        let tokens = [
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
        let expected = Program {
            function: Function {
                name: "main",
                body: Statement::Return(Expression::Constant("2")),
            },
        };
        assert_eq!(parse(&tokens[..]).unwrap(), expected);
    }
}
