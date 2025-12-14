//! Responsible for managing the command-line interface of `cfern`.

use camino::Utf8PathBuf;
use clap::{ArgGroup, Parser};

/// All available `cfern` arguments and flags.
#[derive(Clone, Debug, Parser)]
#[clap(
    author = "Max Gilmour", 
    about = "A basic x86_64 C compiler",
    group(
        ArgGroup::new("run_step")
            .required(false)
            .args(["lex", "parse", "codegen", "assembly"])
    )
)]
#[allow(clippy::struct_excessive_bools)]
pub struct Cli {
    /// Run the lexer, but stop before parsing.
    #[clap(short = 'L', long)]
    pub lex: bool,

    /// Run the lexer and parser, but stop before assembly generation.
    #[clap(short = 'P', long)]
    pub parse: bool,

    /// Perform lexing, parsing, and assembly generation, but stop before code emission.
    #[clap(short = 'C', long)]
    pub codegen: bool,

    /// Emit an assembly file without assembling or linking it.
    #[clap(short = 'S', long)]
    pub assembly: bool,

    /// The C file to compile.
    pub input_file: Utf8PathBuf,
}
