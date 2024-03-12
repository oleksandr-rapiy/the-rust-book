use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        // NOTE: 0 index is a binary of the app
        let query = &args[1];
        let filename = &args[2];

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        return Ok(Config {
            query: query.clone(),
            filename: filename.clone(),
            case_sensitive
        });
    }
}