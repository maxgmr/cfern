use std::fs;

use camino::{Utf8Path, Utf8PathBuf};
use color_eyre::eyre::eyre;

pub mod asm_generator;
pub mod code_emitter;
pub mod lexer;
pub mod parser;
pub mod token;

pub use asm_generator::generate_asm;
pub use code_emitter::emit_code;
pub use lexer::lex;
pub use parser::parse;

const ASSEMBLY_EXTENSION: &str = "s";

const ASM_STUB: &str = ".globl main
main:
    xor %eax, %eax
    ret
";

/// Currently a stub.
pub fn compile(preprocessed_path: &Utf8Path) -> color_eyre::Result<Utf8PathBuf> {
    let mut output_path = preprocessed_path.to_owned();
    if !output_path.set_extension(ASSEMBLY_EXTENSION) {
        return Err(eyre!("no preprocessed file name given"));
    }

    // TODO
    fs::write(&output_path, ASM_STUB)?;

    // Clean up preprocessed file
    fs::remove_file(preprocessed_path)?;

    Ok(output_path)
}
