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
use color_eyre::{
    Section, SectionExt,
    eyre::{OptionExt, eyre},
    owo_colors::OwoColorize,
};

use cfern::{
    assemble_and_link::assemble_and_link,
    compiler::{self, lexer::LexingError, parser::ParseError},
    parse_cli::Cli,
    preprocess::preprocess,
};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let preprocessed_file = IntermediateFile(preprocess(&cli.input_file)?);

    let input_file = fs::read_to_string(&preprocessed_file.0)?;

    let tokens = match compiler::lex(&input_file) {
        Ok(tokens) => tokens,
        Err(lexing_error) => {
            return Err(CodeError::report_lexing_error(&input_file, &lexing_error));
        }
    };
    // Return early if lex-only option enabled
    if cli.lex {
        return Ok(());
    }

    let ast = match compiler::parse(&tokens) {
        Ok(ast) => ast,
        Err(parse_error) => return Err(CodeError::report_parse_error(&input_file, &parse_error)),
    };
    // Return early if parse-only option enabled
    if cli.parse {
        return Ok(());
    }

    let asm = compiler::generate_asm(&input_file, &ast)?;
    // Return early if codegen-only option enabled
    if cli.codegen {
        return Ok(());
    }

    let assembly_file = compiler::emit_code(&asm)?;
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

#[derive(Clone, Debug)]
struct CodeError {
    line_index: usize,
    col_index: usize,
    consumed_line: String,
    remaining_line: String,
    message: String,
}
impl CodeError {
    fn report_parse_error(data: &str, error: &ParseError) -> color_eyre::Report {
        Self::as_report(Self::convert(data, error.index(data), error.to_string()))
    }

    fn report_lexing_error(data: &str, error: &LexingError) -> color_eyre::Report {
        Self::as_report(Self::convert(data, error.index(), error.to_string()))
    }

    fn convert(data: &str, index: usize, message: String) -> Result<Self, color_eyre::Report> {
        let consumed_lines: Vec<&str> = data[..index].split('\n').collect();
        let line_index = consumed_lines.len().saturating_sub(1);
        let consumed_line = consumed_lines.last().ok_or_eyre("no consumed lines")?;
        let col_index = consumed_line.len();
        let remaining_line: &str = data[index..]
            .split('\n')
            .next()
            .ok_or_eyre("no remaining line")?;
        Ok(Self {
            line_index,
            col_index,
            consumed_line: (*consumed_line).to_string(),
            remaining_line: remaining_line.to_string(),
            message,
        })
    }

    fn as_report(result: Result<Self, color_eyre::Report>) -> color_eyre::Report {
        result.map_or_else(|report| report, color_eyre::Report::from)
    }
}
impl From<CodeError> for color_eyre::Report {
    fn from(value: CodeError) -> Self {
        eyre!("{}", value.message)
            .with_section(|| {
                format!(
                    "Line {}, column {}",
                    (value.line_index + 1).blue(),
                    (value.col_index + 1).blue()
                )
                .header("Point in file:")
            })
            .with_section(|| {
                format!(
                    "{}{}\n{}{}",
                    value.consumed_line,
                    value.remaining_line,
                    (0..value.col_index).map(|_| " ").collect::<String>(),
                    "^ here".bright_red().bold(),
                )
                .header("Line info:")
            })
    }
}
