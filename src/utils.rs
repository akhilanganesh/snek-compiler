/*
    utils.rs

    Provides utility variables and functions for the compiler.
*/

use crate::types::*;

// use std::fmt;
use std::collections::HashSet;

/// limit to Snek integers (2^62)
pub const LIM : i64 = 4611686018427387904;

/// word size for x86
pub const WORD_SIZE : i32 = 8i32;

/// mismatch error code
pub const MSMX_ERRCODE : i64 = 7; // msmx = mismatch

/// overflow error code
pub const OF_ERRCODE : i64 = 8;

/// bounds error code
pub const BND_ERRCODE : i64 = 9;

/// true  value representation (code + tag)
pub const TRUE_VAL  : i64 = 7;

/// false value representation (code + tag)
pub const FALSE_VAL : i64 = 3;

lazy_static! {
    /// reserved words or keywords
    pub static ref RESERVED : HashSet<String> = {
        HashSet::from([
            String::from("add1"),
            String::from("sub1"),
            String::from("isnum"),
            String::from("isbool"),
            String::from("*"),
            String::from("-"),
            String::from("+"),
            String::from("<"),
            String::from(">"),
            String::from("<="),
            String::from(">="),
            String::from("="),
            String::from("let"),
            String::from("if"),
            String::from("set!"),
            String::from("block"),
            String::from("loop"),
            String::from("break"),
            String::from("true"),
            String::from("false"),
            String::from("input"),
            String::from("print"),
            String::from("fun"),
        ])
    };
}

// impl fmt::Display for Reg {
//     /// Display method for Reg (registers)
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {\
//         match self {
//             Reg::RAX => write!(f, "rax"),
//             Reg::RBX => write!(f, "rbx"),
//             Reg::RSP => write!(f, "rsp"),
//             Reg::RDI => write!(f, "rdi"),
//         }
//     }
// }

impl LocPtr {
    /// Converts location pointer to Val (asm value)
    pub fn value(&self) -> Val {
        match self {
            LocPtr::LReg(reg) => Val::Reg(*reg),
            LocPtr::LStack(offset) => Val::MemPtr(Reg::RSP, *offset),
        }
    }
}

impl Op1 {
    /// Get Op1Type from Op1
    pub fn get_type(&self) -> Op1Type {
        match self {
            Op1::Add1 | Op1::Sub1 => Op1Type::Arithmetic,
            Op1::IsNum | Op1::IsBool => Op1Type::TypeCheck,
            Op1::Print => Op1Type::Application,
        }
    }
}

impl Op2 {
    /// Get Op2Type from Op2
    pub fn get_type(&self) -> Op2Type {
        match self {
            Op2::Plus | Op2::Minus | Op2::Times => Op2Type::Arithmetic,
            Op2::Lt | Op2::Gt | Op2::Lte | Op2::Gte => Op2Type::Relational,
            Op2::Equal => Op2Type::Equality,
        }
    }
}

/// Returns instructions that perform a runtime mismatch error check
pub fn check_msmx(check : Val, check2_opn : Option<Val>, ctype : ValCheck, lbl : &mut i32) -> Vec<Instr> {
    let mut ret : Vec<Instr> = Vec::new();
    let err_val = Val::Label(format!("throw_error_align"));
    match ctype {
        ValCheck::Integer => {
            ret.push(Instr::Test(check, Val::Imm(1)));
            ret.push(Instr::Mov(Val::Reg(Reg::RDX), Val::Imm(MSMX_ERRCODE)));
            ret.push(Instr::Jne(err_val));
        },
        ValCheck::Boolean => {
            ret.push(Instr::Mov(Val::Reg(Reg::RDX), check));
            ret.push(Instr::And(Val::Reg(Reg::RDX), Val::Imm(0b11)));
            ret.push(Instr::Cmp(Val::Reg(Reg::RDX), Val::Imm(0b11)));
            ret.push(Instr::Mov(Val::Reg(Reg::RDX), Val::Imm(MSMX_ERRCODE)));
            ret.push(Instr::Jne(err_val));
        },
        ValCheck::Tuple => {
            ret.push(Instr::Mov(Val::Reg(Reg::RDX), check));
            ret.push(Instr::And(Val::Reg(Reg::RDX), Val::Imm(0b11)));
            ret.push(Instr::Cmp(Val::Reg(Reg::RDX), Val::Imm(0b01)));
            ret.push(Instr::Mov(Val::Reg(Reg::RDX), Val::Imm(MSMX_ERRCODE)));
            ret.push(Instr::Jne(err_val));
        },
        // for equality, check2 must be Some(value2)
        ValCheck::Equality => {
            if let Some(check2) = check2_opn {
                let int_chk = Val::Label(format!("eqcheck_{}", lbl));
                let chk_end = Val::Label(format!("check_end_{}", lbl));
                *lbl += 1;

                // Check if integer
                ret.push(Instr::Mov(Val::Reg(Reg::RDX), check));
                ret.push(Instr::Test(Val::Reg(Reg::RDX), Val::Imm(1)));
                ret.push(Instr::Je(int_chk.clone()));   // if integer, skip alt check

                // Alt check
                ret.push(Instr::Xor(Val::Reg(Reg::RDX), check2.clone()));
                ret.push(Instr::And(Val::Reg(Reg::RDX), Val::Imm(0b11)));
                ret.push(Instr::Cmp(Val::Reg(Reg::RDX), Val::Imm(0)));
                ret.push(Instr::Mov(Val::Reg(Reg::RDX), Val::Imm(MSMX_ERRCODE)));
                ret.push(Instr::Jne(err_val.clone()));
                ret.push(Instr::Jmp(chk_end.clone()));

                // Int check
                ret.push(Instr::Label(int_chk));
                ret.push(Instr::Xor(Val::Reg(Reg::RDX), check2));
                ret.push(Instr::Test(Val::Reg(Reg::RDX), Val::Imm(1)));
                ret.push(Instr::Mov(Val::Reg(Reg::RDX), Val::Imm(MSMX_ERRCODE)));
                ret.push(Instr::Jne(err_val));
                
                // End check
                ret.push(Instr::Label(chk_end));
            } else {
                panic!("No secondary value provided for equality check");
            }
        },
    }
    
    ret
}

/// Returns instructions that perform a runtime overflow error check
pub fn check_of() -> Vec<Instr> {
    let mut ret : Vec<Instr> = Vec::new();
    let err_val = Val::Label(format!("throw_error_align"));
    ret.push(Instr::Mov(Val::Reg(Reg::RDX), Val::Imm(OF_ERRCODE)));
    ret.push(Instr::Jo(err_val));
    ret
}

/// Returns instructions that perform a runtime overflow error check
// lower, inclusive determine the nature of the validity bound
// e.g. lower = true, inclusive = true means >= bound is good, < bound is bad
pub fn check_bnd(check: Val, bound: Val, lower : bool, inclusive: bool) -> Vec<Instr> {
    let mut ret : Vec<Instr> = Vec::new();
    let err_val = Val::Label(format!("throw_error_align"));
    ret.push(Instr::Cmp(check, bound));
    ret.push(Instr::Mov(Val::Reg(Reg::RDX), Val::Imm(BND_ERRCODE)));
    ret.push(
        match (lower, inclusive) {
            (false, false) => Instr::Jge(err_val),
            (false, true)  => Instr::Jg(err_val),
            (true,  false) => Instr::Jle(err_val),
            (true,  true)  => Instr::Jl(err_val),
        }
    );
    ret
}

/// Converts a vector of instructions to a String representation
/// of the asm instruction list
pub fn to_asm(instrs: &Vec<Instr>) -> String {
    let mut asm_str = String::new();
    if instrs.is_empty() {
        return asm_str;
    }
    // formats instructions with a new line after, except for last line
    for instr in &instrs[..instrs.len()-1] {
        asm_str += &(String::from("  ") + &instr_to_str(&instr) + "\n");
    }
    asm_str += &(String::from("  ") + &instr_to_str(&instrs[instrs.len()-1]));
    asm_str
}

/// Converts an abstract instruction to an asm instruction as a String
pub fn instr_to_str(i: &Instr) -> String {
    if let Instr::Label(v) = i {
        format!("{}:", val_to_str(v))   // label_name:
    } else {
        format!("{} {}", i.to_string().to_lowercase(), // op {}
        match i {
            Instr::Mov(v1, v2) | Instr::Add(v1, v2) | Instr::Sub(v1, v2) |
            Instr::IMul(v1, v2) | Instr::CMovl(v1, v2) | Instr::CMovg(v1, v2) | 
            Instr::CMovle(v1, v2) | Instr::CMovge(v1, v2) | Instr::CMove(v1, v2) | 
            Instr::Cmp(v1, v2) | Instr::Test(v1, v2) | Instr::And(v1, v2) | 
            Instr::Xor(v1, v2) | Instr::Sar(v1, v2)
                => format!("{}, {}", val_to_str(v1), val_to_str(v2)), // _, _
            Instr::Jmp(v) | Instr::Je(v) | Instr::Jne(v) | 
            Instr::Jl(v) | Instr::Jle(v) | Instr::Jg(v) | Instr::Jge(v) |
            Instr::Jo(v) | Instr::Push(v) | Instr::Pop(v) | Instr::Call(v)
                => format!("{}", val_to_str(v)), // _
            Instr::Ret
                => format!(""), // nothing
            _ => String::from(""), // nothing
        })
    }
}

/// Converts a Val (asm value) into an asm String representation
pub fn val_to_str(v: &Val) -> String {
    match v {
        Val::Reg(reg) => reg.to_string().to_lowercase(),  // register name
        Val::Imm(imm) => {
            return imm.to_string();        // immediate integer to string
        },
        Val::MemPtr(reg, imm) => {      // qword [reg +- imm]
            if *imm < 0 {
                format!("qword [{} - {}]", reg.to_string().to_lowercase(), -*imm)
            } else if *imm > 0 {
                format!("qword [{} + {imm}]", reg.to_string().to_lowercase())
            } else {
                format!("qword [{}]", reg.to_string().to_lowercase())
            }
        },
        Val::Label(s) => s.to_string(),    // label
    }
}

/// Returns whether or not an identifier or function name is valid
/// (of the pattern [a-zA-z][a-zA-Z0-9]*)
pub fn is_valid_identifier(s: &str) -> bool {
    // check if first character is alphabetic
    if s.len() <= 0 || !s.chars().nth(0).unwrap().is_ascii_alphabetic() {
        return false;
    }
    // check if all other characters are alphanumeric
    for c in s[1..].chars() {
        if !c.is_ascii_alphanumeric() {
            return false;
        }
    }
    true
}