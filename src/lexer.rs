//! This module is responsible for lexing preprocessed C source code into [`Token`]s.

use crate::token::Token;

mod matchers;
mod stream;
