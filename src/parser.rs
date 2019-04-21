/*
* PARSER module
*
* The Parser -- implemented as an LL(1) / top-down recursive descent parser --
* will read a token and apply a grammar production rule to it.
*
* Grammar: E  -> TE'
*          E' -> +TE' | NULL
*          T  -> intlit


* E  -> TE'
* E' -> +TE' | -TE' | NULL
* T  -> FT'
* T' -> *FT' | /FT' | NULL
* F  -> lit
*
* The grammer specification (removing instances of immediate left recursion) is as follows:
*       E  -> TE'
*       E' -> +TE' | -TE' | NULL
*
*       T  -> FT'
*       T' -> *FT' | /FT' | NULL
*
*       F  -> (E) | lit | id | +F | -F
*/
use crate::scanner::Scanner;
use crate::tokens::Token;
use crate::tokens::TokenType;

enum Type {
    I, R, B, C
}

pub struct Parser {
    scan : Scanner,
    pub code : Vec<u8>,
}

#[allow(non_snake_case)]
impl Parser {
    pub fn new (scan : Scanner) -> Parser {
        Parser { scan, code: Vec::new() }
    }

    pub fn parse(&mut self) {
        println!("Parsing...");
        // println!("a {}", self.scan.cur_token.token_value);
        self.parse_t();
        self.parse_ep();
        self.match_tok(TokenType::Eof);
        self.code.push(0x00);
    }

    fn match_tok(&mut self, tok: TokenType) {
        if tok != self.scan.cur_token.token_type {
            panic!("Unexpected token");
        } else {
            self.scan.get_token();
        }
    }

    fn parse_ep(&mut self) {
        // println!("b {}", self.scan.cur_token.token_value);

        if self.scan.cur_token.token_type == TokenType::OpPlus {
            let decorator = self.scan.cur_token.clone();
            self.match_tok(TokenType::OpPlus);
            // println!("c {}", self.scan.cur_token.token_value);
            self.parse_t();
            self.parse_ep();
            println!("y {:?}", decorator);
            self.code.push(0x10);
        }
    }

    fn parse_t(&mut self) {
        println!("x {:?}", self.scan.cur_token);
        self.code.push(0x01);
        let value_int : u32 = self.scan.cur_token.token_value.parse().expect("Expected u32");
        let mut value_bytes = value_int.to_be_bytes().to_vec();
        self.code.append(&mut value_bytes);
        self.match_tok(TokenType::IntLit);
    }
}
