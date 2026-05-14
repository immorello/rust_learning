# Persistent Store

Fourth Rust exercise in this repository.

## Goal

Build a small persistent key-value store for the terminal.

This exercise extends the in-memory key-value store by making the data survive between program runs.

## Evolution

This project is intended to evolve in small internal versions instead of being replaced immediately by a separate exercise.

Current and planned progression:

1. `v0`: copied baseline from the in-memory key-value store
2. `v1`: persistence with JSON save/load and full snapshot after each write
3. `v2`: split the code into modules so storage logic, persistence, application orchestration, and terminal interaction are easier to follow
4. `v3`: replace JSON persistence with a Protocol Buffers binary snapshot format
5. `v4`: improve persistence strategy with better snapshot timing or an append-only log plus snapshots

Git commits and tags should be used to mark these stages.

## Current Status

The current implementation now includes:

- modular separation between `store`, `persistence`, `app`, `cli`, and `errors`
- full snapshot persistence using Protocol Buffers
- loading state at startup and saving after write operations
- a protobuf roundtrip test that checks conversion between internal Rust types and persisted binary data

This exercise has reached its intended learning milestone and can be considered complete for the Rust learning path.

## What the program should do

The program should:

1. start by loading previously saved data from disk
2. show the available commands
3. stay alive in a loop
4. let the user choose a command from a menu
5. read the extra input required by that command
6. execute the requested operation
7. save changes to disk
8. exit only when the user chooses `quit`

## Recommended data structure

Use a `HashMap<String, Value>` inside a `Store`.

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

Keep the in-memory representation simple. The new part of this exercise is persistence, not a more advanced data model.

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

After updating the store, persist the new state to disk.

### `get`

Ask the user for a key and return the value associated with it.

If the key does not exist:

```text
Key not found
```

### `delete`

Ask the user for a key and remove the key-value pair.

If the key exists, persist the updated state to disk.

### `list`

Print all stored key-value pairs.

If the store is empty:

```text
Store is empty
```

### `quit`

Exit the program.

## Persistence requirements

The program must save data to a file and load it again at startup.

You can choose one of these approaches:

1. save the whole store as a snapshot after every change
2. save the whole store only on `quit`

For this exercise, the snapshot approach after every change is recommended because it is simpler and safer.

The file format does not need to be advanced at first. A simple text or JSON format is a good starting point.

In the current version of this exercise, persistence has already been upgraded to Protocol Buffers to practice schema-based binary serialization.

## Implementation requirements

- use `HashMap<String, Value>` for the in-memory store
- separate responsibilities into modules instead of keeping everything in one file
- do not panic on invalid input
- handle missing keys gracefully
- handle missing files gracefully on first startup
- handle file read and write errors with `Result`
- keep the program alive in a loop
- load data from disk when the program starts
- save data to disk after modifying commands such as `set` and `delete`

## Recommended organization

### In `main.rs`

- main loop
- input reading
- menu selection parsing
- calls into the store methods
- user-facing error messages

### In `lib.rs`

- top-level module wiring
- protobuf generated module inclusion

### In `store.rs`

- `Store`
- `Value`
- methods such as `set`, `get`, `delete`, and `list`

### In `persistence.rs`

- conversions between internal Rust types and protobuf messages
- methods for saving and loading

### In `app.rs`

- application-level orchestration between CLI and store

### In `cli.rs`

- input reading
- command selection
- user-facing output and error printing

## Rust concepts practiced

- `HashMap`
- `struct`
- `impl`
- custom `enum` for typed values
- `Option`
- `Result`
- file I/O with `std::fs`
- schema-based serialization with Protocol Buffers
- module organization
- string parsing
- ownership and borrowing
- error propagation

## Definition of done

The exercise is complete when:

1. the program starts correctly
2. it supports at least `set`, `get`, `delete`, `list`, and `quit`
3. it uses `HashMap<String, Value>` as in-memory storage
4. it loads previous data from a file when starting
5. it saves changes to a file
6. it returns to the prompt after each command
7. it handles invalid input without crashing
8. it handles missing or empty storage files gracefully
9. it successfully roundtrips data through protobuf encoding and decoding

## Optional bonus features

- `exists`
- `count`
- `clear`
- choose the storage file path from the command line
- automated tests for save and load behavior
- append-only log format instead of full snapshots
- richer application-level commands and responses

## Why this matters

This exercise moves your project one step closer to a real database engine:

- state survives program restarts
- disk persistence becomes part of the design
- file errors become part of normal error handling
- loading and saving state introduces database lifecycle thinking
- schema-based serialization introduces a more realistic storage boundary
- the store begins to behave more like a real system than a temporary program

## License

This subproject is licensed under the Apache License, Version 2.0.
Redistributions and derivative works must preserve the `NOTICE`
attribution file as required by the license.
