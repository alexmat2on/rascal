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
        self.expression()?;
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

    fn expression(&mut self) -> Result<(), String> {
        self.term()?;
        self.expression_p()?;

        self.gen.op("OP_EXIT");
        Ok(())
    }

    fn expression_p(&mut self) -> Result<(), String> {
        let tok = self.scan.cur_token.clone();
        match tok.token_type {
            TokenType::OpPlus => {
                self.match_tok(TokenType::OpPlus)?;
                self.term()?;
                self.gen.op(tok.to_op());
                self.expression_p()?;
            },
            TokenType::OpMinus => {
                self.match_tok(TokenType::OpMinus)?;
                self.term()?;
                self.gen.op(tok.to_op());
                self.expression_p()?;
            },
            _ => ()
        };
        Ok(())
    }

    fn term(&mut self) -> Result<(), String> {
        self.factor()?;
        self.term_p()?;
        Ok(())
    }

    fn term_p(&mut self) -> Result<(), String> {
        let tok = self.scan.cur_token.clone();
        match tok.token_type {
            TokenType::OpMult => {
                self.match_tok(TokenType::OpMult)?;
                self.factor()?;
                self.gen.op(tok.to_op());
                self.term_p()?;
            },
            TokenType::OpDivi => {
                self.match_tok(TokenType::OpDivi)?;
                self.factor()?;
                self.gen.op(tok.to_op());
                self.term_p()?;
            },
            _ => ()
        };
        Ok(())
    }

    fn factor(&mut self) -> Result<(), String> {
        let tok = &self.scan.cur_token;
        match tok.token_type {
            TokenType::IntLit => {
                self.gen.op("OP_PUSH");
                self.gen.data(tok.token_value.clone(), "u32", 4);

                self.match_tok(TokenType::IntLit)?;
            },
            _ => {
                let errmsg = parser_error("TK_INTLIT", self.scan.cur_token.clone());
                return Err(errmsg)
            }
        }
        Ok(())
    }
}
