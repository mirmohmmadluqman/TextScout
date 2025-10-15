# TextScout

[![Rust](https://img.shields.io/badge/Rust-Programming-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen?style=for-the-badge)](#)
[![License](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge)](#)

A lightweight command-line file search tool built in Rust, inspired by `grep`.

## Overview

TextScout searches for text patterns in files and displays matching lines. It demonstrates core Rust concepts including:
- File I/O with error handling
- Command-line argument parsing
- Modular code organization (binary + library)
- Environment variable configuration
- Iterator patterns and functional programming
- Test-driven development

## Features

- Fast case-sensitive text search
- Optional case-insensitive mode via environment variable
- Clean error handling with descriptive messages
- Modular architecture separating CLI from core logic
- Comprehensive unit tests

## Usage
```bash
# Basic search (case-sensitive)
cargo run -- "frog" poem.txt

# Case-insensitive search
IGNORE_CASE=1 cargo run -- "nobody" poem.txt

# Or build and run the binary
cargo build --release
./target/release/text_scout "frog" poem.txt
```

## Architecture
```
+----------------+     +----------------+     +----------------+
| CLI Args       | --> | main.rs        | --> | lib.rs         |
| (env::args)    |     | (parse args,   |     | (search logic, |
|                |     |  build Config) |     |  file I/O)     |
+----------------+     +----------------+     +----------------+
                              |                       |
                              v                       v
                       +----------------+     +----------------+
                       | Stdout (lines) |     | Stderr (errors)|
                       +----------------+     +----------------+
```

## Testing
```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture
```

## Learning Focus

This project covers:
- **Chapter 7**: Organizing code with modules and crates
- **Chapter 8**: Working with strings and vectors
- **Chapter 9**: Error handling with `Result` and `?` operator
- **Chapter 10**: Generic lifetimes in function signatures
- **Chapter 11**: Writing automated tests

## Project Status

- Core search functionality  
- Case-sensitive and insensitive modes  
- Error handling  
- Unit tests  
- Integration tests (planned)  
- Regex support (planned)  

## License

MIT