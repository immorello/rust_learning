# Rust Learning Path

This repository tracks a hands-on Rust learning path built around small projects that gradually move toward a larger systems goal.

## Final goal

The long-term objective is to build a key-value database engine in Rust.

That means the learning path is not only about syntax. It is meant to build confidence with:

- ownership and borrowing
- data modeling with `struct` and `enum`
- error handling with `Option` and `Result`
- in-memory data structures
- file persistence
- parsing commands and APIs
- testing
- concurrency later on

## Learning approach

Each project should introduce one or two new ideas without making the whole exercise too complex.

The sequence is designed to move from:

1. simple terminal programs
2. state held in memory
3. richer data structures
4. persistence to disk
5. database-style behavior

## Project roadmap

### 1. Calculator

Focus:

- terminal input
- `Vec`
- `enum`
- `match`
- `Result`

Why it matters:

This project builds the foundation for input handling, control flow, and basic separation between `main.rs` and `lib.rs`.

### 2. Todo List

Focus:

- `struct`
- `impl`
- `Vec<T>`
- mutable and immutable borrowing
- basic program state

Why it matters:

This project introduces application state and methods that act on owned data.

### 3. Key Value Store

Focus:

- `HashMap`
- parsing command strings
- key-based lookup
- insert, update, delete
- cleaner API design

Why it matters:

This is the first exercise that directly resembles a database core.

### 4. Persistent Store

Focus:

- file I/O with `std::fs`
- saving state to disk
- loading state at startup
- `Result`-based error handling for files
- separating business logic from terminal interaction

Why it matters:

This project turns the in-memory store into something closer to a real system by making data survive across program runs.

## Likely next steps after the persistent store

Once the persistent key-value store is complete, the next exercises should move toward:

1. append-only logs
2. document-like values instead of plain strings
3. indexing ideas
4. concurrency and synchronization
5. exposing the engine through a CLI or HTTP interface

## Repository purpose

This repository is both:

- a place to write Rust code
- a written record of the learning progression

Each project folder should keep its own `README.md` describing:

- the goal of the exercise
- the concepts being practiced
- possible next improvements

## License

This repository and its subprojects are licensed under the Apache License,
Version 2.0. See `LICENSE` for the license terms and `NOTICE` for the
attribution notice that must be preserved in redistributions and derivative
works as required by the license.
