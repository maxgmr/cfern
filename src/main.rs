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

use std::fs;

use clap::Parser;
use color_eyre::eyre;

use cfern::{
    assemble_and_link::assemble_and_link, compiler, parse_cli::Cli, preprocess::preprocess,
};

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let preprocessed_file = preprocess(&cli.input_file)?;

    let input_file = fs::read_to_string(&preprocessed_file)?;

    let tokens = compiler::lex(&input_file)?;

    let ast = compiler::parse(&tokens)?;

    let asm = compiler::generate_asm(&ast)?;

    let assembly_file = compiler::emit_code(&asm)?;

    // Return early if assembly-only option enabled
    if cli.assembly {
        return Ok(());
    }

    assemble_and_link(&assembly_file)?;

    Ok(())
}
