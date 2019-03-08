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

    pub fn read_next_token(&mut self) -> Option<Token> {
        if self.scan_ptr == (self.src_length - 0) as usize {
            // Quit if we are at the end of the source vector
            return None;
        }

        let mut byte = self.src_code[self.scan_ptr];
        let first_type = tokens::get_byte_type(byte);

        println!("\n==================================================");
        println!("RUNNING RNT: b - {:?}, t - {:?}, index = {:?}", byte, first_type, self.scan_ptr);

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
        let mut opt_type = Some(TokenType::Ident);

        loop {
            let mut byte = self.src_code[self.scan_ptr];
            let cur_type = tokens::get_byte_type(byte);

            println!("fellas {:?} {}", byte, self.scan_ptr);

            match cur_type {
                ByteType::WHITE => {
                    break;
                },
                ByteType::PUNCT => {
                    // Some punctuation will be allowed in Identifier names, some not
                    if (byte != 95) {
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

    fn match_digit (&mut self) -> (Option<TokenType>, Vec<u8>) {
        // match cur_type {
        //     ByteType::WHITE | ByteType::ALPHA => {
        //         let token_value = String::from_utf8(token_value).expect("Invalid utf8");
        //         Some(Token::new(TokenType::IntLit, token_value))
        //     },
        //     ByteType::PUNCT => {
        //         if byte != 46 {
        //             let token_value = String::from_utf8(token_value).expect("Invalid utf8");
        //             Some(Token::new(TokenType::IntLit, token_value))
        //         } else {
        //             None
        //         }
        //     },
        //     _ => {
        //         /* do nothing yet */
        //         None
        //     },
        // }
        (None, vec![])
    }

    fn match_punct (&mut self) -> (Option<TokenType>, Vec<u8>) {
        (None, vec![])
        // println!("Hello from punct {:?}", byte);
        // match byte {
        //     58 => {
        //         // println!("{:?} - {:?} DEBUGG", *byte, *next_byte);
        //         let mut next_byte : Option<u8> = None;
        //         let mut token_value = token_value.to_vec();
        //
        //         if self.scan_ptr + 1 < self.src_length as usize {
        //             next_byte = Some(self.src_code[self.scan_ptr + 1]);
        //         };
        //
        //         if let Some(61) = next_byte {
        //             token_value.push(byte);
        //             token_value.push(next_byte.unwrap());
        //             self.scan_ptr += 1;
        //             let token_value = String::from_utf8(token_value).expect("Invalid utf8");
        //             Some(Token::new(TokenType::OpAssign, token_value))
        //         } else {
        //             None
        //         }
        //     },
        //     43 => {
        //         Some(Token::new(TokenType::OpPlus, String::from("+")))
        //     },
        //     59 => {
        //         println!("Hello from here");
        //         Some(Token::new(TokenType::Semi, String::from(";")))
        //     },
        //     _ => {
        //         /* do nothing yet */
        //         // println!("{:?} DEBUGG", *byte);
        //         None
        //     }
        // }
    }

    fn match_white (&mut self) -> (Option<TokenType>, Vec<u8>) {
        (None, vec![])
    }

    fn match_invld (&mut self) -> (Option<TokenType>, Vec<u8>) {
        (None, vec![])
    }
    // end here
}
