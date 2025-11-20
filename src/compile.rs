use std::{
    io::{self, Write},
    process::Command,
};

use camino::{Utf8Path, Utf8PathBuf};
use color_eyre::eyre::eyre;

const ASSEMBLY_EXTENSION: &str = ".s";

/// Currently a stub.
pub fn compile(preprocessed_path: &Utf8Path) -> color_eyre::Result<Utf8PathBuf> {
    let mut output_path = preprocessed_path.to_owned();
    if !output_path.set_extension(ASSEMBLY_EXTENSION) {
        return Err(eyre!("no preprocessed file name given"));
    }

    // TODO
    let output = Command::new("touch").arg(&output_path).output()?;
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    let output = Command::new("rm").arg(preprocessed_path).output()?;
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    Ok(output_path)
}
