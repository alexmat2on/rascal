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

use crate::errors::scanner_error;
use crate::tokens;
use crate::tokens::Token;
use crate::tokens::TokenType;
use crate::tokens::CharGroup;

use crate::symbtab::SymbTab;

pub struct Scanner {
    src_code: Vec<u8>,
    src_length: usize,
    scan_ptr: usize,
    pub cur_token: Token,
    pub line_num: usize,
    pub col_num: usize,
    pub symbol_table: SymbTab
}

fn load_buffer(filename: &str) -> Result<(Vec<u8>, usize), String> {
    let mut buffer = vec![];

    let mut open_file;
    match File::open(filename) {
        Ok(f) => open_file = f,
        Err(_e) => return Err(String::from("Could not open file."))
    };

    let f_length;
    match open_file.read_to_end(&mut buffer) {
        Ok(x) => f_length = x,
        Err(_e) => return Err(String::from("Could not read file to buffer."))
    };

    Ok((buffer, f_length))
}

impl Scanner {
    pub fn new(filename : &str) -> Result<Scanner, String> {
        let src_load = load_buffer(filename)?;
        let cur_token = Token::new(TokenType::Null, String::from(""), (0, 0));

        let mut res = Scanner {
            src_code: src_load.0,
            src_length: src_load.1,
            scan_ptr: 0,
            cur_token: cur_token.clone(),
            line_num: 1,
            col_num: 1,
            symbol_table: SymbTab::new(vec![cur_token])
        };

        res.get_token()?;
        Ok(res)
    }

    pub fn get_token(&mut self) -> Result<(), String> {
        // Create operator table
        if self.src_length == 0 {
            let errmsg = "Err: Empty source file.";
            return Err(errmsg.to_string())
        };

        let maybe_eof = self.check_eof();
        self.cur_token = match maybe_eof {
            Some(eof) => eof,
            None => {
                let first_char = tokens::get_char_group(self.get_char());
                let potential_token = match first_char {
                    // A integer or numeric literal
                    CharGroup::DIGIT => self.get_num_lit(),

                    // An identifier: variable, function name, ...
                    CharGroup::ALPHA => self.get_identifier(),

                    // A symbol, operator, ...
                    CharGroup::PUNCT => self.get_symb(),

                    // Skip character
                    CharGroup::WHITE => {
                        if self.get_char() == 10 || self.get_char() == 13 {
                            self.line_num += 1;
                            self.col_num = 0;
                        }

                        self.scan_ptr += 1;
                        self.col_num += 1;

                        match self.get_token() {
                            Ok(_t) => Ok(self.cur_token.clone()),
                            Err(e) => Err(e)
                        }
                    },

                    CharGroup::INVLD => {
                        // Scanner error
                        let character = self.get_char() as char;
                        let errmsg = scanner_error(
                            "Illegal character".to_string(),
                            character.to_string(),
                            self.line_num,
                            self.col_num,
                        );
                        Err(errmsg)
                    }
                };

                match potential_token {
                    Ok(token) => token,
                    Err(e) => return Err(e)
                }
            }
        };

        Ok(())
    }

    fn get_char(&self) -> u8 {
        self.src_code[self.scan_ptr]
    }

    fn check_eof(&self) -> Option<Token> {
        if self.scan_ptr == self.src_length - 1 {
            Some(Token::new(TokenType::Eof, String::from("#"), (self.line_num, self.col_num)))
        } else {
            None
        }
    }

    fn get_num_lit(&mut self) -> Result<Token, String> {
        let mut value = vec![];
        let mut ttype = TokenType::IntLit;
        let cnum = self.col_num;

        loop {
            let char = self.get_char();
            let char_g = tokens::get_char_group(char);

            let next = self.get_char();
            let next_g = tokens::get_char_group(next);

            match char_g {
                CharGroup::DIGIT => value.push(char),
                CharGroup::PUNCT => {
                    if char == 46 && next_g != CharGroup::DIGIT {
                        break;
                    };

                    if char == 46 && ttype == TokenType::IntLit {
                        ttype = TokenType::RealLit;
                        value.push(char);
                    } else {
                        break;
                    };
                },
                _ => break,
            }

            self.scan_ptr += 1;
            self.col_num += 1;
        }

        let value_str : String;
        match String::from_utf8(value) {
            Ok(vstr) => value_str = vstr,
            Err(_e) => return Err(String::from("A UTF-8 Error Occurred"))
        }

        Ok(Token::new(ttype, value_str, (self.line_num, cnum)))
    }

    fn get_identifier(&mut self) -> Result<Token, String> {
        let mut value = vec![];
        let cnum = self.col_num;

        loop {
            let char = self.get_char();
            let char_g = tokens::get_char_group(char);

            match char_g {
                CharGroup::ALPHA => value.push(char),
                CharGroup::DIGIT => value.push(char),
                _ => break,
            }

            self.scan_ptr += 1;
            self.col_num += 1;
        }

        let value_str : String;
        match String::from_utf8(value) {
            Ok(vstr) => value_str = vstr,
            Err(_e) => return Err(String::from("A UTF-8 Error Occurred"))
        }

        let ident_token = match &value_str[..] {
            "begin" => self.make_tok(TokenType::Begin, value_str, cnum),
            "end" => self.make_tok(TokenType::End, value_str, cnum),
            "var" => self.make_tok(TokenType::Var, value_str, cnum),
            "integer" => self.make_tok(TokenType::Integer, value_str, cnum),
            "array" => self.make_tok(TokenType::Array, value_str, cnum),
            "write" => self.make_tok(TokenType::Write, value_str, cnum),
            "repeat" => self.make_tok(TokenType::Repeat, value_str, cnum),
            "until" => self.make_tok(TokenType::Until, value_str, cnum),
            "while" => self.make_tok(TokenType::While, value_str, cnum),
            "do" => self.make_tok(TokenType::Do, value_str, cnum),
            "of" => self.make_tok(TokenType::Of, value_str, cnum),
            "if" => self.make_tok(TokenType::If, value_str, cnum),
            "then" => self.make_tok(TokenType::Then, value_str, cnum),
            "else" => self.make_tok(TokenType::Else, value_str, cnum),
            "and" => self.make_tok(TokenType::OpAnd, value_str, cnum),
            "or" => self.make_tok(TokenType::OpOr, value_str, cnum),
            _ => self.make_tok(TokenType::Ident, value_str, cnum),
        };

        // Add the scanned token to symbol table. The method will only insert if does not exist yet.
        self.symbol_table.add(ident_token.clone().unwrap());

        // Retrieve the newest version of this token from the symbol table
        Ok(self.symbol_table.get(ident_token.unwrap()))
    }

    fn get_symb(&mut self) -> Result<Token, String> {
        let mut value = vec![];
        let cnum = self.col_num;

        loop {
            let char = self.get_char();
            let char_g = tokens::get_char_group(char);

            if !value.is_empty() && char == 59 {
                // Never include semicolons in a token being scanned,
                // unless they appear alone.
                break;
            }

            match char_g {
                CharGroup::PUNCT => value.push(char),
                _ => break,
            }

            self.scan_ptr += 1;
            self.col_num += 1;
        };

        let value_str : String;
        match String::from_utf8(value) {
            Ok(vstr) => value_str = vstr,
            Err(_e) => return Err(String::from("A UTF-8 Error Occurred"))
        };

        match &value_str[..] {
            ":=" => self.make_tok(TokenType::OpAssign, value_str, cnum),
            "=" => self.make_tok(TokenType::OpEqual, value_str, cnum),
            "<>" => self.make_tok(TokenType::OpNEqual, value_str, cnum),
            "<" => self.make_tok(TokenType::OpLessThan, value_str, cnum),
            "<=" => self.make_tok(TokenType::OpLessThanEql, value_str, cnum),
            ">" => self.make_tok(TokenType::OpGreatThan, value_str, cnum),
            ">=" => self.make_tok(TokenType::OpGreatThanEql, value_str, cnum),
            "+" => self.make_tok(TokenType::OpPlus, value_str, cnum),
            "-" => self.make_tok(TokenType::OpMinus, value_str, cnum),
            "*" => self.make_tok(TokenType::OpMult, value_str, cnum),
            "/" => self.make_tok(TokenType::OpDivi, value_str, cnum),
            "(" => self.make_tok(TokenType::LParen, value_str, cnum),
            ")" => self.make_tok(TokenType::RParen, value_str, cnum),
            "[" => self.make_tok(TokenType::LBrack, value_str, cnum),
            "]" => self.make_tok(TokenType::RBrack, value_str, cnum),
            ";" => self.make_tok(TokenType::Semi, value_str, cnum),
            ":" => self.make_tok(TokenType::Colon, value_str, cnum),
            "," => self.make_tok(TokenType::Comma, value_str, cnum),
            "." => self.make_tok(TokenType::Dot, value_str, cnum),
            ".." => self.make_tok(TokenType::Range, value_str, cnum),
            _ => {
                let errmsg = scanner_error(
                    "Invalid operator or symbol".to_string(),
                    value_str,
                    self.line_num,
                    cnum
                );
                Err(errmsg)
            }
        }
    }

    fn make_tok(&self, tt : TokenType, value: String, col: usize) -> Result<Token, String> {
        Ok(Token::new(tt, value, (self.line_num, col)))
    }
}
