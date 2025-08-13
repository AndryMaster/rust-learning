use std::fs;
use std::env;
use std::process;
use std::error::Error;

use minigrep::*;


fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem building config: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(err) = run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    };
}


fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;


    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("\t{line}");
    }

    Ok(())
}
