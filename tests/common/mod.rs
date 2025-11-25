use std::fs;

pub fn get_input_string(path: &str) -> color_eyre::Result<String> {
    Ok(fs::read_to_string(path)?)
}
