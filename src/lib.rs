pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    /// Costruisce una `Config` a partire dagli argomenti CLI.
    /// Restituisce un `Result` con un messaggio d'errore in caso di argomenti insufficienti.
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("String di ricerca mancante"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Percorso del file mancante"),
        };

        Ok(Config { query, filename })
    }
}

/// Esegue la ricerca del contenuto del file specificato nella `Config`.
/// Restituisce un `Result` con un messaggio d'errore in caso di problemi durante la lettura del file.
pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(config.filename)?;

    let results = search(&config.query, &contents);
    println!("{} risultati trovati:", results.len());
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec;

    /// Test per la funzione `search` in caso di nessuna riga contenente la query
    #[test]
    fn no_results() {
        let query = "duct";
        let contents = "\
The quick brown fox jumps over the lazy dog.
This small file contains simple words for testing purposes.
Nothing in these lines will match the target string.";

        let expected: Vec<&str> = vec![];
        assert_eq!(expected, search(query, contents))
    }

    /// Test per la funzione `search` in caso di una sola riga contenente la query
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    /// Test per la funzione `search` in caso di più righe contenenti la query
    #[test]
    fn more_than_one_result() {
        let query = "duct";
        let contents = "\
This new ductile material
helps us create a highly
productive manufacturing process.";

        assert_eq!(
            vec![
                "This new ductile material",
                "productive manufacturing process."
            ],
            search(query, contents)
        )
    }
}
