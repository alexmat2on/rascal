/*
 * SCANNER module
 *
 * The Scanner will read the source file into a byte vector, and perform operations on it.
 * Mainly, the Scanner will maintain an index (`scan_ptr`) to the beginning of the currently
 * active Token which will be stored in the `cur_token` field.
 *
 * It exposes a public method to get the next token in the file
 */
use std::fs::File;
use std::io::Read;

mod tokens;
use tokens::Token;
use tokens::TokenType;
use tokens::ByteType;

pub struct Scanner {
    src_code: Vec<u8>,
    src_length: i32,
    scan_ptr: usize,
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
            println!("{}", *b);
        }
    }

    pub fn get_token(&self) -> &Token {
        // 1. Go through src_code to detect the next token
        // 2. Assign it to cur_token
        // 3. Return
        &self.cur_token
    }

    pub fn read_next_token(&mut self) -> Token {
        let mut byte = &self.src_code[self.scan_ptr];
        let first_type = tokens::get_byte_type(*byte);
        println!("\n==================================================");
        println!("RUNNING RNT: b - {:?}, t - {:?}", *byte, first_type);

        let mut token_value = vec![];

        loop {
            let cur_type = tokens::get_byte_type(*byte);
            if self.scan_ptr == (self.src_length - 1) as usize {
                break Token::new(TokenType::Null, String::from(""));
            }

            self.scan_ptr += 1;
            let next_byte = &self.src_code[self.scan_ptr];

            match first_type {
                ByteType::ALPHA => match cur_type {
                    ByteType::WHITE | ByteType::PUNCT => {
                        let token_value = String::from_utf8(token_value).expect("Invalid utf8");
                        break Token::new(TokenType::Ident, token_value);
                    },
                    _ => {/* do nothing yet */},
                },
                ByteType::DIGIT => match cur_type {
                    ByteType::WHITE | ByteType::ALPHA => {
                        let token_value = String::from_utf8(token_value).expect("Invalid utf8");
                        break Token::new(TokenType::IntLit, token_value);
                    },
                    ByteType::PUNCT => {
                        if *byte != 46 {
                            let token_value = String::from_utf8(token_value).expect("Invalid utf8");
                            break Token::new(TokenType::IntLit, token_value);
                        }
                    },
                    _ => {/* do nothing yet */},
                },
                ByteType::PUNCT => {
                    match *byte {
                        58 => {
                            // println!("{:?} - {:?} DEBUGG", *byte, *next_byte);
                            if *next_byte == 61 {
                                token_value.push(*byte);
                                token_value.push(*next_byte);
                                self.scan_ptr += 1;
                                let token_value = String::from_utf8(token_value).expect("Invalid utf8");
                                break Token::new(TokenType::OpAssign, token_value);
                            }
                        },
                        43 => {
                            break Token::new(TokenType::OpPlus, String::from("+"));
                        },
                        59 => {
                            break Token::new(TokenType::Semi, String::from(";"));
                        },
                        _ => {
                        /* do nothing yet */
                        // println!("{:?} DEBUGG", *byte);
                        }
                    }
                },
                ByteType::WHITE => break Token::new(TokenType::Sep, String::from(" ")),
                ByteType::INVLD => {
                    /* do nothing yet */
                },
            }

            token_value.push(*byte);

            println!("RNT: {}", byte);
            byte = &self.src_code[self.scan_ptr];
        }
    }
}
