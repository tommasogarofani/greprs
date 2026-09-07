use std::path::Path;

use clap::Parser;
use colored::Colorize;

#[derive(Parser, Debug)]
pub struct Config {
    /// String to search for
    pub query: String,

    /// Path to the file to search
    pub filename: String,

    /// Perform a case-insensitive search
    #[arg(short = 'i', long = "ignore-case")]
    pub ignore_case: bool,

    /// Perform a recursive search in directories
    #[arg(short = 'r', long = "recursive")]
    pub recursive: bool,
}

/// Execute the search based on the provided configuration.
/// Returns a `Result` with an error if the file cannot be read or if there is an error during the search.
pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(&config.filename);
    if path.is_dir() {
        if config.recursive {
            let results = search_dir(&config, path)?;

            println!(
                "{} occurrences found:",
                results.len().to_string().cyan().bold()
            );
            for (file_path, line) in results {
                let highlighted = highlight_query_in_line(&line, &config);
                println!(
                    "{}: {highlighted}",
                    file_path.display().to_string().magenta()
                );
            }
        } else {
            eprintln!(
                "Error: '{}' is a directory. Use the --recursive (-r) flag to search in directories.",
                config.filename
            );
            return Err("Directory provided without --recursive flag".into());
        }
    } else {
        let results = search_file(&config, path)?;

        println!(
            "{} occurrences found:",
            results.len().to_string().cyan().bold()
        );
        for line in results {
            let highlighted = highlight_query_in_line(&line, &config);
            println!("{highlighted}");
        }
    }

    Ok(())
}

/// search_dir searches for the query in the specified directory and its subdirectories.
fn search_dir(
    config: &Config,
    path: &Path,
) -> Result<Vec<(std::path::PathBuf, String)>, Box<dyn std::error::Error>> {
    let mut results = Vec::new();

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            let sub_results = search_dir(config, &entry_path)?;
            results.extend(sub_results);
        } else if entry_path.is_file() {
            let file_results = search_file(config, &entry_path)?;
            for line in file_results {
                results.push((entry_path.clone(), line));
            }
        }
    }

    Ok(results)
}

/// searche_file searches for the query in the specified file calling the appropriate search function based on the ignore_case flag.
fn search_file(config: &Config, path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    if config.ignore_case {
        let contents = std::fs::read_to_string(path)?;
        Ok(search_case_insensitive(&config.query, &contents))
    } else {
        let contents = std::fs::read_to_string(path)?;
        Ok(search(&config.query, &contents))
    }
}

/// Search for the query in the contents and return a vector of matching lines.
fn search(query: &str, contents: &str) -> Vec<String> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .map(|line| line.to_string())
        .collect()
}

/// Search for the query in the contents (case-insensitive) and return a vector of matching lines.
fn search_case_insensitive(query: &str, contents: &str) -> Vec<String> {
    let query_lowercase = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query_lowercase))
        .map(|line| line.to_string())
        .collect()
}

/// highlight_query_in_line highlights the occurrences of the query in the given line
fn highlight_query_in_line(line: &str, config: &Config) -> String {
    if config.query.is_empty() {
        return line.to_string();
    }

    let mut result = String::new();
    let mut last_idx = 0;

    if config.ignore_case {
        let line_lower = line.to_lowercase();
        let query_lower = config.query.to_lowercase();

        while let Some(idx) = line_lower[last_idx..].find(&query_lower) {
            let actual_idx = last_idx + idx;
            result.push_str(&line[last_idx..actual_idx]);

            let matched_text = &line[actual_idx..actual_idx + config.query.len()];
            result.push_str(&matched_text.bold().to_string());

            last_idx = actual_idx + config.query.len();
        }
    } else {
        // Trova e colora le corrispondenze esatte
        while let Some(idx) = line[last_idx..].find(&config.query) {
            let actual_idx = last_idx + idx;
            result.push_str(&line[last_idx..actual_idx]);

            let matched_text = &line[actual_idx..actual_idx + config.query.len()];
            result.push_str(&matched_text.bold().to_string());

            last_idx = actual_idx + config.query.len();
        }
    }

    // Aggiunge la parte rimanente della riga
    result.push_str(&line[last_idx..]);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to create a Config instance for testing purposes.
    fn make_test_config(query: &str, ignore_case: bool, recursive: bool) -> Config {
        Config {
            query: query.to_string(),
            filename: String::new(),
            ignore_case,
            recursive,
        }
    }

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

    /// `highlight_query_in_line_exact_match` tests that the `highlight_query_in_line` function correctly highlights the query in the line when there is an exact match.
    #[test]
    fn highlight_query_in_line_exact_match() {
        // Abilita la colorazione anche durante l'esecuzione di cargo test
        colored::control::set_override(true);

        let line = "rust is productive";
        let query = "rust";
        let config = Config {
            query: query.to_string(),
            filename: String::new(),
            ignore_case: false,
            recursive: false,
        };
        let highlighted = highlight_query_in_line(line, &config);

        // Verifica che la parola "rust" contenga i codici di escape ANSI del colore
        assert!(highlighted.contains(&query.bold().to_string()));
    }

    /// `highlight_query_in_line_case_insensitive_preserves_original_casing` tests that the `highlight_query_in_line` function correctly highlights the query in the line when there is a case-insensitive match, preserving the original casing of the matched text.
    #[test]
    fn highlight_query_in_line_case_insensitive_preserves_original_casing() {
        colored::control::set_override(true);

        let line = "Rust is productive";
        let query = "rUsT";
        let config = Config {
            query: query.to_string(),
            filename: String::new(),
            ignore_case: true,
            recursive: false,
        };
        let highlighted = highlight_query_in_line(line, &config);

        // Deve preservare la "R" maiuscola originale di "Rust", ma applicare il colore
        let expected_match = "Rust".bold().to_string();
        assert!(highlighted.contains(&expected_match));
    }

    /// `highlight_query_in_line_multiple_matches` tests that the `highlight_query_in_line` function correctly highlights all occurrences of the query in the line when there are multiple matches.
    #[test]
    fn highlight_query_in_line_multiple_matches() {
        colored::control::set_override(true);

        let line = "rust and rust again";
        let query = "rust";
        let config = Config {
            query: query.to_string(),
            filename: String::new(),
            ignore_case: false,
            recursive: false,
        };
        let highlighted = highlight_query_in_line(line, &config);

        let expected_match = "rust".bold().to_string();
        // Conta quante volte compare la sequenza formattata
        assert_eq!(highlighted.matches(&expected_match).count(), 2);
    }

    /// search_dir_finds_files_recursively tests that the search_dir function correctly finds files in a directory and its subdirectories when the recursive flag is set to true.
    #[test]
    fn search_dir_finds_files_recursively() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = std::env::temp_dir().join("minigrep_test_dir");
        let sub_dir = temp_dir.join("subdir");
        std::fs::create_dir_all(&sub_dir)?;

        let file1 = temp_dir.join("file1.txt");
        let file2 = sub_dir.join("file2.txt");

        std::fs::write(&file1, "rust safe and fast")?;
        std::fs::write(&file2, "learning rust deeply")?;

        let config = make_test_config("rust", false, true);

        let results = search_dir(&config, &temp_dir)?;

        let _ = std::fs::remove_dir_all(&temp_dir);

        assert_eq!(results.len(), 2);
        Ok(())
    }
}
