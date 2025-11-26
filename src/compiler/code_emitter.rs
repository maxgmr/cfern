use crate::compiler::asm_generator::AsmProgram;

use camino::Utf8PathBuf;

/// Writes assembly code to a file, returning the path to the file on success.
pub fn emit_code(_asm: &AsmProgram) -> color_eyre::Result<Utf8PathBuf> {
    todo!()
}
