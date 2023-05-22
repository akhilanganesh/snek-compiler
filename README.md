# Compiler for the Egg-Eater Snek Language

Project by Akhilan Ganesh.

The Snek language is a language defined by the CSE 131: Compiler Construction course
(Spring 2023, Professor Joe Politz). In this course, we design progressively a compiler
for the ever-more-complex Snek language, which includes values, unary operations, binary
operations, conditional expressions and control logic, and functions. This particular compiler
compiles the egg-eater Snek language, which is one of the more complex versions of this language.

The Egg-Eater version builds on previous versions by adding a tuple value, and by implementing heap
data. Previously, all values were stored exclusively on the stack.

This repository is a Rust cargo project implemented with the Rust language. 

## Snek Language

The Snek language is an s-expression language. The concrete syntax of this language is specified in 
`doc/specification.md`.

## Credits

Special thanks to Professor Joe Politz, TAs Nico Lehmann, Rachel Lim, Abhishek Sharma, Ruochen Wang, and tutors Mark Barbone, Shubham Bhargava, for designing this course and the Snek language to meet my educational needs. This project was a great hands-on experience in learning compiler construction.