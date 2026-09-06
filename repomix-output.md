This file is a merged representation of the entire codebase, combined into a single document by Repomix.

# File Summary

## Purpose
This file contains a packed representation of the entire repository's contents.
It is designed to be easily consumable by AI systems for analysis, code review,
or other automated processes.

## File Format
The content is organized as follows:
1. This summary section
2. Repository information
3. Directory structure
4. Repository files (if enabled)
5. Multiple file entries, each consisting of:
  a. A header with the file path (## File: path/to/file)
  b. The full contents of the file in a code block

## Usage Guidelines
- This file should be treated as read-only. Any changes should be made to the
  original repository files, not this packed version.
- When processing this file, use the file path to distinguish
  between different files in the repository.
- Be aware that this file may contain sensitive information. Handle it with
  the same level of security as you would the original repository.

## Notes
- Some files may have been excluded based on .gitignore rules and Repomix's configuration
- Binary files are not included in this packed representation. Please refer to the Repository Structure section for a complete list of file paths, including binary files
- Files matching patterns in .gitignore are excluded
- Files matching default ignore patterns are excluded
- Files are sorted by Git change count (files with more changes are at the bottom)

# Directory Structure
````
src/
  lib.rs
  main.rs
.gitignore
Cargo.toml
Makefile
README.md
````

# Files

## File: .gitignore
````
/target
````

## File: Makefile
````makefile
# Nome del file Makefile per il progetto minigrep

.PHONY: all check fmt lint test build release doc clean run help

# Comando di default eseguito lanciando semplicemente `make`
all: fmt lint check test

## check: Controlla che il codice compili rapidamente senza generare il binario
check:
	cargo check

## fmt: Formatta automaticamente tutto il codice sorgente
fmt:
	cargo fmt

## fmt-check: Verifica se il codice è formattato correttamente senza modificarlo
fmt-check:
	cargo fmt -- --check

## lint: Esegue Clippy per trovare codice non idiomatico o potenziali errori
lint:
	cargo clippy -- -D warnings

## test: Esegue la suite dei test
test:
	cargo test

## build: Compila il progetto in modalità Debug
build:
	cargo build

## release: Compila il binario finale ottimizzato per la produzione
release:
	cargo build --release

## doc: Genera e apre la documentazione del progetto
doc:
	cargo doc --open

## clean: Rimuove i file generati dalla compilazione (cartella target/)
clean:
	cargo clean

## run: Esegue il programma con argomenti di esempio (es. make run ARGS="query file.txt")
run:
	cargo run -- $(ARGS)

## help: Mostra questo messaggio di aiuto
help:
	@echo "Comandi disponibili:"
	@sed -n 's/^##//p' $(MAKEFILE_LIST)
````

## File: src/lib.rs
````rust
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Config {
    /// String to search for
    pub query: String,

    /// Path to the file to search
    pub filename: String,

    /// Perform a case-insensitive search
    #[arg(short = 'i', long = "ignore-case")]
    pub ignore_case: bool,
}

/// Execute the search based on the provided configuration.
/// Returns a `Result` with an error if the file cannot be read or if there is an error during the search.
pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(&config.filename)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    println!("{} risultati trovati:", results.len());
    for line in results {
        println!("{line}");
    }

    Ok(())
}

/// Search for the query in the contents and return a vector of matching lines.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// Search for the query in the contents (case-insensitive) and return a vector of matching lines.
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query_lowercase = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query_lowercase))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// search_returns_empty_when_no_match tests that the search function returns an empty vector when there are no matches for the query in the contents.
    #[test]
    fn search_returns_empty_when_no_match() {
        let query = "duct";
        let contents = "\
The quick brown fox jumps over the lazy dog.
This small file contains simple words for testing purposes.
Nothing in these lines will match the target string.";

        assert!(search(query, contents).is_empty());
    }

    /// search_returns_single_line_match tests that the search function returns a vector containing the single matching line when there is one match for the query in the contents.
    #[test]
    fn search_returns_single_line_match() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    /// search_returns_multiple_lines_match tests that the search function returns a vector containing all matching lines when there are multiple matches for the query in the contents.
    #[test]
    fn search_returns_multiple_lines_match() {
        let query = "duct";
        let contents = "\
This new ductile material
helps us create a highly
productive manufacturing process.";

        let expected = vec![
            "This new ductile material",
            "productive manufacturing process.",
        ];

        assert_eq!(expected, search(query, contents));
    }

    /// search_case_insensitive_matches_mixed_case tests that the search_case_insensitive function correctly matches lines regardless of case, returning all matching lines.
    #[test]
    fn search_case_insensitive_matches_mixed_case() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
````

## File: Cargo.toml
````toml
[package]
name = "minigrep"
version = "0.1.0"
edition = "2024"

[dependencies]
clap = { version = "4.5", features = ["derive"] }
````

## File: src/main.rs
````rust
use clap::Parser;
use minigrep::Config;
use std::process;

fn main() {
    let config = Config::parse();

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
````

## File: README.md
````markdown
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
Use the ignore case flag (`--ignore-case`, or `-i`) to perform a case-insensitive search:

```bash
# On Unix/Linux/macOS
cargo run -- duct poem.txt --ignore-case
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
````
