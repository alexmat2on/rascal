# Rascal

A Pascal(ish) compiler, written in Rust, implemented with top-down recursive
descent parsing.  

This compiler targets a custom virtual stack machine, `RVM`, which is included
here within `src/rvm/mod.rs`. Future work may include adding a real-world compiler backend
to target something like WebAssembly or the Java VM.

## Setup and Run

* Set up a Rust development environment by running the official Rustup installer
script, located here: https://rustup.rs/#
* Open a command line terminal in the project's root directory, and run
`cargo run samples/variables.pas`, or replace `variables.pas` with any of the
other sample source files.
* Rascal will parse the source file, generate RVM bytecode, and then
immediately execute the resulting bytecode at once.


## Language Features

* [x] Expression parsing & evaluation
* [x] Variable declarations & assignments
* [x] Loop structures:
  - [x] `repeat`
  - [x] `while`
* [ ] Control structures:
  - [ ] `if`
  - [ ] `switch`
* [ ] Void procedures
* [ ] Arrays
* [ ] `goto` statements


## Current Constraints

No real type system -- all values are currently constrained to
unsigned 32-bit integers. Currently does not support floats, chars, or booleans (conditions are evaluated by comparing zero and non-zero values).
