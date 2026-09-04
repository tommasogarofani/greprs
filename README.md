# MiniGrep

A fast, lightweight command-line text search utility written in **Rust**, inspired by the classic `grep` tool.

This project was built as a hands-on exercise to learn Rust fundamentals, memory management (ownership & borrowing), error handling, and CLI design.

---

## Repo Architecture

```text
minigrep/
├── src/
│   ├── lib.rs     # Core business logic (config parsing, file reading, search algorithms)
│   └── main.rs    # Entry point (CLI argument parsing, I/O handling)
├── Cargo.lock
├── Cargo.toml
├── Makefile
└── README.md
```

---

## Features

* **Pattern Matching:** Search for specific string patterns inside target text files.
* **Case-Insensitive Search:** Optional environment variable support (`IGNORE_CASE=1`) to ignore case sensitivity during searches.
* **Standard Error Separation:** Error messages are sent directly to `stderr`, while matching results are routed to `stdout`.
* **Zero External Dependencies:** Built entirely using Rust's standard library (`std`).

---

## Installation & Building
Ensure you have Rust and Cargo installed. If not, follow the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

```bash
# Clone the repository
git clone https://github.com/tommasogarofani/minigrep.git
cd minigrep

# Build optimized binary
cargo build --release
```
The compiled binary will be available under `./target/release/minigrep`.

---

##  Usage

### 1. Basic Command

```bash
cargo run -- <QUERY> <FILE_PATH>
```
#### Example:
Search for the pattern "duct" inside a file named "poem.txt":

```bash
cargo run -- duct poem.txt
```

### 2. Case-Insensitive Search
Set the IGNORE_CASE environment variable to 1 (or any non-empty value) to perform a case-insensitive search:

```bash
# On Unix/Linux/macOS
IGNORE_CASE=1 cargo run -- duct poem.txt

# On Windows (PowerShell)
$env:IGNORE_CASE=1; cargo run -- duct poem.txt
```

---

## Testing
Run the test suite (includes both unit and documentation tests):
```bash
cargo test
```

---

## Contributing
Contributions are welcome! If you find a bug or have a feature request, please open an issue. For code contributions, fork the repository and submit a pull request.

### Rules for Contributions:
1. Follow Rust's best practices and idioms.
2. Make atomic commits with clear commit messages.
3. Ensure all tests pass before submitting a pull request.

---

## Credits

Tommaso Garofani [@tommasogarofani](https://github.com/tommasogarofani) - Original author and maintainer of MiniGrep.