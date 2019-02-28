/*
 * Main LIBRARY file for RASCAL
 * Imports scanner
 */
use std::fs;
use std::error::Error;

mod scanner;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(&config.filename)?;

    println!("{}", contents);
    scanner::scanfile(&config.filename);

    Ok(())
}
