/*
    parser.rs

    Parses the concrete syntax into an internal abstract syntax for rigorous compilation.
*/

use crate::types::*;
use crate::utils::*;

use sexp::Atom::*;
use sexp::*;

use std::collections::HashSet;

/// Parses the s-expression comprising the program into an abstract program structure
pub fn parse_program(sexps : Sexp) -> Program {
    // Match the program s-exp
    match sexps {
        // Internal list of s-expressions comprising 
        // each function definition and the main expression
        Sexp::List(vec) => {
            // A program without any contents is invalid
            if vec.is_empty() {
                panic!("Invalid");
            }
            
            // Compile the function map via the function names
            let mut defns : Vec<Function> = Vec::new();
            let mut fnames : HashSet<String> = HashSet::new();
            for i in 0..(vec.len()-1) {
                let fname = parse_defn_name(&vec[i]);
                if !is_valid_identifier(&fname) {
                    panic!("Invalid function naming conventions");
                }
                if fnames.contains(&fname) {
                    panic!("Duplicate function name");
                }
                if (*RESERVED).contains(&fname) {
                    panic!("Invalid function definition - keyword");
                }
                fnames.insert(fname); // append to function map
            }
            
            // Parse the function definitions and append to list
            for i in 0..(vec.len()-1) {
                defns.push(parse_defn(&vec[i], &fnames));
            }

            // Return program structure with parsed main expression
            Program { defns: defns, main : parse_expr(&vec[vec.len()-1], &fnames) }
        },
        _ => panic!("Invalid - Must send sexp list with parentheses"),
    }
}

/// Parses out the function name from a function s-expression
pub fn parse_defn_name(s: &Sexp) -> String {
    // Match the s-expression to a list
    match s {
        Sexp::List(vec) => {
            match &vec[..] {
                // Match the list to [fun, definition-list, etc...]
                [Sexp::Atom(S(fun_word)), Sexp::List(decl), _] if fun_word == "fun" => {
                    // Match out the first element string of the definition list
                    match &decl[..] {
                        [Sexp::Atom(S(name)), ..] => {
                            name.to_string() // Return function name
                        },
                        _ => panic!("Invalid"),
                    }
                },
                _ => panic!("Invalid"),
            }
        },
        _ => panic!("Invalid"),
    }
}

/// Parse complete function definition into abstract function structure
pub fn parse_defn(s: &Sexp, fmap: &HashSet<String>) -> Function {
    let fname : String;
    let mut fargs : Vec<String> = Vec::new();           // list of function arguments
    let mut fargset : HashSet<String> = HashSet::new(); // set  of function arguments:
                                                        // used for checking duplicates
    let fbody : Expr;

    // Match function s-expression
    match s {
        Sexp::List(vec) => {
            match &vec[..] {
                // Match list to [fun, definition-list, expr-body]
                [Sexp::Atom(S(fun_word)), Sexp::List(decl), body] if fun_word == "fun" => {
                    fbody = parse_expr(body, fmap);     // parse main expression of function
                    match &decl[..] {
                        [Sexp::Atom(S(name)), args @ ..] => {
                            fname = name.to_string();   // function name from first element

                            // Compile function arguments into the list and set
                            for sexp in args {
                                if let Sexp::Atom(S(arg)) = sexp {
                                    if !is_valid_identifier(arg) {
                                        panic!("Invalid argument naming conventions");
                                    }
                                    if fargset.contains(arg) {
                                        panic!("Duplicate parameter name");
                                    }
                                    if (*RESERVED).contains(arg) {
                                        panic!("Invalid function definition - keyword");
                                    }
                                    fargs.push(arg.to_string());
                                    fargset.insert(arg.to_string());
                                } else {
                                    panic!("Invalid");
                                }
                            }
                        },
                        _ => { panic!("Invalid"); },
                    }
                },
                _ => { panic!("Invalid"); },
            }
        },
        _ => { panic!("Invalid"); },
    };

    // Return function structure
    Function { name: fname, args: fargs, body: fbody }
}

/// Parse from s-expression into a Snek abstract expression format
pub fn parse_expr(s: &Sexp, fmap: &HashSet<String>) -> Expr {
    match s {
        // Match the single value s-exp (no parentheses)
        Sexp::Atom(val) => {
            match val {
                // If number
                I(n) => {
                    let n = i64::try_from(*n).unwrap();
                    if n > LIM - 1 || n < -1 * LIM {
                        panic!("Invalid")
                    } else {
                        Expr::Number(n)
                    }
                },
                // If boolean string
                S(v) if v == "true" => Expr::Boolean(true),
                S(v) if v == "false" => Expr::Boolean(false),
                // If non-boolean string (identifier)
                S(v) => {
                    if !is_valid_identifier(v) {
                        panic!("Invalid identifier naming conventions");
                    }
                    if (*RESERVED).contains(v) && v != "input" {
                        panic!("Invalid identifier - keyword");
                    }
                    Expr::Id(v.to_string())
                }
                _ => panic!("Invalid"),
            }
        },
        // Match the list s-exp (has outer parentheses)
        Sexp::List(vec) => {
            match &vec[..] {
                // Match loop
                [Sexp::Atom(S(loop_word)), e] if loop_word == "loop" => {
                    Expr::Loop(Box::new(parse_expr(e, fmap)))
                },
                // Match break
                [Sexp::Atom(S(break_word)), e] if break_word == "break" => {
                    Expr::Break(Box::new(parse_expr(e, fmap)))
                },
                // Match block
                [Sexp::Atom(S(block_word)), s_exprs @ ..] if block_word == "block" => {
                    if s_exprs.is_empty() {
                        panic!("Invalid");
                    } else {
                        let mut exprs = Vec::new();
                        for e in s_exprs {
                            exprs.push(parse_expr(e, fmap));
                        }
                        Expr::Block(exprs)
                    }
                },
                // Match function call w/o arguments
                [Sexp::Atom(S(func))] if fmap.contains(func) => {
                    Expr::Call(func.clone(), Vec::new())
                },
                // Match function call with arguments
                [Sexp::Atom(S(func)), sexprs @ ..] if fmap.contains(func) => {
                    let mut exprs = Vec::new();
                    for e in sexprs {
                        exprs.push(parse_expr(e, fmap));
                    }
                    Expr::Call(func.clone(), exprs)
                },
                // Match unary operations
                [Sexp::Atom(S(op)), e] => {
                    let op = match Some(&*op.to_string()) {
                        Some("add1") => Op1::Add1,
                        Some("sub1") => Op1::Sub1,
                        Some("isnum") => Op1::IsNum,
                        Some("isbool") => Op1::IsBool,
                        Some("print") => Op1::Print,
                        _ => panic!("Invalid"),
                    };
                    Expr::UnOp(op, Box::new(parse_expr(e, fmap)))
                },
                // Match let bindings
                [Sexp::Atom(S(let_word)), Sexp::List(bindings), e] if let_word == "let" => {
                    let mut binds : Vec<(String, Expr)> = Vec::new();
                    let mut ids : HashSet<String> = HashSet::new();
                    
                    // loop through the bindings and add them to a list (and hashset to check duplicates)
                    for b in bindings {
                        let (id, expr) = parse_bind(b, fmap);
                        // if any duplicate bindings, error out
                        if !is_valid_identifier(&id) {
                            panic!("Invalid identifier naming conventions");
                        }
                        if ids.contains(&id) {
                            panic!("Duplicate binding");
                        }
                        if (*RESERVED).contains(&id) {
                            panic!("Invalid identifier - keyword");
                        }
                        ids.insert(id.clone());
                        binds.push((id, expr));
                    }
                    // cannot have no bindings in the let
                    if binds.len() == 0 {
                        panic!("Invalid");
                    }
                    Expr::Let(binds, Box::new(parse_expr(e, fmap)))
                },
                // Match if clause
                [Sexp::Atom(S(if_word)), e1, e2, e3] if if_word == "if" => {
                    Expr::If(Box::new(parse_expr(e1, fmap)), 
                        Box::new(parse_expr(e2, fmap)), 
                        Box::new(parse_expr(e3, fmap)))
                },
                // Match set!
                [Sexp::Atom(S(set_word)), Sexp::Atom(S(var)), e] if set_word == "set!" => {
                    if !is_valid_identifier(var) {
                        panic!("Invalid identifier naming conventions");
                    }
                    if (*RESERVED).contains(var) {
                        panic!("Invalid identifier - keyword");
                    }
                    Expr::Set(var.to_string(), Box::new(parse_expr(e, fmap)))
                },
                // Match binary operations
                [Sexp::Atom(S(op)), e1, e2] => {
                    let op = match Some(&*op.to_string()) {
                        Some("+") => Op2::Plus,
                        Some("-") => Op2::Minus,
                        Some("*") => Op2::Times,
                        Some("<") => Op2::Lt,
                        Some(">") => Op2::Gt,
                        Some("<=") => Op2::Lte,
                        Some(">=") => Op2::Gte,
                        Some("=") => Op2::Equal,
                        _ => panic!("Invalid"),
                    };
                    Expr::BinOp(op, Box::new(parse_expr(e1, fmap)), Box::new(parse_expr(e2, fmap)))
                },
                _ => panic!("Invalid"),
            }
        },
    }
}

/// Parse a singular let binding by recursion
fn parse_bind(s: &Sexp, fmap: &HashSet<String>) -> (String, Expr) {
    // Match let binding list s-exp
    if let Sexp::List(vec) = s {
        if let [Sexp::Atom(S(var)), e] = &vec[..] {
            // Return identifier string and parsed inner expression
            (var.to_string(), parse_expr(e, fmap)) 
        } else {
            panic!("Invalid");
        }
    } else {
        panic!("Invalid");
    }
}