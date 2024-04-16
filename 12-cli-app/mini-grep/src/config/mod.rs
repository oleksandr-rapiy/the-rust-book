use std::env::{self, Args};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // if args.len() < 3 {
        //     return Err("Not enough arguments");
        // }

        // // NOTE: 0 index is a binary of the app
        // let query = &args[1];
        // let filename = &args[2];

        // NOTE: 0 index is a binary of the app
        args.next();

        let query = match args.next() {
            Some(value) => value,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(value) => value,
            None => return Err("Didn't get a filename string"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        return Ok(Config {
            query,
            filename,
            case_sensitive
        });
    }
}