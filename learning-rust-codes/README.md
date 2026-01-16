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
```rust
# No need to do this because Rust has predule
  use std::string;
  let mut guess = string::String::new();
# Just do this instead
  let mut guess = String::new();
```

## The :: syntax

```bash
# Variables are immutable by default, to make a variable mutable, we add mut
# Creates a mutable variable that is currently bound to a new, empty instance of a String
let mut guess = String::new();
G
# Is a string type that is a GROWABLE, UTF-8 encoded bit of text.
String

# Indicates that 'new' is an associated function of the String type.
# An associated function is a function that’s implemented on a type, in this case String.
::new
```


## User Input

```bash
# We can import the io module and use it
  use std::io;
  io::stdin().read_line(...
# or we could still use the function directly by writing this
  std::io::stdin().read_line(...

# This function returns an instance of std::io::Stdin which is a type
# that represents a handle to the standard input for your terminal
stdin()

# Method on the standard input handle to get input from the user
# take whatever the user types into standard input and append that into a string
.read_line(...)

# Argument to read_line() to tell it what string to store the user input in
&mut guess

# It's a method of the returned 'Result' value from read_line(), it's an enumeration (enum)
# It's a type that can be in one of multiple possible states. 
# We call each possible state a variant. Result’s variants are Ok and Err.
# Has an 'expect' method, if Result is an 'Err' value, the program crash and display the message
# If Result is an Ok value, 'expect' will take the return value that Ok is holding and return
# just that value to use it, the value is the number of bytes in the user’s input.
.expect("Failed to read line");
```

## &
```bash
# & indicates that this argument is a reference
# Let multiple parts of the code access one piece of data without needing to copy it.
# Like variables, references are immutable by default.
# Need '&mut guess' rather than &guess to make it mutable.
&mut guess
```

## Dependencies
From Crates.io, to Cargo.toml file
```bash
[dependencies]
rand = "0.8.5"  # Means ^0.8.5, this is, at least 0.8.5 but below 0.9.0
```




