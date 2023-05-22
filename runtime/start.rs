/*
    start.rs

    Compiles a Snek assembly file into a runtime Snek binary.
*/
use std::env;

/// mismatch error code
const MSMX_ERRCODE : i64 = 7; // msmx = mismatch

/// overflow error code
const OF_ERRCODE : i64 = 8;

/// true  value representation (code + tag)
const TRUE_VAL  : i64 = 7;

/// false value representation (code + tag)
const FALSE_VAL : i64 = 3;

/// limit to Snek integers (2^62)
const LIM : i64 = 4611686018427387904;

// Links the "our_code_starts_here" function to the C binary
#[link(name = "our_code")]
extern "C" {
    // The \x01 here is an undocumented feature of LLVM that ensures
    // it does not add an underscore in front of the name.
    // Courtesy of Max New (https://maxsnew.com/teaching/eecs-483-fa22/hw_adder_assignment.html)
    #[link_name = "\x01our_code_starts_here"]
    fn our_code_starts_here(input: u64) -> u64;
}

/// Exported external C function for the runtime environment
/// that reports an error and aborts the process
#[export_name = "\x01snek_error"]
pub extern "C" fn snek_error(errcode: i64) {
    // Print error message according to error code
    match errcode {
        MSMX_ERRCODE => { eprintln!("Operation with invalid argument(s)"); }
        OF_ERRCODE => { eprintln!("Operation caused arithmetic overflow"); }
        _ => { eprintln!("An error occurred {errcode}"); }
    }

    // Abort the process immediately
    std::process::exit(1);
}

/// Exported external C function for the runtime environment
/// that prints out the String representation of an internal value
#[export_name = "\x01snek_print"]
pub extern "C" fn snek_print(val: u64) {
    // Match the internal value representation type
    match val as i64 {
        FALSE_VAL => println!("false"),
        TRUE_VAL => println!("true"),
        _ => println!("{}", (val as i64) >> 1),
    }
}

/// Parses input string into an internal value representation
fn parse_input(input: &str) -> u64 {
    // Match input to the various possible String representations
    match input {
        "true" => TRUE_VAL as u64,     // true  => true value representation
        "false" => FALSE_VAL as u64,    // false => false value representation
        _ => {
            // If number, check if within bounds
            // Multiply by 2 to get internal value representation
            if let Ok(n) = input.parse::<i64>() {
                if n > LIM - 1 || n < -1 * LIM {
                    panic!("Invalid input")
                } else {
                    (n*2) as u64
                }
            } else {
                panic!("Invalid input")
            }
        }
    }
}

/// Collects input argument and runs the snek binary, printing
/// the output
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() == 2 { &args[1] } else { "false" };
    let input = parse_input(&input);

    let i: u64 = unsafe { our_code_starts_here(input) };
    snek_print(i);
    // println!("{i}");
}
