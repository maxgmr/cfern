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

use clap::Parser;
use color_eyre::eyre;

mod parse_cli;
mod preprocess;

use parse_cli::Cli;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let preprocessed_file = preprocess::preprocess(&cli.input_file)?;

    Ok(())
}
