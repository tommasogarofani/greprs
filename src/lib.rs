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
