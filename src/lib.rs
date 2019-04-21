/*
 * Main LIBRARY file for RASCAL
 * Utilizes the scanner module
 */
mod tokens;
// mod symbtab;
mod scanner;
mod parser;
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
    let scan = Scanner::new(&config.filename);
    let mut parser = Parser::new(scan);

    parser.parse();

    println!("The generated code is: {:?}", parser.code);

    let mut rvm = RvmMachine::new(
        vec![
            0x01, 0x00, 0x00, 0x00, 0x05,
            0x01, 0x00, 0x00, 0x00, 0x14,
            0x10,
            0x00
        ]
    );
    rvm.exec();

    let mut rvm2 = RvmMachine::new(parser.code);
    rvm2.exec();

    Ok(())
}
