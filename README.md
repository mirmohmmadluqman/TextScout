# TextScout

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-blue?logo=rust)](https://www.rust-lang.org/)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/mirmohmmadluqman/TextScout/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A minimal command-line tool for searching files, inspired by `grep`. Built with Rust for speed and safety.

## Overview

TextScout reads a file and prints lines containing a specified query string. It supports case-insensitive searching via the `IGNORE_CASE` environment variable.

This project demonstrates:
- CLI argument parsing with `clap`
- File I/O with the standard library
- Modular code organization (main.rs for CLI, lib.rs for logic)
- Error handling with `Result` and the `?` operator
- Unit tests for core functionality

## Features

- Search for a query string in a file
- Case-insensitive mode (set `IGNORE_CASE=1`)
- Outputs matching lines to stdout
- Errors to stderr
- Cross-platform binary

## Usage

```bash
# Basic search
cargo run -- "query" path/to/file.txt

# Case-insensitive search
IGNORE_CASE=1 cargo run -- "Query" path/to/file.txt
```

Example:
```
$ cargo run -- "duct" poem.txt
safe, fast, productive.
```

## Installation

1. Install Rust: https://rustup.rs/
2. Clone the repo: `git clone https://github.com/mirmohmmadluqman/TextScout`
3. Build: `cargo build --release`
4. Run: `./target/release/textscout "query" file.txt`

## Learning Focus

- File reading with `std::fs::read_to_string`
- String processing with iterators and `filter`
- Ownership and borrowing in search functions
- Test-driven development for library funcs

## License

MIT
