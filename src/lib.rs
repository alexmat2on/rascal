/*
 * Main LIBRARY file for RASCAL
 * Utilizes the scanner module
 */
use std::fs;
use std::error::Error;

mod scanner;
use scanner::Scanner;

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

    let mut scan = Scanner::new(&config.filename);
    scan.print_types();
    println!("tok: {:?}", scan.read_next_token());
    println!("tok: {:?}", scan.read_next_token());
    println!("tok: {:?}", scan.read_next_token());
    println!("tok: {:?}", scan.read_next_token());
    println!("tok: {:?}", scan.read_next_token());
    println!("tok: {:?}", scan.read_next_token());
    println!("tok: {:?}", scan.read_next_token());
    println!("tok: {:?}", scan.read_next_token());
    println!("tok: {:?}", scan.read_next_token());
    println!("tok: {:?}", scan.read_next_token());
    println!("tok: {:?}", scan.read_next_token());
    println!("tok: {:?}", scan.read_next_token());

    Ok(())
}
