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

    println!(
        "{} risultati trovati:",
        results.len().to_string().cyan().bold()
    );
    for line in results {
        let highlighted = highlight_query_in_line(line, &config.query, config.ignore_case);
        println!("{highlighted}");
    }

    Ok(())
}

/// highlight_query_in_line highlights the occurrences of the query in the given line
fn highlight_query_in_line(line: &str, query: &str, ignore_case: bool) -> String {
    if query.is_empty() {
        return line.to_string();
    }

    let mut result = String::new();
    let mut last_idx = 0;

    if ignore_case {
        let line_lower = line.to_lowercase();
        let query_lower = query.to_lowercase();

        while let Some(idx) = line_lower[last_idx..].find(&query_lower) {
            let actual_idx = last_idx + idx;
            result.push_str(&line[last_idx..actual_idx]);

            let matched_text = &line[actual_idx..actual_idx + query.len()];
            result.push_str(&matched_text.bold().to_string());

            last_idx = actual_idx + query.len();
        }
    } else {
        // Trova e colora le corrispondenze esatte
        while let Some(idx) = line[last_idx..].find(query) {
            let actual_idx = last_idx + idx;
            result.push_str(&line[last_idx..actual_idx]);

            let matched_text = &line[actual_idx..actual_idx + query.len()];
            result.push_str(&matched_text.bold().to_string());

            last_idx = actual_idx + query.len();
        }
    }

    // Aggiunge la parte rimanente della riga
    result.push_str(&line[last_idx..]);
    result
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

    /// `highlight_query_in_line_exact_match` tests that the `highlight_query_in_line` function correctly highlights the query in the line when there is an exact match.
    #[test]
    fn highlight_query_in_line_exact_match() {
        // Abilita la colorazione anche durante l'esecuzione di cargo test
        colored::control::set_override(true);

        let line = "rust is productive";
        let query = "rust";
        let highlighted = highlight_query_in_line(line, query, false);

        // Verifica che la parola "rust" contenga i codici di escape ANSI del colore
        assert!(highlighted.contains(&query.bold().to_string()));
    }

    /// `highlight_query_in_line_case_insensitive_preserves_original_casing` tests that the `highlight_query_in_line` function correctly highlights the query in the line when there is a case-insensitive match, preserving the original casing of the matched text.
    #[test]
    fn highlight_query_in_line_case_insensitive_preserves_original_casing() {
        colored::control::set_override(true);

        let line = "Rust is productive";
        let query = "rUsT";
        let highlighted = highlight_query_in_line(line, query, true);

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
        let highlighted = highlight_query_in_line(line, query, false);

        let expected_match = "rust".bold().to_string();
        // Conta quante volte compare la sequenza formattata
        assert_eq!(highlighted.matches(&expected_match).count(), 2);
    }
}
