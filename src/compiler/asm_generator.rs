use crate::compiler::parser::CProgram;

/// Generates an assembly abstract syntax tree from a [`CProgram`] abstract syntax tree.
pub fn generate_asm<'a>(data: &'a str, ast: &CProgram<'_>) -> color_eyre::Result<AsmProgram<'a>> {
    todo!()
}

/// The root node of an ASM abstract syntax tree. Represents an ASM program.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AsmProgram<'a> {
    pub function: Function<'a>,
}

/// Represents a function within an ASM abstract syntax tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function<'a> {
    pub name: &'a str,
    pub instructions: Vec<Instruction>,
}

/// Represents an ASM instruction within an ASM abstract syntax tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Instruction {
    Mov {
        source: Operand,
        destination: Operand,
    },
    Ret,
}

/// Represents an operand used in an ASM instruction within an ASM abstract syntax tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Operand {
    Register(Register),
    ImmediateInt(isize),
}

/// Represents a particular register used as an operand within an ASM abstract syntax tree.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Register {
    Eax,
}

/// An error which can occur whilst parsing a [`CProgram`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AsmGenerationError {}
