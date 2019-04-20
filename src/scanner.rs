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
use crate::tokens::CharGroup;
// use crate::symtab::SymbTab;

pub struct Scanner {
    src_code: Vec<u8>,
    src_length: usize,
    scan_ptr: usize,
    pub cur_token: Token,
}

impl Scanner {
    pub fn new(filename : &String) -> Scanner {
        let mut buffer = vec![];

        let mut f = File::open(filename).unwrap();
        let len = f.read_to_end(&mut buffer).unwrap();

        let mut res = Scanner {
            src_code: buffer,
            src_length: len,
            scan_ptr: 0,
            cur_token: Token::new(TokenType::Null, String::from("")),
        };
        res.get_token();
        res
    }

    pub fn get_token(&mut self) {
        // Create operator table
        let maybe_eof = self.check_eof();
        self.cur_token = match maybe_eof {
            Some(eof) => eof,
            None => {
                let first_char = tokens::get_char_group(self.get_char());
                match first_char {
                    CharGroup::DIGIT => self.get_num_lit(), // A integer or numeric literal
                    CharGroup::ALPHA => self.get_identifier(), // An identifier: variable, function name, ...
                    // CharGroup::QUOTE => (), // A string or char literal... ?? will i keep this?
                    CharGroup::PUNCT => self.get_symb(), // A symbol, operator, ...
                    CharGroup::WHITE => {
                        // Skip character
                        self.scan_ptr += 1;
                        self.get_token();

                        self.cur_token.clone()
                    },
                    CharGroup::INVLD | _ => {
                        // Scanner error
                        panic!("omg no");
                    }
                }
            }
        }
    }

    pub fn get_char(&self) -> u8 {
        self.src_code[self.scan_ptr]
    }

    fn check_eof(&self) -> Option<Token> {
        if self.scan_ptr == self.src_length - 1 {
            Some(Token::new(TokenType::Eof, String::from("#")))
        } else {
            None
        }
    }

    fn get_num_lit(&mut self) -> Token {
        let mut value = vec![];
        let mut ttype = TokenType::IntLit;

        loop {
            let char = self.get_char();
            let char_g = tokens::get_char_group(char);

            match char_g {
                CharGroup::DIGIT => value.push(char),
                CharGroup::PUNCT => {
                    if char == 46 && ttype == TokenType::IntLit {
                        ttype = TokenType::RealLit;
                        value.push(char);
                    } else {
                        break;
                    }
                }
                _ => break,
            }

            self.scan_ptr += 1;
        }

        let value_str = String::from_utf8(value).expect("Found invalid UTF-8");
        Token::new(ttype, value_str)
    }

    fn get_identifier(&self) -> Token {
        Token::new(TokenType::Null, String::from(""))
    }

    fn get_symb(&mut self) -> Token {
        let mut value = vec![];

        loop {
            let char = self.get_char();
            let char_g = tokens::get_char_group(char);

            match char_g {
                CharGroup::PUNCT => value.push(char),
                _ => break,
            }

            self.scan_ptr += 1;
        }

        let value_str = String::from_utf8(value).expect("Found invalid UTF-8");
        match &value_str[..] {
            "+" => Token::new(TokenType::OpPlus, value_str),
            "-" => Token::new(TokenType::OpMinus, value_str),
            _ => panic!("Invalid operator")
        }
    }
}
