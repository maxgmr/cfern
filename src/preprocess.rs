use std::process::Command;

use camino::{Utf8Path, Utf8PathBuf};

const PREPROCESSED_EXTENSION: &str = ".i";

/// Currently just a wrapper around `gcc -E -P INPUT_FILE -o PREPROCESSED_FILE`.
///
/// If successful, returns the path to the preprocessed file as a [`Utf8PathBuf`].
pub fn preprocess(path: &Utf8Path) -> color_eyre::Result<Utf8PathBuf> {
    let mut output_path = path.to_owned();
    output_path.set_extension(PREPROCESSED_EXTENSION);
    Command::new("gcc")
        .arg("-E")
        .arg("-P")
        .arg(path)
        .arg("-o")
        .arg(&output_path)
        .output()?;
    Ok(output_path)
}
