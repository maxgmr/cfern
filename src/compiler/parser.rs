use crate::compiler::token::{Keyword, Symbol, Token, TokenKind};

/// Converts an array of [`Token`]s into an abstract syntax tree in the form of a [`Program`].
pub fn parse<'a>(tokens: &'a [Token]) -> color_eyre::Result<Program<'a>> {
    let mut index = 0;
    Ok(parse_program(tokens, &mut index)?)
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

#[derive(Debug)]
struct ParseError;
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TODO")
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
    let ident = match get_next(tokens, index).kind {
        TokenKind::Identifier(name) => name,
        _ => return Err(ParseError),
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
    match get_next(tokens, index).kind {
        TokenKind::Constant(s) => Ok(Expression::Constant(s)),
        _ => Err(ParseError),
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
        _ => Err(ParseError),
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
        _ => Err(ParseError),
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
