# Greprs

A fast, lightweight command-line text search utility written in **Rust**, inspired by the classic `grep` tool.

This project was built as a hands-on exercise to learn Rust fundamentals, memory management (ownership & borrowing), error handling, and CLI design.

---

## Repo Architecture

```text
greprs/
├── src/
│   ├── lib.rs            # Core business logic (config parsing, file reading, search algorithms)
│   └── main.rs           # Entry point (CLI argument parsing, I/O handling)
├── tests/
│   ├── common/
│   │   └── mod.rs        # Helper/utility condivise dai test
│   ├── flags.rs          # Test per le varie flag (--ignore-case, --line-number)
│   ├── recursive.rs      # Test per la modalità ricorsiva (-r)
│   └── search.rs         # Test di integrazione per le ricerche di testo
├── Cargo.lock            # Cargo's lock file for reproducible builds (auto-managed)
├── Cargo.toml            # Rust package manifest
├── Makefile              # Optional build automation (e.g., for testing, building, cleaning)
└── README.md             # Project documentation
```

---

## Features

* **Pattern Matching:** Search for specific string patterns inside target text files.
* **Case-Insensitive Search:** Optional environment variable support (`IGNORE_CASE=1`) to ignore case sensitivity during searches.
* **Standard Error Separation:** Error messages are sent directly to `stderr`, while matching results are routed to `stdout`.
* **Minimal Dependencies:** Built using clap for CLI parsing and colored for beautiful output.

---

## Installation & Building
Ensure you have Rust and Cargo installed. If not, follow the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

```bash
# Clone the repository
git clone https://github.com/tommasogarofani/greprs.git
cd greprs

# Build optimized binary
cargo build --release
```
The compiled binary will be available under `./target/release/greprs`.

---

##  Usage

### Basic Command

```bash
cargo run -- <QUERY> <FILE_PATH>
```
#### Example:
Search for the pattern "duct" inside a file named "poem.txt":

```bash
cargo run -- duct poem.txt
```

### Flags
- `--help`, `-h`: Display help information.
- `--ignore-case`, `-i`: Perform a case-insensitive search.
- `--line-number`, `-n`: Display line numbers for each match.
- `--recursive`, `-r`: Search within directories and their subdirectories. (Note: <FILE_PATH> should be a directory)
- `--invert-match`, `-v`: Invert the match, showing lines that do not contain the pattern.
- `--version`, `-V`: Display the current version of Greprs.

---

## Testing
Run the test suite (includes both unit and documentation tests):
```bash
cargo test
```

---

## Contributing
Contributions are welcome! If you find a bug or have a feature request, please open an issue. For code contributions, fork the repository and submit a pull request.

### Branching & Release Workflow
We follow a Git flow model focused on feature isolation and Semantic Versioning:

1. **Development Branches (`feature/<name>` or `bugfix/<name>`):**
   * Create a new branch from `develop` for any new feature or fix (e.g., `feature/invert-match-flag`).
   * Never commit directly to `master` or `develop`.

2. **Integration (`develop`):**
   * Submit a Pull Request targeting the `develop` branch.
   * Ensure all tests pass (`cargo test`) before requesting a review or merging.

3. **Preparing a Release (`Cargo.toml`):**
   * When features on `develop` are ready for a new release, update the `version` field in `Cargo.toml` according to [Semantic Versioning](https://semver.org/):
     * **MAJOR** (`X.0.0`): Breaking changes (e.g., major CLI restructuring or breaking lib API changes).
     * **MINOR** (`1.X.0`): New backward-compatible features (e.g., adding a new CLI flag like `-n` or `-i`).
     * **PATCH** (`1.0.X`): Backward-compatible bug fixes or minor documentation updates.
   * Commit the version bump on `develop` (e.g., `git commit -m "chore: bump version to 1.4.0"`).

4. **Publishing Releases (`master`):**
   * Merge `develop` into `master`, tag the merge commit with the version number (e.g., `v1.4.0`), and push the tag to publish the new GitHub Release.

### Rules for Contributions:
1. Follow Rust's best practices and idioms.
2. Make atomic commits with clear commit messages.
3. Ensure all tests pass before submitting a pull request (`cargo test`).

---

## Credits

Tommaso Garofani [@tommasogarofani](https://github.com/tommasogarofani) - Original author and maintainer of Greprs.