# Rascal

A Pascal(ish) compiler, written in Rust, implemented with top-down recursive descent parsing.  

This compiler targets a custom virtual stack machine, `RVM`, which is included here within `src/rvm/mod.rs`. Future work may include adding a real-world compiler backend to target something like WebAssembly or the Java VM.

This project was created as an exercise to learn about compiler theory, design, and implementation. As such, it generates naive code and lacks certain features.

## Setup and Run

* Set up a Rust development environment by running the official Rustup installer script, located here: https://rustup.rs/#
* Open a command line terminal in the project's root directory, and run `cargo run samples/bubblesort.pas`, or replace `bubblesort.pas` with any of the other sample source files.
* Rascal will parse the source file, generate RVM bytecode, and then immediately execute the resulting bytecode at once.


## Language Features

* [x] Expression parsing & evaluation
* [x] Variable declarations & assignments
* [x] `write` system procedure for stdout
* [x] Loop structures:
  - [x] `repeat`
  - [x] `while`
* [ ] Control structures:
  - [x] `if`
  - [ ] `switch`
* [x] Arrays
* [x] Void procedures
* [ ] `goto` statements

## Notable Sample Programs

* `samples/findmax.pas` -> This program implements an algorithm for locating the largest element of an array.
* `samples/bubblesort.pas` -> This program implements the bubblesort algorithm to sort an array. The program prints out the integer literal `888888888` as a make-shift separator between the before and after array print outs. This example showcases nested procedure calls.

## Current Constraints

No real type system -- all values are currently constrained to unsigned 32-bit integers. Currently does not support floats, chars, or booleans (conditions are evaluated by comparing zero and non-zero values). As a result, the global/system procedure `write()` only prints out unsigned integers as well.

All `if`-statements require surrounding `begin` and `end;` blocks, even if they contain only one statement.

Only void procedures with no parameters are supported. Procedures don't have local variables, they are all global/statically defined in the declarations section.
