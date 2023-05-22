# Implementation Details

Here I'll note some interesting and relevant implementation details of my compiler, which are design choices and do not reflect a change in the core syntax and functionality underlying the language.

## Stack Alignment

For C calling conventions, the stack must be 16-byte aligned. Since the word size is 8 bytes, there is a possibility of the stack being misaligned before a call to the external C functions (e.g. `snek_error` or `snek_print`). This must be handled accordingly. Additionally, because the `our_code_starts_here` function stores the return address on the stack, the stack starts out misaligned (8 bytes extra).

The current implementation assumes that each expression must be built from an 8-byte misaligned stack. Expressions may push additional values on top of the stack, and so any external calls must be aware of the stack pointer at all times. Then, before all calls to external C functions, to maintain C calling convention, if the stack is at that time misaligned, we subtract and additional 8 from the stack pointer register `rsp` to re-align the stack. *(Note: before `snek_error` we simply always subtract 8 from the original/base stack pointer since we do not intend to return to the Snek runtime environment.)*

However, for all calls to internal Snek functions, we need to re-assert the underlying assumption of an 8-byte misaligned stack. Therefore, we force an 8-byte mis-alignment again by subtracting an additional 8 bytes if the stack is 16-byte aligned.