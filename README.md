# Rust Programming Language Basics

This project serves as a learning resource for Rust programming language fundamentals. It contains code examples with descriptive documentation to help you understand core Rust concepts.

## What is Rust?

Rust is a systems programming language that focuses on safety, speed, and concurrency. It provides memory safety guarantees without needing a garbage collector, making it ideal for performance-critical applications, embedded systems, and situations where control over system resources is important.

## Rust Project Structure

A typical Rust project has the following structure:

```
project_name/
├── Cargo.toml       # Project manifest: dependencies, metadata, etc.
├── Cargo.lock       # Lock file (generated automatically)
├── src/
│   ├── main.rs      # Entry point for binary projects
│   └── lib.rs       # Entry point for library projects
├── tests/           # Integration tests
├── benches/         # Benchmarks
├── examples/        # Example code
└── target/          # Build output (generated)
```

### Key Files

- **Cargo.toml**: The project manifest that contains project metadata, dependencies, build settings, etc.
- **src/main.rs**: The entry point for binary (executable) crates
- **src/lib.rs**: The entry point for library crates
- **modules**: Can be defined in separate files (e.g., src/module_name.rs) or directories (src/module_name/mod.rs)

## Compilation and Build Process

Rust uses a compiler (`rustc`) and a build system/package manager (`cargo`). The compilation process involves:

1. **Parsing**: Converts source code into an Abstract Syntax Tree (AST)
2. **Macro expansion**: Processes any macros in the code
3. **Type checking**: Ensures type safety and ownership rules
4. **MIR generation**: Converts to Mid-level Intermediate Representation
5. **Optimization**: Performs various optimizations
6. **Code generation**: Creates machine code

### Common Cargo Commands

- `cargo new project_name`: Create a new project
- `cargo build`: Compile the project
- `cargo run`: Compile and run the project
- `cargo check`: Check for errors without producing an executable
- `cargo test`: Run tests
- `cargo doc`: Generate documentation
- `cargo publish`: Publish a library to crates.io

## Key Rust Concepts

### Ownership and Borrowing

Rust's most unique feature is its ownership system that ensures memory safety without garbage collection:

- Each value has exactly one owner
- When the owner goes out of scope, the value is dropped
- Values can be borrowed immutably by multiple references or mutably by a single reference
- The borrow checker enforces these rules at compile time

### Type System

Rust has a strong, static type system with:

- Primitive types: integers, floats, booleans, characters, etc.
- Compound types: tuples, arrays, structs, enums
- Special types: Option<T>, Result<T, E> for error handling without exceptions

### Traits

Rust uses traits (similar to interfaces) to define shared behavior:

- Traits define method signatures that types can implement
- They enable generic programming and polymorphism
- Common traits include: Clone, Copy, Debug, Display, etc.

### Pattern Matching

Rust's pattern matching via `match` expressions provides powerful control flow:

- Exhaustive checking ensures all possible cases are handled
- Can destructure complex data types
- Often used with enums like Option and Result

### Concurrency

Rust's ownership system helps prevent data races:

- Threads can be created with `std::thread`
- Data can be passed between threads safely
- `Arc` and `Mutex` types provide thread-safe sharing
- Channels allow message-passing between threads

## Tooling for Rust Development

- **Cargo**: Package manager and build system
- **Rustup**: Toolchain installer and version manager
- **Clippy**: Linting tool for catching common mistakes
- **Rustfmt**: Code formatter for consistent style
- **Rust Analyzer**: Language server for IDE integration
- **Rust Playground**: Online environment for testing code snippets (play.rust-lang.org)

## Additional Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)
- [Rustlings](https://github.com/rust-lang/rustlings): Small exercises to get used to reading and writing Rust code
- [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)
