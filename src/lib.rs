use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line)
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() == 1 || args.contains(&"--help".to_string()) {
            return Err("Usage: minigrep <query> <filename> [--case-insensitive]\n\
                \n\
                Searches for <query> in <filename>.\n\
                Use --case-insensitive flag for case-insensitive search.");
        }

        if args.len() < 3 {
            return Err("Not enough arguments!\n\
                Usage: minigrep <query> <filename> [--case-insensitive]");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = !args.contains(&"--case-insensitive".to_string());

        Ok(Config { query, filename, case_sensitive })
    }
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
     let query = query.to_lowercase();
     let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";


        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }



    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
}

}