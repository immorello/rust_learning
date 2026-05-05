# Key Value Store

Third Rust exercise in this repository.

## Goal

Build a small in-memory key-value store for the terminal.

This exercise is meant to bridge the gap between beginner Rust projects and a future database-oriented project.

## What the program should do

The program should:

1. start by showing the available commands
2. stay alive in a loop
3. read a command from the user
4. parse the command
5. execute the requested operation
6. exit only when the user types `quit`

## Recommended data structure

Use a `HashMap<String, String>`.

Recommended shape:

```rust
struct KvStore {
    data: HashMap<String, String>,
}
```

## Minimum required commands

### `set <key> <value>`

Store a key-value pair.

Example:

```text
set name Mario
```

If the key already exists, update the value.

### `get <key>`

Return the value associated with a key.

Example:

```text
get name
```

Expected output:

```text
Mario
```

If the key does not exist:

```text
Key not found
```

### `delete <key>`

Remove the key and its value.

### `list`

Print all stored key-value pairs.

If the store is empty:

```text
Store is empty
```

### `exists <key>`

Tell whether a key exists.

Output:

```text
true
```

or:

```text
false
```

### `quit`

Exit the program.

## Implementation requirements

- use `HashMap<String, String>`
- separate `main.rs` and `lib.rs`
- do not panic on invalid input
- handle missing keys gracefully
- keep the program alive in a loop

## Recommended organization

### In `main.rs`

- main loop
- input reading
- command parsing
- calls into the store methods

### In `lib.rs`

- `KvStore`
- methods such as `set`, `get`, `delete`, `list`, and `exists`

## Rust concepts practiced

- `HashMap`
- `struct`
- `impl`
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
3. it uses `HashMap<String, String>` as in-memory storage
4. it returns to the prompt after each command
5. it handles invalid input without crashing

## Optional bonus features

- `count`
- `clear`
- support for values containing spaces
- automated tests
- file persistence in a later version

## Why this matters

This exercise practices the core building blocks of a simple database engine:

- key-based lookup
- insert
- update
- delete
- in-memory state
- command parsing
