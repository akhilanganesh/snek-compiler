/*
    main.rs

    Runs compiler on .snek program file and places assembly instructions into a pre-defined
    assembly code template. This is written out to a .s file.
*/


mod types;
mod utils;
mod parser;
mod compiler;

#[macro_use]
extern crate lazy_static;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use sexp::*;

use crate::parser::*;
use crate::compiler::*;
use crate::types::*;

/// Compiles a .snek file into an x86 assembly .s file.
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let in_name = &args[1];
    let out_name = &args[2];

    // Parses .snek file
    let mut in_file = File::open(in_name)?;
    let mut in_contents = String::new();
    in_file.read_to_string(&mut in_contents)?;
    let sexp_str = format!("({})", in_contents);
    let prog : Program;
    if let Ok(parsed) = &parse(&sexp_str) {
        prog = parse_program(parsed.clone());
    } else {
        panic!("Invalid");
    }
    
    // Compiles parsed contents into assembly instructions
    let (functions, result) = compile(&prog);

    // Base assembly format with new assembly instructions
    // for functions and main expression
    let asm_program = format!(
        "
section .text
extern snek_error
extern snek_print
global our_code_starts_here
throw_error_align:
  sub rsp, 8
  mov rdi, rbx
  call snek_error
  add rsp, 8
{}
our_code_starts_here:
{}
  ret
", functions, result);

    // Writes out assembly program contents to file
    let mut out_file = File::create(out_name)?;
    out_file.write_all(asm_program.as_bytes())?;

    Ok(())
}