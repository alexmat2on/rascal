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
use crate::errors::parser_error;
// use crate::tokens::Token;
use crate::tokens::TokenType;
use crate::scanner::Scanner;

use crate::codegen::CodeGenerator;
use crate::codegen::rvm_gen::RvmGenerator;

enum Type {
    I, R, B, C
}

pub struct Parser {
    scan : Scanner,
    pub gen: RvmGenerator,
}

#[allow(non_snake_case)]
impl Parser {
    pub fn new (scan : Scanner) -> Parser {
        Parser {
            scan,
            gen: RvmGenerator::new()
        }
    }

    pub fn parse(&mut self) -> Result<(), String> {
        self.parse_t()?;
        self.parse_ep()?;
        self.match_tok(TokenType::Eof)?;
        self.gen.op("OP_EXIT");
        Ok(())
    }

    fn check_tok(&mut self, tok: TokenType) -> Result<(), String> {
        if tok != self.scan.cur_token.token_type {
            let errmsg = parser_error(tok.to_str(), self.scan.cur_token.clone());
            return Err(errmsg)
        } else {
            return Ok(())
        }
    }

    fn match_tok(&mut self, tok: TokenType) -> Result<(), String> {
        self.check_tok(tok)?;

        match self.scan.get_token() {
            Ok(()) => Ok(()),
            Err(e) => Err(e)
        }
    }

    fn parse_ep(&mut self) -> Result<(), String> {
        if self.scan.cur_token.token_type == TokenType::OpPlus {
            let decorator = self.scan.cur_token.clone();
            self.match_tok(TokenType::OpPlus)?;
            self.parse_t()?;
            self.parse_ep()?;
            self.gen.op("OP_ADD");
        }
        Ok(())
    }

    fn parse_t(&mut self) -> Result<(), String> {
        if self.scan.cur_token.token_type == TokenType::IntLit {
            self.gen.op("OP_PUSH");
            self.gen.data(self.scan.cur_token.token_value.clone(), "u32", 4);

            self.match_tok(TokenType::IntLit)?;
        } else {
            let errmsg = parser_error("TK_INTLIT", self.scan.cur_token.clone());
            return Err(errmsg)
        }
        Ok(())
    }
}
