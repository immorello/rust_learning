# Calculator

First Rust exercise in this repository.

## Goal

Build a terminal calculator that:

- asks how many numbers will be used
- reads the numbers from user input
- asks for an operator
- performs the calculation
- prints either the result or an error

## Features

- addition
- subtraction
- multiplication
- division

## Rust concepts practiced

- `Vec<i32>`
- slices with `&[i32]`
- `enum`
- `match`
- `Result`
- terminal input with `stdin`
- basic ownership and borrowing

## Design notes

- `main.rs` handles the program flow and user interaction
- `lib.rs` contains the calculator logic
- the operator is converted from `char` into an `Operator` enum
- errors are handled with `Result` instead of panicking

## Possible improvements

- add automated tests
- support `f64`
- allow multiple calculations in a loop
- improve error messages
- support more complex expressions

## License

This subproject is licensed under the Apache License, Version 2.0.
Redistributions and derivative works must preserve the `NOTICE`
attribution file as required by the license.
