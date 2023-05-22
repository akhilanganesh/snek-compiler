/*
    start.rs

    Compiles a Snek assembly file into a runtime Snek binary.
*/
use std::env;
use std::convert::TryInto;

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

/// size of heap in bytes
const HEAP_SIZE : usize = 1000000;

// Links the "our_code_starts_here" function to the C binary
#[link(name = "our_code")]
extern "C" {
    // The \x01 here is an undocumented feature of LLVM that ensures
    // it does not add an underscore in front of the name.
    // Courtesy of Max New (https://maxsnew.com/teaching/eecs-483-fa22/hw_adder_assignment.html)
    #[link_name = "\x01our_code_starts_here"]
    fn our_code_starts_here(input: u64, memory: *mut u64) -> u64;
}

/// Exported external C function for the runtime environment
/// that reports an error and aborts the process
#[no_mangle]
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
#[no_mangle]
#[export_name = "\x01snek_print"]
pub extern "C" fn snek_print(val: u64) {
    println!("{}", snek_string(val));
}

/// Exported external C function for the runtime environment
/// that returns the String representation of an internal value
// #[export_name = "\x01snek_string"]
pub fn snek_string(val: u64) -> String {
    // Match the internal value representation type
    match val as i64 {
        FALSE_VAL => String::from("false"),
        TRUE_VAL => String::from("true"),
        _ if val % 4 == 1 => {
            let mut output = String::new();
            let ptr = (val - 1) as *const u64;
            let len = unsafe { *ptr };
            if len > 0 {
                for index in 1..len {
                    output += &(snek_string(unsafe { *ptr.offset(index.try_into().unwrap()) }) + ", ");
                }
                output += &snek_string(unsafe { *ptr.offset((len).try_into().unwrap()) });
            }
            
            format!("[{output}]")
        }
        _ => format!("{}", (val as i64) >> 1),
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

// fn mem_allocate(capacity: usize) -> *mut u64 {
    
// }

/// Collects input argument and runs the snek binary, printing
/// the output
fn main() {
    let args: Vec<String> = env::args().collect();

    // set input to first argument
    // if first argument is non-existent, set input to false
    let input = if args.len() == 2 { &args[1] } else { "false" };
    let input = parse_input(&input);

    // allocate heap memory
    let mut memory = Vec::<u64>::with_capacity(HEAP_SIZE);
    let heap : *mut u64 = memory.as_mut_ptr();// mem_allocate(HEAP_SIZE);

    let i: u64 = unsafe { our_code_starts_here(input, heap) };
    snek_print(i);
}
