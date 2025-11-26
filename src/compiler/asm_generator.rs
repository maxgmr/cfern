use crate::compiler::parser::{self, CProgram};

/// Generates an assembly abstract syntax tree from a [`CProgram`] abstract syntax tree.
pub fn generate_asm<'a>(ast: &CProgram<'a>) -> Result<AsmProgram<'a>, AsmGenerationError> {
    parse_program(ast)
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
impl Operand {
    fn try_parse_imm_int(expr: &parser::Expression<'_>) -> Result<Self, AsmGenerationError> {
        match expr {
            parser::Expression::Constant(s) => {
                Ok(Self::ImmediateInt(s.parse::<isize>().map_err(|_| {
                    AsmGenerationError::IntParseError {
                        input: s.to_string(),
                    }
                })?))
            }
        }
    }
}

/// Represents a particular register used as an operand within an ASM abstract syntax tree.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Register {
    Eax,
}

/// An error which can occur whilst parsing a [`CProgram`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AsmGenerationError {
    IntParseError { input: String },
}
impl std::fmt::Display for AsmGenerationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsmGenerationError::IntParseError { input } => {
                write!(f, "failed to parse string \"{input}\" as int")
            }
        }
    }
}
impl std::error::Error for AsmGenerationError {}

fn parse_program<'a>(ast: &CProgram<'a>) -> Result<AsmProgram<'a>, AsmGenerationError> {
    Ok(AsmProgram {
        function: parse_function(&ast.function)?,
    })
}

fn parse_function<'a>(function: &parser::Function<'a>) -> Result<Function<'a>, AsmGenerationError> {
    Ok(Function {
        name: function.name,
        instructions: parse_instructions(&function.body)?,
    })
}

fn parse_instructions(
    body: &parser::Statement<'_>,
) -> Result<Vec<Instruction>, AsmGenerationError> {
    match body {
        parser::Statement::Return(expr) => Ok(vec![
            Instruction::Mov {
                source: Operand::try_parse_imm_int(expr)?,
                destination: Operand::Register(Register::Eax),
            },
            Instruction::Ret,
        ]),
    }
}
