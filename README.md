# TextScout (In Development)

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-blue?logo=rust)](https://www.rust-lang.org/)

**Status:** Development phase  

A minimal command-line tool for searching files, inspired by `grep`.  
Currently in development: core search logic works, but more features coming soon.

## Overview

- Reads a file and prints lines containing a query.
- Basic case-insensitive search via `IGNORE_CASE`.
- CLI argument parsing using `env::args` (work-in-progress).
- Modular code (main.rs for CLI, lib.rs for search logic).

## Usage (development phase)

```bash
# Example usage
cargo run -- "query" path/to/file.txt

# Case-insensitive (basic)
IGNORE_CASE=1 cargo run -- "Query" path/to/file.txt
