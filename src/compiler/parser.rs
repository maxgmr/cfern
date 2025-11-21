use crate::compiler::token::Token;

/// Stub struct until real AST can be implemented
#[derive(Clone, Debug)]
pub struct AbstractSyntaxTree;

/// Convert an array of [`Token`]s to an [`AbstractSyntaxTree`].
pub fn parse(tokens: &[Token]) -> color_eyre::Result<AbstractSyntaxTree> {
    todo!()
}
