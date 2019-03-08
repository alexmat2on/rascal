/* RASCAL: Pascal (subset) Compiler implemented in Rust
 *
 * Author: Alexander Matson
 * Course: CSC 42000 Compiler Construction
 * Professor: Vulis
 */
use std::env;
use std::process;

use rascal;
use rascal::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("No input file specified: {}", err);
        process::exit(1);
    });

    if let Err(e) = rascal::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
