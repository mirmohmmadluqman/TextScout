# TextScout Makefile - Real Dev Version
# Author: YourName
# Description: Build, run, test, and manage the Rust CLI tool TextScout

CARGO := cargo
BIN    := target/debug/textscout

.PHONY: all build run run-query test fmt clean help

# Default: build project
all: build

# Build project
build:
	@echo "[+] Building TextScout..."
	$(CARGO) build

# Run project without args
run: build
	@echo "[+] Running TextScout..."
	$(BIN)

# Run project with query args
# Usage: make run-query query="search term" num=3
run-query: build
ifeq ($(query),)
	$(error Please provide a query. Example: make run-query query="hello" num=2)
endif
	@echo "[+] Running TextScout with query='$(query)' num='$(num)'..."
	$(BIN) "$(query)" --num-count=$(num)

# Run tests
test:
	@echo "[+] Running tests..."
	$(CARGO) test

# Format code
fmt:
	@echo "[+] Formatting Rust code..."
	$(CARGO) fmt

# Clean build artifacts
clean:
	@echo "[+] Cleaning project..."
	$(CARGO) clean

# Show Makefile commands
help:
	@echo "TextScout Makefile Commands:"
	@echo "  make build        Build the project"
	@echo "  make run          Run the project (no args)"
	@echo "  make run-query    Run with query args: query='text' num=1"
	@echo "  make test         Run all tests"
	@echo "  make fmt          Format code with rustfmt"
	@echo "  make clean        Clean build artifacts"
	@echo "  make help         Show this message"
