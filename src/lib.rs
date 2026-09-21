use clap::Parser;
use colored::Colorize;
use std::io::{BufRead, BufReader};
use std::path::Path;

type LineReturn = (std::path::PathBuf, usize, String);

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Search for patterns in files or directories.",
    long_about = "\
Search for PATTERNS in FILE.
Example: greprs -i 'hello world' main.c."
)]
pub struct Config {
    /// String to search for
    #[arg(value_name = "PATTERNS")]
    pub query: String,

    /// Path to the file to search
    #[arg(value_name = "FILE")]
    pub file_path: String,

    /// Perform a case-insensitive search
    #[arg(short = 'i', long = "ignore-case")]
    pub ignore_case: bool,

    /// Perform a recursive search in directories
    #[arg(short = 'r', long = "recursive")]
    pub recursive: bool,

    /// Display line numbers
    #[arg(short = 'n', long = "line-number")]
    pub line_number: bool,

    /// Invert the match, showing lines that do not contain the pattern
    #[arg(short = 'v', long = "invert-match")]
    pub invert_match: bool,

    /// Count the number of occurrences instead of displaying them
    #[arg(short = 'c', long = "count")]
    pub count: bool,
}

/// Execute the search based on the provided configuration.
/// Returns a `Result` with an error if the file cannot be read or if there is an error during the search.
pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(&config.file_path);
    if path.is_dir() {
        if config.recursive {
            if config.count {
                println!("{}", count_dir(&config, path)?);
                return Ok(());
            }

            let results = match search_dir(&config, path) {
                Ok(results) => results,
                Err(error) => {
                    eprintln!("Error searching '{}': {error}", path.display());
                    return Ok(());
                }
            };

            if config.line_number {
                for (file_path, line_number, line) in results {
                    let highlighted = if !config.invert_match {
                        highlight_query_in_line(&line, &config)
                    } else {
                        line
                    };
                    println!(
                        "{}:{}: {highlighted}",
                        file_path.display().to_string().magenta(),
                        line_number.to_string().cyan().bold()
                    );
                }
            } else {
                for (file_path, _line_number, line) in results {
                    let highlighted = if !config.invert_match {
                        highlight_query_in_line(&line, &config)
                    } else {
                        line
                    };
                    println!(
                        "{}: {highlighted}",
                        file_path.display().to_string().magenta(),
                    );
                }
            }
        } else {
            eprintln!(
                "Error: '{}' is a directory. Use the --recursive (-r) flag to search in directories.",
                config.file_path
            );
            return Err("Directory provided without --recursive flag".into());
        }
    } else {
        if config.count {
            let count = match count_file(&config, path) {
                Ok(count) => count,
                Err(error) => {
                    eprintln!("Error reading '{}': {error}", path.display());
                    return Ok(());
                }
            };
            println!("{count}");
            return Ok(());
        }

        let results = match search_file(&config, path) {
            Ok(results) => results,
            Err(error) => {
                eprintln!("Error reading '{}': {error}", path.display());
                return Ok(());
            }
        };

        if config.line_number {
            for (line_number, line) in results {
                let highlighted = if !config.invert_match {
                    highlight_query_in_line(&line, &config)
                } else {
                    line
                };
                println!("{line_number}: {highlighted}");
            }
        } else {
            for (_line_number, line) in results {
                let highlighted = if !config.invert_match {
                    highlight_query_in_line(&line, &config)
                } else {
                    line
                };
                println!("{highlighted}");
            }
        }
    }

    Ok(())
}

fn count_dir(config: &Config, path: &Path) -> Result<usize, Box<dyn std::error::Error>> {
    let mut count = 0;

    for entry in std::fs::read_dir(path)? {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                eprintln!("Error reading an entry in '{}': {error}", path.display());
                continue;
            }
        };
        let entry_path = entry.path();

        if entry_path.is_dir() {
            match count_dir(config, &entry_path) {
                Ok(sub_count) => count += sub_count,
                Err(error) => {
                    eprintln!("Error searching '{}': {error}", entry_path.display());
                }
            }
        } else if entry_path.is_file() {
            match count_file(config, &entry_path) {
                Ok(file_count) => count += file_count,
                Err(error) => {
                    eprintln!("Error reading '{}': {error}", entry_path.display());
                }
            }
        }
    }

    Ok(count)
}

fn count_file(config: &Config, path: &Path) -> Result<usize, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let reader = BufReader::new(file);
    let query_lower = config.query.to_lowercase();
    let mut count = 0;

    for line in reader.lines() {
        let line = line?;
        let matched = {
            let matched = if config.ignore_case {
                line.to_lowercase().contains(&query_lower)
            } else {
                line.contains(&config.query)
            };
            if config.invert_match {
                !matched
            } else {
                matched
            }
        };
        if matched {
            count += 1;
        }
    }

    Ok(count)
}

/// search_dir searches for the query in the specified directory and its subdirectories.
fn search_dir(config: &Config, path: &Path) -> Result<Vec<LineReturn>, Box<dyn std::error::Error>> {
    let mut results = Vec::new();

    for entry in std::fs::read_dir(path)? {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                eprintln!("Error reading an entry in '{}': {error}", path.display());
                continue;
            }
        };
        let entry_path = entry.path();

        if entry_path.is_dir() {
            match search_dir(config, &entry_path) {
                Ok(sub_results) => results.extend(sub_results),
                Err(error) => {
                    eprintln!("Error searching '{}': {error}", entry_path.display());
                }
            }
        } else if entry_path.is_file() {
            match search_file(config, &entry_path) {
                Ok(file_results) => {
                    for (line_number, line) in file_results {
                        results.push((entry_path.clone(), line_number, line));
                    }
                }
                Err(error) => {
                    eprintln!("Error reading '{}': {error}", entry_path.display());
                }
            }
        }
    }

    Ok(results)
}

/// search_file searches for the query in the specified file calling the appropriate search function based on the ignore_case flag.
fn search_file(
    config: &Config,
    path: &Path,
) -> Result<Vec<(usize, String)>, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path)?;
    Ok(search(
        &config.query,
        &contents,
        config.ignore_case,
        config.invert_match,
    )
    .into_iter()
    .map(|(line_number, line)| (line_number, line.to_owned()))
    .collect())
}

/// search searches for the query in the given contents and returns a vector of matching lines with their line numbers.
/// If the ignore_case flag is set, it performs a case-insensitive search.
fn search<'a>(
    query: &str,
    contents: &'a str,
    ignore_case: bool,
    invert_search: bool,
) -> Vec<(usize, &'a str)> {
    let query_lower = query.to_lowercase();

    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| {
            let matched = if ignore_case {
                line.to_lowercase().contains(&query_lower)
            } else {
                line.contains(query)
            };
            if invert_search { !matched } else { matched }
        })
        .map(|(idx, line)| (idx + 1, line))
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
        let mut line_lower = String::new();
        let mut lower_ranges = Vec::new();
        for (start, character) in line.char_indices() {
            let end = start + character.len_utf8();
            let lower_start = line_lower.len();
            line_lower.extend(character.to_lowercase());
            lower_ranges.push((lower_start, line_lower.len(), start, end));
        }
        let query_lower = config.query.to_lowercase();
        let mut lower_last_idx = 0;

        while let Some(idx) = line_lower[lower_last_idx..].find(&query_lower) {
            let lower_start = lower_last_idx + idx;
            let lower_end = lower_start + query_lower.len();
            let (_, _, actual_start, _) = lower_ranges
                .iter()
                .find(|(start, end, _, _)| *end > lower_start && *start < lower_end)
                .expect("case-insensitive match must map to the original line");
            let (_, _, _, actual_end) = lower_ranges
                .iter()
                .rev()
                .find(|(start, end, _, _)| *end > lower_start && *start < lower_end)
                .expect("case-insensitive match must map to the original line");

            result.push_str(&line[last_idx..*actual_start]);
            let matched_text = &line[*actual_start..*actual_end];
            result.push_str(&matched_text.bold().to_string());

            last_idx = *actual_end;
            lower_last_idx = lower_end;
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
    use clap::Parser;

    /// Helper function to create a Config instance for testing purposes.
    fn make_test_config(
        query: &str,
        ignore_case: bool,
        recursive: bool,
        line_number: bool,
        invert_match: bool,
        count: bool,
    ) -> Config {
        Config {
            query: query.to_string(),
            file_path: String::new(),
            ignore_case,
            recursive,
            line_number,
            invert_match,
            count,
        }
    }

    #[test]
    fn all_search_flags_are_enabled_when_parsed_together() {
        let config = Config::try_parse_from([
            "greprs",
            "--ignore-case",
            "--recursive",
            "--line-number",
            "--invert-match",
            "--count",
            "needle",
            "file.txt",
        ])
        .unwrap();

        assert!(config.ignore_case);
        assert!(config.recursive);
        assert!(config.line_number);
        assert!(config.invert_match);
        assert!(config.count);
    }

    #[test]
    fn search_counts_matching_lines_for_count_flag() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = tempfile::tempdir()?;
        let file_path = temp_dir.path().join("input.txt");
        std::fs::write(
            &file_path,
            "\
needle one
other line
needle two",
        )?;

        let config = make_test_config("needle", false, false, false, false, true);
        let results = search_file(&config, &file_path)?;

        assert_eq!(results.len(), 2);
        Ok(())
    }

    /// search_returns_empty_when_no_match tests that the search function returns an empty vector when there are no matches for the query in the contents.
    #[test]
    fn search_returns_empty_when_no_match() {
        let query = "duct";
        let contents = "\
The quick brown fox jumps over the lazy dog.
This small file contains simple words for testing purposes.
Nothing in these lines will match the target string.";

        assert!(search(query, contents, false, false).is_empty());
    }

    #[test]
    fn search_matches_empty_query_on_every_line() {
        let results = search("", "first\nsecond", false, false);

        assert_eq!(results, vec![(1, "first"), (2, "second")]);
    }

    #[test]
    fn search_invert_match_excludes_matching_lines() {
        let results = search("needle", "needle\nother\nNEEDLE", true, true);

        assert_eq!(results, vec![(2, "other")]);
    }

    #[test]
    fn search_preserves_unicode_lines_and_line_numbers() {
        let results = search("caffe", "prima\ncaffè\nultima", false, false);

        assert!(results.is_empty());

        let results = search("caffè", "prima\ncaffè\nultima", false, false);
        assert_eq!(results, vec![(2, "caffè")]);
    }

    /// search_returns_single_line_match tests that the search function returns a vector containing the single matching line when there is one match for the query in the contents.
    #[test]
    fn search_returns_single_line_match() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec![(2, "safe, fast, productive.")],
            search(query, contents, false, false)
        );
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
            (1, "This new ductile material"),
            (3, "productive manufacturing process."),
        ];

        assert_eq!(expected, search(query, contents, false, false));
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
            vec![(1, "Rust:"), (4, "Trust me.")],
            search(query, contents, true, false)
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
            file_path: String::new(),
            ignore_case: false,
            recursive: false,
            line_number: false,
            invert_match: false,
            count: false,
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
            file_path: String::new(),
            ignore_case: true,
            recursive: false,
            line_number: false,
            invert_match: false,
            count: false,
        };
        let highlighted = highlight_query_in_line(line, &config);

        // Deve preservare la "R" maiuscola originale di "Rust", ma applicare il colore
        let expected_match = "Rust".bold().to_string();
        assert!(highlighted.contains(&expected_match));
    }

    #[test]
    fn highlight_query_in_line_case_insensitive_handles_lowercase_expansion() {
        colored::control::set_override(true);

        let config = Config {
            query: "i".to_string(),
            file_path: String::new(),
            ignore_case: true,
            recursive: false,
            line_number: false,
            invert_match: false,
            count: false,
        };

        let highlighted = highlight_query_in_line("İstanbul", &config);

        assert!(highlighted.contains(&"İ".bold().to_string()));
    }

    /// `highlight_query_in_line_multiple_matches` tests that the `highlight_query_in_line` function correctly highlights all occurrences of the query in the line when there are multiple matches.
    #[test]
    fn highlight_query_in_line_multiple_matches() {
        colored::control::set_override(true);

        let line = "rust and rust again";
        let query = "rust";
        let config = Config {
            query: query.to_string(),
            file_path: String::new(),
            ignore_case: false,
            recursive: false,
            line_number: false,
            invert_match: false,
            count: false,
        };
        let highlighted = highlight_query_in_line(line, &config);

        let expected_match = "rust".bold().to_string();
        // Conta quante volte compare la sequenza formattata
        assert_eq!(highlighted.matches(&expected_match).count(), 2);
    }

    /// search_dir_finds_files_recursively tests that the search_dir function correctly finds files in a directory and its subdirectories when the recursive flag is set to true.
    #[test]
    fn search_dir_finds_files_recursively() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = std::env::temp_dir().join("greprs_test_dir");
        let sub_dir = temp_dir.join("subdir");
        std::fs::create_dir_all(&sub_dir)?;

        let file1 = temp_dir.join("file1.txt");
        let file2 = sub_dir.join("file2.txt");

        std::fs::write(&file1, "rust safe and fast")?;
        std::fs::write(&file2, "learning rust deeply")?;

        let config = make_test_config("rust", false, true, false, false, false);

        let results = search_dir(&config, &temp_dir)?;

        let _ = std::fs::remove_dir_all(&temp_dir);

        assert_eq!(results.len(), 2);
        Ok(())
    }

    /// search_returns_lines_with_line_numbers tests that the search function returns matching lines with 1-based line numbers.
    #[test]
    fn search_returns_lines_with_line_numbers() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        let results = search(query, contents, false, false);
        let expected = vec![(2, "safe, fast, productive.")];
        assert_eq!(expected, results);
    }

    /// search_case_insensitive_returns_lines_with_line_numbers tests that the search_case_insensitive function returns lines with line numbers when the line_number flag is set to true.
    #[test]
    fn search_case_insensitive_returns_lines_with_line_numbers() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.Pick three.
Trust me.";

        let results = search(query, contents, true, false);
        let expected = vec![(1, "Rust:"), (3, "Trust me.")];
        assert_eq!(expected, results);
    }

    /// search_invert_match_returns_lines_without_query tests that the search function returns lines that do not contain the query when the invert_match flag is set to true.
    #[test]
    fn search_invert_match_returns_lines_without_query() {
        let query = "Rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let results = search(query, contents, false, true);
        let expected = vec![
            (2, "safe, fast, productive."),
            (3, "Pick three."),
            (4, "Trust me."),
        ];
        assert_eq!(expected, results);
    }
}
