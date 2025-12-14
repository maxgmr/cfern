use std::fs;

use camino::Utf8PathBuf;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct IntermediateFile(pub Utf8PathBuf);
impl Drop for IntermediateFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}
