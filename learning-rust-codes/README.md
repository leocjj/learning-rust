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
- **else**: Fallback for if and if let control flow constructs.
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
e.g.: String, without prelude we should do:
```bash
# No need to do this because Rust has prelude
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

# Is a string type that is a GROWABLE, UTF-8 encoded bit of text.
String

# :: indicates that 'new' is an associated function of the String type.
# An associated function is a function that’s implemented on a type, in this case String.
::new
```


## User Input

```bash
# We can import the io module and use it
use std::io;
io::stdin().read_line(&mut guess);
# or we could still use the function directly by writing this
std::io::stdin().read_line(&mut guess).expect("Failed to read line");

# This function returns an instance of std::io::Stdin which is a type
# that represents a handler to the standard input for your terminal
stdin()

# Method on the standard input handle to get input from the user
# take whatever the user types into standard input and append that into a string
.read_line(&mut guess);

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
# Use caret ^, tilde ~, or comparison operators >=, =
rand = "0.8.5"  # Means ^0.8.5, this is, at least 0.8.5 but below 0.9.0
```

## Cargo.lock

- 'cargo build' creates the Cargo.lock file the first time, next time,
- 'cargo build' will use Cargo.lock file rather than figure it out versions again.
- Commit this file into source control with the rest of the code.
- This is a reproducible build. The project will remain at 0.8.5 until explicitly upgraded.

- To update a crate use 'cargo update', this ignores the Cargo.lock file and figure out
  all the latest versions that fit your specifications in Cargo.toml.
- Cargo will then write those versions to the Cargo.lock file.

## Cargo commands:
    new         Create a new cargo package
    init        Create a new cargo package in an existing directory

    build, b    Compile the current package
    run, r      Run a binary or example of the local package
    check, c    Analyze the current package and report errors, but don't build object files

    add         Add dependencies to a manifest file,        e.g. cargo add rand@=0.8.5
    remove      Remove dependencies from a manifest file,   e.g. cargo remove rand
    update      Update dependencies listed in Cargo.lock,
    tree        Display a tree visualization of a dependency graph

    doc --open  Build documentation locally for all dependencies and open it in browser

## Traits

The Rng trait defines methods that random number generators implement
```bash
# This trait must be in scope to use its methods.
rand::Rng;

let secret_number = rand::thread_rng().gen_range(1..=100);

# Function that gives random number generator, local to the current thread, is seeded by OS.
rand::thread_rng

# Method on the random number generator, defined by the Rng trait.
gen_range 

# Range expression inclusive on the lower and upper bounds
start..=end  # e.g. 1..=100
```

## Result type
```bash
# OPTION 1:
# parse() method returns a Result type -> use expect() to handle it.
# If parse returns an Err Result variant
#   expect() call will crash the game and print the message we give it.
# If parse return the Ok Result variant
#   expect() will return the number from the Ok value.
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

```bash
# OPTION 2:
# Handle potential parse errors using a match expression without crashing
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("Please type a valid number!");
        continue;
    }
};
```

# 3. Common Programming Concepts

## Variables and Mutability

By default, variables are immutable

### Constants
  Use the *const* keyword, and the type of the value must be annotated.
  const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

  - They’re always immutable.
  - Can be declared in any scope, including the global scope (variables don't).
  - Are valid for the entire time a program runs, within the scope in which they were declared.
  - May be set only to a constant expression, not the result of a value that could only be computed at runtime.

### Shadowing

  Reuse the same variable name even with different types, second overshadows the first

    let spaces = "   ";
    let spaces = spaces.len();

### Types

All variables need a type at compile time. If not obvious, type should be defined:
```bash
let guess: u32 = "42a".parse().expect("Not a number!");
```

Length	  Signed	Unsigned
8-bit	    i8	    u8
16-bit	  i16	    u16
32-bit	  i32	    u32
64-bit	  i64	    u64
128-bit 	i128	  u128
Architecture-dependent	isize	usize

Note that number literals that can be multiple numeric types allow a type suffix, such as 57u8, to designate the type.

let num1 = 43u16;

Number literals	Example
Decimal	        98_222
Hex	            0xff
Octal	          0o77
Binary	        0b1111_0000
Byte (u8 only)  b'A'







































