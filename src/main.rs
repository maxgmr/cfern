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

mod assemble_and_link;
mod compile;
mod parse_cli;
mod preprocess;

use parse_cli::Cli;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let preprocessed_file = preprocess::preprocess(&cli.input_file)?;
    let assembly_file = compile::compile(&preprocessed_file)?;
    assemble_and_link::assemble_and_link(&assembly_file)?;

    Ok(())
}
