# README.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview
A Rust implementation of the FizzBuzz problem that demonstrates idiomatic Rust patterns including struct-based design with pattern matching.

## Commands

### Build and Run
- **Build the project**: `cargo build`
- **Run the application**: `cargo run`
- **Build release version**: `cargo build --release`
- **Run release version**: `cargo run --release`

### Testing
- **Run all tests**: `cargo test`
- **Run tests verbosely**: `cargo test -- --nocapture`
- **Run specific test**: `cargo test <test_name>`

Note: Currently no tests are implemented.

### Code Quality
- **Check code without building**: `cargo check`
- **Format code**: `cargo fmt`
- **Run linter**: `cargo clippy`

## Architecture

### Core Design Pattern
The implementation uses a **struct-based approach with pattern matching** rather than traditional if-else conditionals:

1. **FizzBuzz struct**: Encapsulates the state of divisibility checks
   - `multiple_of_3`: bool - whether number is divisible by 3
   - `multiple_of_5`: bool - whether number is divisible by 5

2. **Constructor pattern**: `FizzBuzz::new(number)` creates instances with pre-computed divisibility state

3. **Pattern matching**: The main logic uses Rust's match expression with struct patterns to handle all four cases:
   - Both multiples (fizzbuzz)
   - Multiple of 3 only (fizz)
   - Multiple of 5 only (buzz)
   - Neither (print number)

4. **Side-effect methods**: Each output case has a dedicated method (`fizz_buzz_side_effect`, `fizz_side_effect`, `buzz_side_effect`)

### Code Organization
- Single-file application in `src/main.rs`
- All logic contained in one module
- No external dependencies (pure Rust std library)

### Rust Edition
Project uses Rust 2018 edition as specified in Cargo.toml.
