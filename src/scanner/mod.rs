/*
 * SCANNER module
 *
 * The Scanner will read the source file into a byte vector, and perform operations on it.
 * Mainly, the Scanner will maintain a pointer to the beginning of the currently active Token and
 * keep it stored in the `cur_token` field.
 *
 * It exposes a public method to get the next token in the file
 */
use std::fs::File;
use std::io::Read;

mod tokens;
use tokens::Token;
use tokens::TokenType;

pub struct Scanner {
    src_code: Vec<u8>,
    src_length: i32,
    scan_ptr: u32,
    cur_token: Token,
}

impl Scanner {
    pub fn new(filename : &String) -> Scanner {
        let mut buffer = vec![];

        let mut f = File::open(filename).unwrap();
        let len = f.read_to_end(&mut buffer).unwrap();

        Scanner {
            src_code: buffer,
            src_length: len as i32,
            scan_ptr: 0,
            cur_token: Token::new(TokenType::Null, String::from("")),
        }
    }

    pub fn print_types(&self) {
        for b in &self.src_code {
            let tp = tokens::get_byte_type(*b);
            println!("{}", tp);
        }
    }

    pub fn get_token(&self) -> &Token {
        // 1. Go through src_code to detect the next token
        // 2. Assign it to cur_token
        // 3. Return
        &self.cur_token
    }

    fn read_next_token(&self) {

    }
}
