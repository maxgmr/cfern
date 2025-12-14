//! Handles directives like file inclusion and macro expansion before compilation.

use std::{
    io::{self, Write},
    process::Command,
};

use camino::{Utf8Path, Utf8PathBuf};
use color_eyre::eyre::eyre;

const PREPROCESSED_EXTENSION: &str = "i";

/// Currently just a wrapper around `gcc -E -P INPUT_FILE -o PREPROCESSED_FILE`.
///
/// If successful, returns the path to the preprocessed file as a [`Utf8PathBuf`].
///
/// # Errors
///
/// This function returns an error if the call to GCC's preprocessor fails for any reason.
pub fn preprocess(path: &Utf8Path) -> color_eyre::Result<Utf8PathBuf> {
    let mut output_path = path.to_owned();
    if !output_path.set_extension(PREPROCESSED_EXTENSION) {
        return Err(eyre!("no input file name given"));
    }

    let output = Command::new("gcc")
        .arg("-E")
        .arg("-P")
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

    Ok(output_path)
}
