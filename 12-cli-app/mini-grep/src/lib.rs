pub mod config;
use colored::Colorize;
use config::Config;
use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    print_result(&result, &config.query);

    return Ok(());
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    // let mut result: Vec<&str> = Vec::new();

    // for line in content.lines() {
    //     if line.contains(query) {
    //         result.push(line)
    //     }
    // }

    // return result;

    content.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result: Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line)
        }
    }

    return result;
}

fn print_result(result: &Vec<&str>, query: &str) {
    for line in result {
        for word in line.split(" ") {
            if word.to_lowercase().contains(query) {
                print!("{} ", word.green());
            } else {
                print!("{} ", word);
            }
        }
        println!()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "requ";
        let content = "\
Rust: 
safe, fast, product
require huge nerve
to learn
Requ
";
        assert_eq!(vec!["require huge nerve"], search(query, content))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUSt";
        let content = "\
Rust:
safe, fast, product
require huge nerve
to learn rust
";

        assert_eq!(
            vec!["Rust:", "to learn rust"],
            search_case_insensitive(query, content)
        )
    }
}
