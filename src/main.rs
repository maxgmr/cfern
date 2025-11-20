//! # cfern
//! A basic `x86_64` C compiler.
#![cfg(all(target_os = "linux", target_arch = "x86_64"))]
#![warn(
    missing_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    clippy::all,
    clippy::pedantic,
    clippy::todo
)]

use std::{
    fs,
    io::{self, Read},
};

use camino::Utf8PathBuf;
use clap::Parser;
use color_eyre::eyre;

mod parse_cli;

use parse_cli::Cli;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let input = read_input_file(cli.input_file.as_ref())?;

    Ok(())
}

fn read_input_file(input_file: Option<&Utf8PathBuf>) -> io::Result<String> {
    match input_file {
        Some(path) => fs::read_to_string(path),
        None => read_stdin(),
    }
}

fn read_stdin() -> io::Result<String> {
    let mut bytes = Vec::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_end(&mut bytes)?;
    String::from_utf8(bytes).map_err(|e| {
        let s = e.to_string();
        io::Error::new(io::ErrorKind::InvalidInput, s)
    })
}
