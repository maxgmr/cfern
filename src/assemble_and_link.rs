use std::{
    io::{self, Write},
    process::Command,
};

use camino::Utf8Path;
use color_eyre::eyre::eyre;

/// Currently just a wrapper around `gcc ASSEMBLY_FILE -o OUTPUT_FILE`.
pub fn assemble_and_link(path: &Utf8Path) -> color_eyre::Result<()> {
    let mut output_path = path.to_owned();
    if !output_path.set_extension("") {
        return Err(eyre!("no input file name given"));
    }

    let output = Command::new("gcc")
        .arg(path)
        .arg("-o")
        .arg(&output_path)
        .output()?;
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    let output = Command::new("rm").arg(path).output()?;
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    Ok(())
}
