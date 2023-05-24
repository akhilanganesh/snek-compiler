/*
    types.rs

    Provides all type definitions used by the parser-compiler complex.
*/

use im::HashMap;

/// Assembly values: register, immediate, or stack pointer via register value w/ offset
#[derive(Debug, Clone)]
pub enum Val {
    Reg(Reg),
    Imm(i64),
    MemPtr(Reg, i32),
    Label(String),
}

/// Registers: rax, rbx, rsp
#[derive(Debug, Eq, Hash, Copy, Clone, PartialEq, strum_macros::Display)]
pub enum Reg {
    RAX,    // main register rax
    RBX,    // secondary register rbx
    RCX,    // tertiary register rcx
    RDX,    // main ERROR CHECKING register
    // REX,    // secondary error check register
    RSP,    // stack pointer
    RDI,    // stores first integer argument (input)
    R15,    // r15 stores the current heap pointer
}

/// Instruction Types: mov, add, sub, imul
#[derive(Debug, strum_macros::Display)]
pub enum Instr {
    Label(Val),
    Mov(Val, Val),
    Add(Val, Val),
    Sub(Val, Val),
    IMul(Val, Val),
    CMovl(Val, Val),
    CMovg(Val, Val),
    CMovle(Val, Val),
    CMovge(Val, Val),
    CMove(Val, Val),
    Cmp(Val, Val),
    Test(Val, Val),
    And(Val, Val),
    Xor(Val, Val),
    Sar(Val, Val),
    Jmp(Val),
    Je(Val),
    Jne(Val),
    Jl(Val),
    Jle(Val),
    Jg(Val),
    Jge(Val),
    Jo(Val),
    Push(Val),
    Pop(Val),
    Call(Val),
    Ret,
}

/// Unary operator types
#[derive(Debug, PartialEq)]
pub enum Op1Type {
    Arithmetic,
    TypeCheck,
    Application,
}

/// Unary operators
#[derive(Debug)]
pub enum Op1 {
    Add1,
    Sub1,
    IsNum,
    IsBool,
    Print,
}

/// Binary operator types
#[derive(Debug, PartialEq)]
pub enum Op2Type {
    Arithmetic,
    Equality,
    Relational,
}

/// Binary operators
#[derive(Debug)]
pub enum Op2 {
    Plus,
    Minus,
    Times,
    Lt,
    Gt,
    Lte,
    Gte,
    Equal,
}

/// Snek Expression Types
#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Boolean(bool),
    Tuple(Vec<Expr>),
    Id(String),
    Let(Vec<(String, Expr)>, Box<Expr>),
    UnOp(Op1, Box<Expr>),
    BinOp(Op2, Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Loop(Box<Expr>),
    Break(Box<Expr>),
    Set(String, Box<Expr>),
    TInit(Box<Expr>, Box<Expr>),
    TSet(Box<Expr>, Box<Expr>, Box<Expr>),
    TGet(Box<Expr>, Box<Expr>),
    Block(Vec<Expr>),
    Call(String, Vec<Expr>),
}

/// Function type
#[derive(Debug)]
pub struct Function {
    pub name : String,
    pub args : Vec<String>,
    pub body : Expr,
}

/// Program type
#[derive(Debug)]
pub struct Program {
    pub defns : Vec<Function>,
    pub main : Expr,
}

/// Location pointer type -- either register or stack memory
#[allow(dead_code)]
#[derive(Eq, Hash, Copy, Clone, PartialEq)]
pub enum LocPtr {
    LReg(Reg),
    LStack(i32),
}

/// Value check types
#[allow(dead_code)]
pub enum ValCheck {
    Integer,
    Boolean,
    Tuple,
    Equality,
}

/// Context to an Expr that helps with compilation
#[derive(Copy, Clone)]
pub struct ExprContext<'a> {
    pub si : i32,                           // current stack index
    pub env : &'a HashMap<String, LocPtr>,  // variable environment
    pub loop_num : i32,                     // current loop identifier
    pub func_map : &'a HashMap<String,i32>, // function name map
    pub in_func : bool,                     // whether inside a function or not
    // lbl : &'a mut i32,
}