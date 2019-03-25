/*
* PARSER module
*
* The Parser -- implemented as an LL(1) / top-down recursive descent parser --
* will read a token and apply a grammar production rule to it.
*
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

pub struct Parser {
    scan : Scanner,
}

#[allow(non_snake_case)]
impl Parser {
    pub fn new (scan : Scanner) -> Parser {
        Parser { scan: scan }
    }

    pub fn parse(&mut self) {
        while !self.scan.reached_eof() {
            // parser.E(scan.get_token());
            println!("tok: {:?}", self.scan.get_token());
        }
    }

    fn E (tok : Token) {
        match tok.token_type {
            TokenType::OpPlus => {

            },
            TokenType::OpMinus => {

            },
            _ => {},
        }
    }

    // fn E_ (tok : Token) {
    //
    // }
    //
    // fn T (tok : Token) {
    //
    // }
    //
    // fn T_ (tok : Token) {
    //
    // }
    //
    // fn F (tok : Token) {
    //
    // }
}
