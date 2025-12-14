use std::fs;

use camino::Utf8PathBuf;
use cfern::{
    compiler::{emit_code, generate_asm, parse},
    lexer::lex,
    preprocess::preprocess,
};

mod common;

use common::IntermediateFile;

fn assemble(input_path: &'static str) -> Utf8PathBuf {
    let input_path = Utf8PathBuf::from(input_path);
    let preprocessed_file = IntermediateFile(preprocess(&input_path).unwrap());
    let input_file = fs::read_to_string(&preprocessed_file.0).unwrap();
    let tokens = lex(&input_file).unwrap();
    let ast = parse(&tokens).unwrap();
    let asm = generate_asm(&ast).unwrap();
    emit_code(&input_path, &asm).unwrap()
}

#[test]
fn return_2() {
    let output_path = assemble("test_inputs/return_2.c");
    let expected = "\t.globl main
main:
\tmovl $2, %eax
\tret
\t.section .note.GNU-stack,\"\",@progbits
";
    let result = fs::read_to_string(&output_path).unwrap();
    fs::remove_file(&output_path).unwrap();
    assert_eq!(result, expected);
}
