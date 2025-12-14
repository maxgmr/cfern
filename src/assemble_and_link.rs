//! Responsible for assembling and linking the generated assembly code.

use std::{
    fs,
    io::{self, Write},
    process::Command,
};

use camino::Utf8Path;
use color_eyre::eyre::eyre;

/// Currently just a wrapper around `gcc ASSEMBLY_FILE -o OUTPUT_FILE`.
///
/// # Errors
///
/// This function returns an error if the call to the GCC assembler and linker fails.
pub fn assemble_and_link(path: &Utf8Path) -> color_eyre::Result<()> {
    let Some(stem) = path.file_stem() else {
        return Err(eyre!("no input file name given"));
    };
    let output_path = path.with_file_name(stem);

    let output = Command::new("gcc")
        .arg(path)
        .arg("-o")
        .arg(&output_path)
        .output()?;
    if !output.status.success() {
        let msg = String::from_utf8_lossy(&output.stderr).into_owned();
        return Err(eyre!(msg));
    }
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    fs::remove_file(path)?;

    Ok(())
}
