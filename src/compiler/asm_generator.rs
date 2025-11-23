use crate::compiler::parser::Program;

/// Generates assembly code from a C [`Program`] abstract syntax tree.
pub fn generate_asm(_ast: &Program<'_>) -> color_eyre::Result<String> {
    todo!()
}
