use std::fs;

use camino::Utf8PathBuf;

pub fn get_input_string(path: &str) -> color_eyre::Result<String> {
    Ok(fs::read_to_string(path)?)
}

#[derive(Clone, Debug)]
pub struct IntermediateFile(pub Utf8PathBuf);
impl Drop for IntermediateFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}
