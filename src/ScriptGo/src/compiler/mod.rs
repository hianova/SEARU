#![allow(unused_imports)]
use covopt_macro::covopt_param;
use std::io::Write;
pub mod ast;
pub mod codegen;
pub mod ir;
pub mod lexer;
pub mod optimizer;
pub mod parser;

pub fn compile_high_level(source: &str) -> Result<(alloc::vec::Vec<crate::sgl::instruction::Instruction>, no_std_tool::structures::collections::HashMap<alloc::string::String, u8>), alloc::string::String> {
    use alloc::format;
    use crate::compiler::{lexer::Lexer, parser::Parser, codegen::CodeGen};
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().map_err(|e| format!("Lexer error: {}", e))?;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().map_err(|e| format!("Parser error: {}", e))?;
    let mut codegen = CodeGen::new();
    let bytecode = codegen.compile(&ast)?;
    Ok((bytecode, codegen.vars_reg))
}

#[cfg(test)]
mod lexer_tests;
#[cfg(test)]
mod m2_stress_tests;
#[cfg(test)]
mod parser_tests;
