use std::process::Command;

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
    Command::new("touch").arg(&output_path).output()?;

    Command::new("rm").arg(preprocessed_path).output()?;

    Ok(output_path)
}
