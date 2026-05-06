# Key Value Store

Third Rust exercise in this repository.

## Goal

Build a small in-memory key-value store for the terminal.

This exercise is meant to bridge the gap between beginner Rust projects and a future database-oriented project.

## What the program should do

The program should:

1. start by showing the available commands
2. stay alive in a loop
3. let the user choose a command from a menu
4. read the extra input required by that command
5. execute the requested operation
6. exit only when the user chooses `quit`

## Recommended data structure

Use a `HashMap<String, Value>`.

Recommended shape:

```rust
enum Value {
    Integer(i32),
    Float(f64),
    Text(String),
    Boolean(bool),
}

struct Store {
    data: HashMap<String, Value>,
}
```

## Minimum required commands

### `set`

Ask the user for:

- a key
- a value

If the key already exists, update the value.

The value should be parsed into one of the supported types:

- integer
- float
- boolean
- text

### `get`

Ask the user for a key and return the value associated with it.

If the key does not exist:

```text
Key not found
```

### `delete`

Ask the user for a key and remove the key-value pair.

### `list`

Print all stored key-value pairs.

If the store is empty:

```text
Store is empty
```

### `quit`

Exit the program.

## Implementation requirements

- use `HashMap<String, Value>`
- separate `main.rs` and `lib.rs`
- do not panic on invalid input
- handle missing keys gracefully
- keep the program alive in a loop

## Recommended organization

### In `main.rs`

- main loop
- input reading
- menu selection parsing
- calls into the store methods

### In `lib.rs`

- `Store`
- `Value`
- methods such as `set`, `get`, `delete`, and `list`
- helper functions for input parsing

## Rust concepts practiced

- `HashMap`
- `struct`
- `impl`
- custom `enum` for typed values
- `Option`
- `Result`
- string parsing
- `match`
- ownership and borrowing
- basic iterators

## Definition of done

The exercise is complete when:

1. the program starts correctly
2. it supports at least `set`, `get`, `delete`, `list`, and `quit`
3. it uses `HashMap<String, Value>` as in-memory storage
4. it returns to the prompt after each command
5. it handles invalid input without crashing

## Optional bonus features

- `exists`
- `count`
- `clear`
- automated tests
- file persistence in a later version

## Why this matters

This exercise practices the core building blocks of a simple database engine:

- key-based lookup
- insert
- update
- delete
- in-memory state
- value typing
- command parsing
