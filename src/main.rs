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

use camino::Utf8PathBuf;
use clap::Parser;
use color_eyre::{Section, SectionExt, eyre::eyre, owo_colors::OwoColorize};

use cfern::{
    assemble_and_link::assemble_and_link,
    compiler::{self, parser::ParseError},
    lexer::{LexingError, lex},
    parse_cli::Cli,
    preprocess::preprocess,
};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let preprocessed_file = IntermediateFile(preprocess(&cli.input_file)?);

    let input_str = fs::read_to_string(&preprocessed_file.0)?;

    let tokens = match lex(&input_str) {
        Ok(tokens) => tokens,
        Err(lexing_error) => {
            return Err(lexing_error.report(&input_str));
        }
    };
    // Return early if lex-only option enabled
    if cli.lex {
        return Ok(());
    }

    let ast = match compiler::parse(&tokens) {
        Ok(ast) => ast,
        Err(parse_error) => {
            return Err(parse_error.report(&input_str));
        }
    };
    // Return early if parse-only option enabled
    if cli.parse {
        return Ok(());
    }

    let asm = compiler::generate_asm(&ast)?;
    // Return early if codegen-only option enabled
    if cli.codegen {
        return Ok(());
    }

    let assembly_file = compiler::emit_code(&cli.input_file, &asm)?;
    // Return early if assembly-only option enabled
    if cli.assembly {
        return Ok(());
    }

    assemble_and_link(&assembly_file)?;

    Ok(())
}

#[derive(Clone, Debug)]
struct IntermediateFile(Utf8PathBuf);
impl Drop for IntermediateFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}

trait CodeError {
    fn report(&self, input_data: &str) -> color_eyre::Report;

    /// Take the input data, index, and message, then generate a `[color_eyre::Report]`.
    fn data_to_report(input_data: &str, index: usize, message: String) -> color_eyre::Report {
        let consumed_lines: Vec<&str> = input_data[..index].split('\n').collect();
        let line_index = consumed_lines.len().saturating_sub(1);
        let consumed_line = consumed_lines.last().expect("no consumed lines");
        let col_index = consumed_line.len();
        let remaining_line: &str = input_data[index..]
            .split('\n')
            .next()
            .expect("no remaining line");

        eyre!("{}", message)
            .with_section(|| {
                format!(
                    "Line {}, column {}",
                    (line_index + 1).blue(),
                    (col_index + 1).blue()
                )
                .header("Point in file:")
            })
            .with_section(|| {
                format!(
                    "{}{}\n{}{}",
                    consumed_line,
                    remaining_line,
                    (0..col_index).map(|_| " ").collect::<String>(),
                    "^ here".bright_red().bold(),
                )
                .header("Line info:")
            })
    }
}
impl CodeError for LexingError {
    fn report(&self, input_data: &str) -> color_eyre::Report {
        LexingError::data_to_report(input_data, self.index(), self.to_string())
    }
}
impl CodeError for ParseError {
    fn report(&self, input_data: &str) -> color_eyre::Report {
        ParseError::data_to_report(input_data, self.index(input_data), self.to_string())
    }
}
