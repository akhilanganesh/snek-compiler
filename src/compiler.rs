/*
    compiler.rs

    Compiles the internal abstract syntax into a list of abstract assembly instructions, and converts
    this list to a assembly program String.
*/

use crate::types::*;
use crate::utils::*;

use im::HashMap;

/// Compiles Expr expression to a corresponding vector of instructions
fn compile_expr(e: &Expr, ctxt : ExprContext, lbl: &mut i32) -> Vec<Instr> {
    // Initialize instruction vector
    let mut instrs : Vec<Instr> = Vec::new();

    // Match expression enum
    match e {
        // Integer value representation into rax
        Expr::Number(n) => { instrs.push(Instr::Mov(Val::Reg(Reg::RAX), Val::Imm((*n)*2))); },
        // Boolean value representation into rax
        Expr::Boolean(b) => {
            instrs.push(Instr::Mov(Val::Reg(Reg::RAX), Val::Imm(if *b { TRUE_VAL } else { FALSE_VAL })));
        },
        // Tuple value representation into rax
        Expr::Tuple(vec) => {
            // Allocate tuple on the heap
            instrs.push(Instr::Mov(Val::MemPtr(Reg::RSP, -ctxt.si*WORD_SIZE), Val::Reg(Reg::R15)));
            instrs.push(Instr::Add(Val::Reg(Reg::R15), Val::Imm(((vec.len() as i32+1)*WORD_SIZE).into())));

            // Evaluate and place elements in tuple
            let mut offset = 0;
            // TODO: Check the len to see if it fits in i64 (doesn't become negative after)
            instrs.push(Instr::Mov(Val::Reg(Reg::RBX), Val::MemPtr(Reg::RSP, -ctxt.si*WORD_SIZE)));
            instrs.push(Instr::Mov(Val::MemPtr(Reg::RBX, offset*WORD_SIZE), Val::Imm(vec.len() as i64)));
            offset += 1;
            for expr in vec {
                instrs.append(&mut compile_expr(expr, ExprContext { si: ctxt.si + 1, ..ctxt }, lbl));
                instrs.push(Instr::Mov(Val::Reg(Reg::RBX), Val::MemPtr(Reg::RSP, -ctxt.si*WORD_SIZE)));
                instrs.push(Instr::Mov(Val::MemPtr(Reg::RBX, offset*WORD_SIZE), Val::Reg(Reg::RAX)));
                offset += 1;
            }
            
            // Return tuple representation
            instrs.push(Instr::Mov(Val::Reg(Reg::RAX), Val::MemPtr(Reg::RSP, -ctxt.si*WORD_SIZE)));
            instrs.push(Instr::Add(Val::Reg(Reg::RAX), Val::Imm(1)));
        },
        // "input" identifier value in rdi moved to rax
        Expr::Id(s) if s == "input" => {
            if ctxt.in_func {
                panic!("\'input\' cannot be used within function");
            }
            instrs.push(Instr::Mov(Val::Reg(Reg::RAX), Val::Reg(Reg::RDI))); 
        },
        // Identifier value on stack moved to rax
        Expr::Id(s)     => {
            if let Some(loc) = ctxt.env.get(s) {
                instrs.push(Instr::Mov(Val::Reg(Reg::RAX), loc.value()));
            } else {
                panic!("Unbound variable identifier {}", s); // unbound variable error
            }
        },
        // Unary operation performed and result moved to rax
        Expr::UnOp(op, e) => {
            // Compile inner expression into rax
            instrs.append(&mut compile_expr(e, ctxt, lbl));

            // If arithmetic, check for mismatch error
            if op.get_type() == Op1Type::Arithmetic {
                instrs.append(&mut check_msmx(LocPtr::LReg(Reg::RAX)));
            }

            // Match unary operator and perform relevant instructions
            // Store result in rax
            match op {
                // add1 (+= 1)
                Op1::Add1 => {
                    instrs.push(Instr::Add(Val::Reg(Reg::RAX), Val::Imm(2)));
                    instrs.append(&mut check_of()); // Check for overflow
                },
                // sub1 (-= 1)
                Op1::Sub1 => {
                    instrs.push(Instr::Sub(Val::Reg(Reg::RAX), Val::Imm(2)));
                    instrs.append(&mut check_of()); // Check for overflow
                },
                // isnum (whether it is an integer or not)
                Op1::IsNum => { 
                    instrs.push(Instr::Test(Val::Reg(Reg::RAX), Val::Imm(1)));
                    instrs.push(Instr::Mov(Val::Reg(Reg::RBX), Val::Imm(TRUE_VAL)));
                    instrs.push(Instr::Mov(Val::Reg(Reg::RAX), Val::Imm(FALSE_VAL)));
                    instrs.push(Instr::CMove(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
                },
                // isbool (whether it is a boolean or not)
                Op1::IsBool => {
                    instrs.push(Instr::Test(Val::Reg(Reg::RAX), Val::Imm(1)));
                    instrs.push(Instr::Mov(Val::Reg(Reg::RBX), Val::Imm(FALSE_VAL)));
                    instrs.push(Instr::Mov(Val::Reg(Reg::RAX), Val::Imm(TRUE_VAL)));
                    instrs.push(Instr::CMove(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
                },
                // print (calls snek_print using C calling conventions)
                Op1::Print => {
                    let offset = (ctxt.si + if ctxt.si % 2 == 0 { 1 } else { 0 } )*WORD_SIZE;
                    instrs.push(Instr::Sub(Val::Reg(Reg::RSP), Val::Imm(offset as i64)));
                    instrs.push(Instr::Push(Val::Reg(Reg::RDI)));
                    instrs.push(Instr::Push(Val::Reg(Reg::RAX)));
                    instrs.push(Instr::Mov(Val::Reg(Reg::RDI), Val::Reg(Reg::RAX)));
                    instrs.push(Instr::Call(Val::Label(String::from("snek_print"))));
                    instrs.push(Instr::Pop(Val::Reg(Reg::RAX)));
                    instrs.push(Instr::Pop(Val::Reg(Reg::RDI)));
                    instrs.push(Instr::Add(Val::Reg(Reg::RSP), Val::Imm(offset as i64)));
                },
            }
        },
        // Binary operation performed and result moved to rax
        Expr::BinOp(op, e1, e2) => {
            // Compile inner expression 2 and push to stack
            instrs.append(&mut compile_expr(e2, ctxt, lbl));
            instrs.push(Instr::Mov(Val::MemPtr(Reg::RSP, -ctxt.si*WORD_SIZE), Val::Reg(Reg::RAX)));
            // Compile inner expression 1 into rax
            instrs.append(&mut compile_expr(e1, ExprContext { si: ctxt.si + 1, ..ctxt }, lbl));
            
            // If equality operation, compare types
            if op.get_type() == Op2Type::Equality {
                // Type check instructions with mismatch check
                instrs.push(Instr::Mov(Val::Reg(Reg::RBX), Val::Reg(Reg::RAX)));
                instrs.push(Instr::Xor(Val::Reg(Reg::RBX), Val::MemPtr(Reg::RSP, -ctxt.si*WORD_SIZE)));
                instrs.append(&mut check_msmx(LocPtr::LReg(Reg::RBX)));
            // Otherwise the binary operation is arithmetic, and 
            // we check if both types are numbers
            } else {
                instrs.append(&mut check_msmx(LocPtr::LReg(Reg::RAX)));
                instrs.append(&mut check_msmx(LocPtr::LStack(-ctxt.si*WORD_SIZE)));
            }

            // Match binary operator
            match op {
                // +
                Op2::Plus  => { 
                    instrs.push(Instr::Add(Val::Reg(Reg::RAX), Val::MemPtr(Reg::RSP, -ctxt.si*WORD_SIZE)));
                    instrs.append(&mut check_of()); // Check for overflow
                },
                // -
                Op2::Minus => { 
                    instrs.push(Instr::Sub(Val::Reg(Reg::RAX), Val::MemPtr(Reg::RSP, -ctxt.si*WORD_SIZE)));
                    instrs.append(&mut check_of()); // Check for overflow
                },
                // *
                Op2::Times => {
                    instrs.push(Instr::Sar(Val::Reg(Reg::RAX), Val::Imm(1)));
                    instrs.push(Instr::IMul(Val::Reg(Reg::RAX), Val::MemPtr(Reg::RSP, -ctxt.si*WORD_SIZE)));
                    instrs.append(&mut check_of()); // Check for overflow
                },
                // <, less than
                Op2::Lt => {
                    instrs.push(Instr::Cmp(Val::Reg(Reg::RAX), Val::MemPtr(Reg::RSP, -ctxt.si*WORD_SIZE)));
                    instrs.push(Instr::Mov(Val::Reg(Reg::RAX), Val::Imm(FALSE_VAL)));
                    instrs.push(Instr::Mov(Val::Reg(Reg::RBX), Val::Imm(TRUE_VAL)));
                    instrs.push(Instr::CMovl(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
                },
                // >, greater than
                Op2::Gt => {
                    instrs.push(Instr::Cmp(Val::Reg(Reg::RAX), Val::MemPtr(Reg::RSP, -ctxt.si*WORD_SIZE)));
                    instrs.push(Instr::Mov(Val::Reg(Reg::RAX), Val::Imm(FALSE_VAL)));
                    instrs.push(Instr::Mov(Val::Reg(Reg::RBX), Val::Imm(TRUE_VAL)));
                    instrs.push(Instr::CMovg(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
                },
                // <=, less than or equal to
                Op2::Lte => {
                    instrs.push(Instr::Cmp(Val::Reg(Reg::RAX), Val::MemPtr(Reg::RSP, -ctxt.si*WORD_SIZE)));
                    instrs.push(Instr::Mov(Val::Reg(Reg::RAX), Val::Imm(FALSE_VAL)));
                    instrs.push(Instr::Mov(Val::Reg(Reg::RBX), Val::Imm(TRUE_VAL)));
                    instrs.push(Instr::CMovle(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
                }, 
                // >=, greater than or equal to
                Op2::Gte => {
                    instrs.push(Instr::Cmp(Val::Reg(Reg::RAX), Val::MemPtr(Reg::RSP, -ctxt.si*WORD_SIZE)));
                    instrs.push(Instr::Mov(Val::Reg(Reg::RAX), Val::Imm(FALSE_VAL)));
                    instrs.push(Instr::Mov(Val::Reg(Reg::RBX), Val::Imm(TRUE_VAL)));
                    instrs.push(Instr::CMovge(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
                }, 
                // =, equal to
                Op2::Equal => {
                    instrs.push(Instr::Cmp(Val::Reg(Reg::RAX), Val::MemPtr(Reg::RSP, -ctxt.si*WORD_SIZE)));
                    instrs.push(Instr::Mov(Val::Reg(Reg::RAX), Val::Imm(FALSE_VAL)));
                    instrs.push(Instr::Mov(Val::Reg(Reg::RBX), Val::Imm(TRUE_VAL)));
                    instrs.push(Instr::CMove(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
                },
            }
        },
        // Let bindings evaluated and pushed on stack, and used for main expression
        // Value moved into rax
        Expr::Let(binds, e) => {
            // Build new inner environment
            let mut new_env = ctxt.env.clone();
            let mut sii     = ctxt.si;
            // For loop through all let bindings and evaluate each expression progressively
            for (id, expr) in binds {
                let loc : LocPtr = LocPtr::LStack(-sii*WORD_SIZE);
                instrs.append(&mut compile_expr(expr, ExprContext { si: sii, env: &new_env, ..ctxt }, lbl));
                instrs.push(Instr::Mov(loc.value(), Val::Reg(Reg::RAX)));
                new_env = new_env.update(id.to_string(), loc);
                sii += 1;
            }
            // Evaluate final expression with the new environment
            instrs.append(&mut compile_expr(e, ExprContext { si: sii, env: &new_env, ..ctxt }, lbl));
        },
        // If condition is true, first expression moved to rax
        // Otherwise, second expression moved to rax
        Expr::If(cond_e, e1, e2) => {
            let block_num = *lbl;
            *lbl += 1;
            let else_lbl = format!("else_{}", block_num);
            let endif_lbl = format!("endif_{}", block_num);
            instrs.append(&mut compile_expr(cond_e, ctxt, lbl));
            instrs.push(Instr::Cmp(Val::Reg(Reg::RAX), Val::Imm(FALSE_VAL)));
            instrs.push(Instr::Je(Val::Label(else_lbl.clone())));
            instrs.append(&mut compile_expr(e1, ctxt, lbl));
            instrs.push(Instr::Jmp(Val::Label(endif_lbl.clone())));
            instrs.push(Instr::Label(Val::Label(else_lbl.clone())));
            instrs.append(&mut compile_expr(e2, ctxt, lbl));
            instrs.push(Instr::Label(Val::Label(endif_lbl.clone())));
        },
        // Loop the inner expression infinitely
        Expr::Loop(e) => {
            let new_ctxt = ExprContext { loop_num: *lbl, ..ctxt };
            *lbl += 1;
            let loop_lbl = format!("loop_{}", new_ctxt.loop_num);
            instrs.push(Instr::Label(Val::Label(loop_lbl.clone())));
            instrs.append(&mut compile_expr(e, new_ctxt, lbl));
            instrs.push(Instr::Jmp(Val::Label(loop_lbl.clone())));
            instrs.push(Instr::Label(Val::Label(format!("endloop_{}", new_ctxt.loop_num))));
        },
        // Break out of innermost loop with the following expression
        // moved into rax
        Expr::Break(e) => {
            if ctxt.loop_num > 0 {
                instrs.append(&mut compile_expr(e, ctxt, lbl));
                instrs.push(Instr::Jmp(Val::Label(format!("endloop_{}", ctxt.loop_num))));
            } else {
                panic!("Invalid break outside of loop");
            }
        },
        // Set a let-binding identifier to the result of the expression, by
        // moving rax into the corresponding stack address
        Expr::Set(s, e) => {
            if let Some(loc) = ctxt.env.get(s) {
                instrs.append(&mut compile_expr(e, ctxt, lbl));
                instrs.push(Instr::Mov(loc.value(), Val::Reg(Reg::RAX)));
            } else {
                panic!("Unbound variable identifier {}", s);
            }
        },
        // Set a tuple's element at a certain index to a new expression value
        Expr::TSet(e_tuple, e_index, e_value) => {
            // Evaluate index
            instrs.append(&mut compile_expr(e_index, ctxt, lbl));

            // Perform type check for rax (index)

            // Convert index to correct iterator and store on stack
            instrs.push(Instr::IMul(Val::Reg(Reg::RAX), Val::Imm((WORD_SIZE/2).into())));
            instrs.push(Instr::Add(Val::Reg(Reg::RAX), Val::Imm(WORD_SIZE.into())));
            instrs.push(Instr::Mov(Val::MemPtr(Reg::RSP, -ctxt.si*WORD_SIZE), Val::Reg(Reg::RAX)));

            // Evaluate value expression and store on stack
            instrs.append(&mut compile_expr(e_value, ExprContext { si: ctxt.si + 1, ..ctxt }, lbl));
            instrs.push(Instr::Mov(Val::MemPtr(Reg::RSP, -(ctxt.si+1)*WORD_SIZE), Val::Reg(Reg::RAX)));

            // Store heap pointer to tuple in rax
            instrs.append(&mut compile_expr(e_tuple, ExprContext { si: ctxt.si + 2, ..ctxt }, lbl));
            instrs.push(Instr::Mov(Val::Reg(Reg::RBX), Val::Reg(Reg::RAX)));
            
            // Perform type check for rax (tuple)

            // Get heap pointer from tuple representation
            instrs.push(Instr::Sub(Val::Reg(Reg::RAX), Val::Imm(1)));

            // perform out-of-bounds check (>= len)

            // shift memory pointer
            instrs.push(Instr::Add(Val::Reg(Reg::RAX), Val::MemPtr(Reg::RSP, -ctxt.si*WORD_SIZE)));

            // update value
            instrs.push(Instr::Mov(Val::Reg(Reg::RCX), Val::MemPtr(Reg::RSP, -(ctxt.si+1)*WORD_SIZE)));
            instrs.push(Instr::Mov(Val::MemPtr(Reg::RAX, 0), Val::Reg(Reg::RCX)));

            // move tuple value representation to rax
            instrs.push(Instr::Mov(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
        },
        // Get a tuple's element at a certain index
        Expr::TGet(e_tuple, e_index) => {
            // Evaluate index
            instrs.append(&mut compile_expr(e_index, ctxt, lbl));

            // Perform type check for rax (index)

            // Convert index to correct iterator and store on stack
            instrs.push(Instr::IMul(Val::Reg(Reg::RAX), Val::Imm((WORD_SIZE/2).into())));
            instrs.push(Instr::Add(Val::Reg(Reg::RAX), Val::Imm(WORD_SIZE.into())));
            instrs.push(Instr::Mov(Val::MemPtr(Reg::RSP, -ctxt.si*WORD_SIZE), Val::Reg(Reg::RAX)));

            // Store tuple in rax
            instrs.append(&mut compile_expr(e_tuple, ExprContext { si: ctxt.si + 1, ..ctxt }, lbl));

            // Perform type check for rax (tuple)

            // Get heap pointer from tuple representation
            instrs.push(Instr::Sub(Val::Reg(Reg::RAX), Val::Imm(1)));

            // perform out-of-bounds check (>= len)

            // shift memory pointer
            instrs.push(Instr::Add(Val::Reg(Reg::RAX), Val::MemPtr(Reg::RSP, -ctxt.si*WORD_SIZE)));

            // get value
            instrs.push(Instr::Mov(Val::Reg(Reg::RAX), Val::MemPtr(Reg::RAX, 0)));
        },
        // A block of expressions each evaluated on its own, with the
        // value of the last expression moved to rax
        Expr::Block(exprs) => {
            for e in exprs {
                instrs.append(&mut compile_expr(e, ctxt, lbl));
            }
        },
        // A call to an internal Snek function, which requires
        // a mis-aligned stack before the call
        Expr::Call(fname, exprs) => {
            if let Some(n) = ctxt.func_map.get(fname) {
                if *n != exprs.len() as i32 {
                    panic!("Incorrect number of function parameters for \'{}\'", fname);
                }
            } else {
                panic!("Unknown function \'{}\'", fname);
            }
            let offset = if (exprs.len() as i32+ctxt.si) % 2 == 0 { 0 } else { 1 };
            let mut sii     = ctxt.si+offset;
            for e in exprs.iter().rev() {
                let loc : LocPtr = LocPtr::LStack(-sii*WORD_SIZE);
                instrs.append(&mut compile_expr(e, ExprContext { si: sii, ..ctxt }, lbl));
                instrs.push(Instr::Mov(loc.value(), Val::Reg(Reg::RAX)));
                sii += 1;
            }
            sii -= 1;
            instrs.push(Instr::Sub(Val::Reg(Reg::RSP), Val::Imm((sii*WORD_SIZE) as i64)));
            instrs.push(Instr::Call(Val::Label(fname.clone())));
            instrs.push(Instr::Add(Val::Reg(Reg::RSP), Val::Imm((sii*WORD_SIZE) as i64)));
        }
    }
    instrs
}

/// Compile the function into a vector of instructions, including the header label and
/// the ending ret instruction
fn compile_func(func: &Function, func_map: &HashMap<String,i32>, lbl: &mut i32) -> Vec<Instr> {
    let mut instrs : Vec<Instr> = Vec::new();
    let mut vars : HashMap<String, LocPtr> = HashMap::new();

    // Add each function parameter into the variable environment (scope)
    let mut sii = 1;
    for arg in &func.args {
        vars.insert(arg.to_string(), LocPtr::LStack(sii*WORD_SIZE));
        sii += 1;
    }

    // Add the "label: " assembly label
    instrs.push(Instr::Label(Val::Label(func.name.clone())));

    // Compile the inner expression
    let ctxt = ExprContext { si: 1, env: &vars, loop_num: 0, func_map: &func_map , in_func: true };
    instrs.append(&mut compile_expr(&func.body, ctxt, lbl));

    // ret instruction
    instrs.push(Instr::Ret);
    instrs
}

/// Compile a program into a String containing all functions represented
/// in assembly instructions and a String containing the main expression represented
/// in assembly instructions
pub fn compile(prog: &Program) -> (String, String) {
    let mut lbl = 1; // generator for unique label numbers
    let mut func_map : HashMap<String,i32> = HashMap::new();    // represents list of function names

    // Insert all function names into the function map for future internal function calls
    for func in &prog.defns {
        func_map.insert(func.name.clone(),func.args.len() as i32);
    }

    // Compile each function and append them together into a single instruction vector
    let mut defn_instrs = Vec::new();
    for func in &prog.defns {
        let mut c_instrs = compile_func(func, &func_map, &mut lbl);
        defn_instrs.append(&mut c_instrs);
    }
    
    // Compile the main expression
    let ctxt = ExprContext { si: 1, env: &HashMap::new(), loop_num: 0, func_map: &func_map, in_func: false };
    let main_instrs = compile_expr(&prog.main, ctxt, &mut lbl);
    
    // Convert each vector of instructions into Strings and return the tuple
    (to_asm(&defn_instrs), to_asm(&main_instrs))
}