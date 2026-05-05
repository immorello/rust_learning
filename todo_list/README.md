# Todo List

Second Rust exercise in this repository.

## Goal

Build a terminal todo list that stays alive in a loop and manages tasks in memory.

## Features

- add a task
- show a task or list all tasks
- complete a task
- delete a task
- quit the program

## Overall structure

- `Task`: represents a single task
- `TodoList`: owns the `Vec<Task>` and the related methods
- `Choice`: enum used for menu options
- `main.rs`: handles the menu and user input
- `lib.rs`: contains the data structures and logic

## Rust concepts practiced

- `struct`
- `impl`
- `Vec<T>`
- `enum`
- `match`
- `Option`
- `Result`
- methods with `&self` and `&mut self`
- iterator methods such as `find`, `position`, and `iter_mut`

## Important decisions

- the data lives only in memory
- when the program exits, the tasks are lost
- `Task` can remain private if it is managed only through `TodoList`
- the menu runs inside a `loop` and exits only on `quit`

## Possible improvements

- add automated tests
- use incremental ids instead of random ids
- save data to a file
- clean up error messages
- separate terminal I/O from business logic more strictly
