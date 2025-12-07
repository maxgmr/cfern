use std::{fs::OpenOptions, io::Write};

use crate::compiler::asm_generator::AsmProgram;

use camino::{Utf8Path, Utf8PathBuf};
use color_eyre::eyre::eyre;

const ASM_EXTENSION: &str = "s";

/// Writes assembly code to a file, returning the path to the file on success.
pub fn emit_code(input_file_path: &Utf8Path, asm: &AsmProgram) -> color_eyre::Result<Utf8PathBuf> {
    let mut output_path = input_file_path.to_owned();
    if !output_path.set_extension(ASM_EXTENSION) {
        return Err(eyre!("no input file name given"));
    }

    // Create/truncate file
    {
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&output_path)?;
    }

    let mut output_file = OpenOptions::new().append(true).open(&output_path)?;

    let lines = get_lines(asm);

    for line in lines {
        writeln!(output_file, "{line}")?;
    }

    Ok(output_path)
}

fn get_lines(asm: &AsmProgram) -> Vec<String> {
    let mut lines = Vec::new();

    lines.push(asm.function.name_string_asm());

    for instruction in &asm.function.instructions {
        lines.push(instruction.to_string());
    }

    lines.push("\t.section .note.GNU-stack,\"\",@progbits".to_string());

    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::asm_generator::{Function, Instruction, Operand, Register};

    #[test]
    fn return_2_to_lines() {
        let program = AsmProgram {
            function: Function {
                name: "main",
                instructions: vec![
                    Instruction::Mov {
                        source: Operand::ImmediateInt(2),
                        destination: Operand::Register(Register::Eax),
                    },
                    Instruction::Ret,
                ],
            },
        };
        let expected = vec![
            String::from("\t.globl main\nmain:"),
            String::from("\tmovl $2, %eax"),
            String::from("\tret"),
            String::from("\t.section .note.GNU-stack,\"\",@progbits"),
        ];
        assert_eq!(get_lines(&program), expected);
    }
}
