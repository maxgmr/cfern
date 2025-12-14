//! Library crate for the `cfern` C compiler

#![warn(
    missing_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    clippy::all,
    clippy::pedantic,
    clippy::unwrap_used,
    clippy::todo
)]

pub mod assemble_and_link;
pub mod compiler;
pub mod lexer;
pub mod parse_cli;
pub mod preprocess;
pub mod token;
