/*
 * Main LIBRARY file for RASCAL
 * Utilizes the scanner module
 */
mod errors;
mod tokens;
mod symbtab;
mod scanner;
mod parser;
mod codegen;
mod rvm;

use std::error::Error;
use scanner::Scanner;
use parser::Parser;
use rvm::RvmMachine;

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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let scan = Scanner::new(&config.filename)?;
    let mut parser = Parser::new(scan);

    parser.parse()?;

    println!("The generated code is: {:02x?}", &parser.gen.code[..]);

    let mut rvm = RvmMachine::new(parser.gen.code);
    rvm.exec();

    Ok(())
}
