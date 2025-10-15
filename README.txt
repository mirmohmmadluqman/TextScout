# 1. Build the project
cargo build

# 2. Run tests
cargo test

# 3. Try basic search
cargo run -- "frog" poem.txt
# Expected output: How public, like a frog

# 4. Try case-insensitive
IGNORE_CASE=1 cargo run -- "NOBODY" poem.txt
# Expected output: 
# I'm nobody! Who are you?
# Are you nobody, too?

# 5. Test error handling
cargo run -- "test"
# Expected: Problem parsing arguments: not enough arguments

cargo run -- "test" nonexistent.txt
# Expected: Application error: No such file or directory