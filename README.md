# learning-rust path

## Follow the book
https://doc.rust-lang.org/book/
1. ✅ Getting Started
2. ✅ Programming a Guessing Game
3. 🔄 Common Programming Concepts
    1. ✅ Variables and Mutability
    2. ✅ Data types
    3. 🔄 Functions
    4. Comments
    5. Control flow
---
4. Understanding Ownership
5. Using Structs to Structure Related Data
6. Enums and Pattern Matching
---
7. Packages, Crates, and Modules
8. Common Collections
9. Error Handling
---
10. Generic Types, Traits, and Lifetimes
11. Writing Automated Tests
12. An I/O Project: Building a Command Line Program
---
---
13. Functional Language Features: Iterators and Closures
14. More about Cargo and Crates.io
15. Smart Pointers
---
16. Fearless Concurrency
17. Fundamentals of Asynchronous Programming: Async, Await, Futures, and Streams
---
18. Object Oriented Programming Features
19. Patterns and Matching
20. Advanced Features
---
21. Final Project: Building a Multithreaded Web Server
---

## Test knowledge with examples
- https://doc.rust-lang.org/rust-by-example/

- https://rustlings.rust-lang.org/

- https://exercism.org/tracks/rust (easy category)

- https://leetcode.com/

- https://adventofcode.com/

## Learn basic useful libraries

- https://tokio.rs/
- https://github.com/yewstack/yew
- https://github.com/rwf2/Rocket/tree/v0.5.1
- https://github.com/launchbadge/sqlx
- https://github.com/clap-rs/clap
- https://github.com/serde-rs/serde
- https://github.com/kreuzberg-dev/kreuzberg/
- https://github.com/actix/actix
- https://docs.rs/polars/latest/polars/ (Dataframes)
- https://github.com/mstange/samply (Profiler)

## Create basic projects
- Simple UI
- Todo App
- Build an API
- Integrate and API
- CLI App with DB support
- Read code of a public library framework

## Create portfolio based on:
- Relevance:
    - High-performance CLI tool
    - High-performance microservices
    - Web3/blockchain applications/analysis
    - Crypto trading tools
    - Ultra-optimized embedded systems
- Substance:
    - Clean
    - Modular
    - Troughtfully architected
- Including:
    - Real db, queue, metrics, logging, kafka
    - Data processing pipeline that must not drop events
    - Highhroughput component that sits between services
    - Deployable with Docker and CI without drama
    - Model errors so other services know what happened
- Initial ideas (develop 2 or 3):
    - Open Source contributions
        - https://github.com/orgs/astral-sh/repositories

## Check Rust use cases and roadmap

https://product.letsgetrusty.com/

### Use cases
- Systems Programming
    - Databases, OS
    - Security, encryption
    - Data processing
    - IoT / embedded devices
- Tool / Infrastructure
    - CLI apps
    - Platform code
    - Shared code (libraries)
    - Build & deply (CI/CD)
- Web
    - Backend
        - APIs
        - Distributed Systems
        - Microservices
    - Frontend
        - WebAssembly
- Emerging technologies
    - AR/VR
    - Blockchain

### The roadmap
- Prerequisites
    - CS fundamentals (stack, heap, pointers, etc.)
    - Memory management fundamentals
    - Programming languages fundamentals (e.g. types)
- The Rust language
    - Primities, variables and control flow
    - Memory safety with ownership & borrowing
    - Type system (option, result, structs, enums)
    - How to structure Rust projects
    - How to test and document code
    - Polymorphism with generics & traits.
    - Lifetimes & smart pointers
    - Underdanding error handling
    - Functional features
    - Concurrency & async/.await
    - The macro system
    - Unsafe Rust & FFI
    
## Next steps

- https://github.com/ctjhoa/rust-learning
- https://exercism.org/tracks/rust (medium and hard)
- https://rust-cli.github.io/book/index.html
- https://rust-embedded.org/
- https://docs.rust-embedded.org/book
- https://rustwasm.github.io/docs/book/
- https://github.com/wasm-bindgen/wasm-bindgen


---
---


# Installation

https://doc.rust-lang.org/book/ch01-01-installation.html

```bash
# Installation
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
rustc --version

# Updating and Uninstalling
rustup update
rustup self uninstall

# Reading the Local Documentation
rustup doc
# If doesn't work in WSL, run this first
sudo apt install wslu

# VS Code extension
https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer
```
