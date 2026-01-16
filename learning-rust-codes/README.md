# Appendix A - Keywords Currently in Use
The following is a list of keywords currently in use, with their functionality described.

- **as**: Perform primitive casting, disambiguate the specific trait containing an item, or rename items in use statements.
- **async**: Return a Future instead of blocking the current thread.
- **await**: Suspend execution until the result of a Future is ready.
- **break**: Exit a loop immediately.
- **const**: Define constant items or constant raw pointers.
- **continue**: Continue to the next loop iteration.
- **crate**: In a module path, refers to the crate root.
- **dyn**: Dynamic dispatch to a trait object.
-else**: Fallback for if and if let control flow constructs.
- **enum**: Define an enumeration.
- **extern**: Link an external function or variable.
- **false**: Boolean false literal.
- **fn**: Define a function or the function pointer type.
- **for**: Loop over items from an iterator, implement a trait, or specify a higher ranked lifetime.
- **if**: Branch based on the result of a conditional expression.
- **impl**: Implement inherent or trait functionality.
- **in**: Part of for loop syntax.
- **let**: Bind a variable.
- **loop**: Loop unconditionally.
- **match**: Match a value to patterns.
- **mod**: Define a module.
- **move**: Make a closure take ownership of all its captures.
- **mut**: Denote mutability in references, raw pointers, or pattern bindings.
- **pub**: Denote public visibility in struct fields, impl blocks, or modules.
- **ref**: Bind by reference.
- **return**: Return from function.
- **Self**: A type alias for the type we are defining or implementing.
- **self**: Method subject or current module.
- **static**: Global variable or lifetime lasting the entire program execution.
- **struct**: Define a structure.
- **super**: Parent module of the current module.
- **trait**: Define a trait.
- **true**: Boolean true literal.
- **type**: Define a type alias or associated type.
- **union**: Define a union; is a keyword only when used in a union declaration.
- **unsafe**: Denote unsafe code, functions, traits, or implementations.
- **use**: Bring symbols into scope.
- **where**: Denote clauses that constrain a type.
- **while**: Loop conditionally based on the result of an expression.


# Appendix B - Operators

https://doc.rust-lang.org/book/appendix-02-operators.html

---
---


# 1. Getting Started

## **00_hello_world/**
```bash
rustc main.rs
./main
```

## **hello_cargo/**

Rust’s build system and package manager. Comes installed with Rust (packages of code are referred to as crates).
```bash
cargo --version
```

### Creating a Project

https://doc.rust-lang.org/cargo/

```bash
cargo new hello_cargo  # creates a new directory and project
cd hello_cargo
  or
cargo init  # In a folder, this will create a project automatically.

cargo build  # Creates an executable file in target/debug/hello_cargo,
             # because the default build is a debug build
./target/debug/hello_cargo  # Runs the executable file.

cargo run  # Compiles the code and then run the executable.

cargo check  # Checks code to make sure it compiles, doesn’t produce an executable

cargo build --release  # Compile with optimizations.
                       # Creates an executable in target/release
```

# 2. Programming a Guessing Game

## Prelude
Automatically imports into every Rust program, no need to import them
https://doc.rust-lang.org/std/prelude/index.html
e.g.: String, whithout prelude we should do:
```bash
# No need to do this because Rust has predule
  use std::string;
  let mut guess = string::String::new();
# Just do this instead
  let mut guess = String::new();
