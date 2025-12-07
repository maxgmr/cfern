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
impl<'a> Function<'a> {
    pub fn name_string_asm(&self) -> String {
        format!("\t.globl {}\n{}:", self.name, self.name)
    }
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
impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Instruction::*;

        match self {
            Mov {
                source,
                destination,
            } => write!(f, "\tmovl {source}, {destination}"),
            Ret => write!(f, "\tret"),
        }
    }
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
impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Operand::*;

        match self {
            Register(r) => r.fmt(f),
            ImmediateInt(i) => write!(f, "${i}"),
        }
    }
}

/// Represents a particular register used as an operand within an ASM abstract syntax tree.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Register {
    Eax,
}
impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Register::*;

        match self {
            Eax => write!(f, "%eax"),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_reg() {
        assert_eq!(Operand::Register(Register::Eax).to_string(), "%eax");
    }

    #[test]
    fn display_imm() {
        assert_eq!(Operand::ImmediateInt(-4).to_string(), "$-4");
        assert_eq!(Operand::ImmediateInt(256).to_string(), "$256");
    }

    #[test]
    fn display_mov() {
        assert_eq!(
            Instruction::Mov {
                source: Operand::ImmediateInt(1234),
                destination: Operand::Register(Register::Eax)
            }
            .to_string(),
            "\tmovl $1234, %eax"
        );
    }
}
