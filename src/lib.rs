use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> { //error values will always be string literals that have the 'static lifetime.
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        //env::var will return a Result but as we do not care about it content
        // we use is_ok()->bool instead of unwrap or expect
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(
            Config {
                query: args[1].clone(), 
                file_path: args[2].clone(),
                ignore_case,
            })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> { //  means the function will return a type that implements the 
    let contents = fs::read_to_string(config.file_path)?; // Error trait, but we don’t have to specify what particular type the return value will be.
    // "? will return the error value from the current function for the caller to handle
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{line}");
    }
    Ok(())
}

// The lifetime parameters specify which argument lifetime is connected 
// to the lifetime of the return value. In this case, we indicate that 
// the returned vector should contain string slices that reference slices 
// of the argument contents (rather than the argument query)
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    if results.len() < 1 {
        results.push("Word not fund");
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query){ // as we have a new query var and to_lowercase return in own String 
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
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
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