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

use crate::tokens;
use crate::tokens::Token;
use crate::tokens::TokenType;
use crate::tokens::ByteType;

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

    pub fn get_token(&mut self) -> &Token {
        // 1. Go through src_code to detect the next token
        // 2. Assign it to cur_token
        // 3. Return
        let mut read_token = self.read_next_token();

        loop {
            if read_token.is_some() {
                self.cur_token = read_token.unwrap();
                break;
            }
            read_token = self.read_next_token();
        }

        &self.cur_token
    }

    pub fn reached_eof(&self) -> bool {
        self.scan_ptr == (self.src_length - 1) as usize
    }

    fn read_next_token(&mut self) -> Option<Token> {
        if self.scan_ptr == (self.src_length - 1) as usize {
            // Quit if we are at the end of the source vector
            println!("Reached end of token stream.");
            return Some(Token::new(TokenType::Eof, "\0".to_string()));
        }

        let byte = self.src_code[self.scan_ptr];
        let first_type = tokens::get_byte_type(byte);

        let (token_type, token_value) : (Option<TokenType>, Vec<u8>) = match first_type {
            ByteType::ALPHA => self.match_alpha(),
            ByteType::DIGIT => self.match_digit(),
            ByteType::PUNCT => self.match_punct(),
            ByteType::WHITE => self.match_white(),
            ByteType::INVLD => self.match_invld(),
        };

        match token_type {
            Some(toktype) => {
                let token_value = String::from_utf8(token_value).expect("Invalid utf8");
                Some(Token::new(toktype, token_value))
            },
            None => {
                self.scan_ptr += 1;
                None
            },
        }
    }

    fn match_alpha (&mut self) -> (Option<TokenType>, Vec<u8>) {
        let mut token_value = vec![];
        let opt_type = Some(TokenType::Ident);

        loop {
            let byte = self.src_code[self.scan_ptr];
            let cur_type = tokens::get_byte_type(byte);

            match cur_type {
                ByteType::WHITE => {
                    // let isKeyword = match_keyword (&self, String::from_utf8(token_value);
                    break;
                },
                ByteType::PUNCT => {
                    // Some punctuation will be allowed in Identifier names, some not
                    if byte != 95 {
                        break;
                    }
                }
                _ => {}
            }

            token_value.push(byte);
            self.scan_ptr += 1;
        };

        (opt_type, token_value)
    }

    // fn match_keyword (&self, tval : String) -> Option<TokenType> {
    //
    // }

    fn match_digit (&mut self) -> (Option<TokenType>, Vec<u8>) {
        let mut token_value = vec![];
        let mut opt_type = Some(TokenType::IntLit);

        let mut found_decimal = false;

        loop {
            let byte = self.src_code[self.scan_ptr];
            let cur_type = tokens::get_byte_type(byte);

            match cur_type {
                ByteType::WHITE => {
                    break;
                },
                ByteType::PUNCT => {
                    // Some punctuation will be allowed in numeric literals names, like "."
                    if byte != 46 || found_decimal {
                        break;
                    }

                    if byte == 46 {
                        found_decimal = true;
                        opt_type = Some(TokenType::RealLit);
                    }
                }
                _ => {}
            }

            token_value.push(byte);
            self.scan_ptr += 1;
        };

        (opt_type, token_value)
    }

    fn match_punct (&mut self) -> (Option<TokenType>, Vec<u8>) {
        let mut token_value = vec![];
        let mut opt_type : Option<TokenType> = None;

        loop {
            let byte = self.src_code[self.scan_ptr];
            let cur_type = tokens::get_byte_type(byte);

            // println!("fellas {:?} {}", byte, self.scan_ptr);

            match cur_type {
                ByteType::ALPHA => break,
                ByteType::WHITE => break,
                ByteType::DIGIT => break,
                ByteType::INVLD => break,
                ByteType::PUNCT => {
                    // Some punctuation will be allowed in Identifier names, some not
                    match byte {
                        40 => {
                            // (
                            opt_type = Some(TokenType::LParen);
                        },
                        41 => {
                            // )
                            opt_type = Some(TokenType::RParen);
                        },
                        42 => {
                            // *
                            opt_type = Some(TokenType::OpMult);
                        }
                        43 => {
                            // +
                            opt_type = Some(TokenType::OpPlus);
                        },
                        45 => {
                            // -
                            opt_type = Some(TokenType::OpMinus);
                        },
                        47 => {
                            // /
                            opt_type = Some(TokenType::OpDivi);
                        }
                        58 => {
                            // :
                            let next_byte = self.src_code[self.scan_ptr + 1];
                            if next_byte == 61 {
                                opt_type = Some(TokenType::OpAssign);
                                token_value.push(byte);
                                token_value.push(next_byte);
                                self.scan_ptr += 2;
                                break;
                            }
                        },
                        59 => {
                            // ;
                            opt_type = Some(TokenType::Semi);
                        },
                        _ => {
                            opt_type = None;
                        }
                    }
                }
            }

            token_value.push(byte);
            self.scan_ptr += 1;
        };

        (opt_type, token_value)
    }

    fn match_white (&mut self) -> (Option<TokenType>, Vec<u8>) {
        (None, vec![])
    }

    fn match_invld (&mut self) -> (Option<TokenType>, Vec<u8>) {
        (None, vec![])
    }
    // end here
}
