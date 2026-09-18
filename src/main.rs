use minigrep::{search, search_case_insensitive};
use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    if let Err(e) = run(&config) {
        eprintln!("Error: {}", e);
        process::exit(1)
    }
}

fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_path)?;

    if config.ignore_case {
        for line in search_case_insensitive(&config.query, &content) {
            println!("{line}");
        }
    } else {
        for line in search(&config.query, &content) {
            println!("{line}");
        }
    }
    Ok(())
}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();
        let query = match args.next() {
            Some(s) => s,
            None => return Err("Didn't get a query string!"),
        };
        let file_path = match args.next() {
            Some(s) => s,
            None => return Err("Didn't get a file path string!"),
        };
        let ignore_case = env::var("MINIGREP_IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
