/*
* PARSER module
*
* The Parser -- implemented as an LL(1) / top-down recursive descent parser --
* will read a token and apply a grammar production rule to it.
*
* The language grammar specification
*       <S> -> <decl> <prog> EOF
*       <decl> -> <stat-decl>;
*       <prog> -> begin <stat> end
*       <stat> -> <stat-decl>; | <expr>;
*       <stat-decl> -> var id
*
* The expression grammer specification (removing instances of immediate left recursion) is as follows:
*       E  -> TE'
*       E' -> +TE' | -TE' | NULL
*
*       T  -> FT'
*       T' -> *FT' | /FT' | NULL
*
*       F  -> (E) | lit | id | +F | -F
*/
use crate::errors::parser_error;
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
        self.declar()?;
        self.program()?;
        self.match_tok(TokenType::Eof)?;
        self.gen.op("OP_EXIT");
        Ok(())
    }

    // === HELPERS ================================================================================
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

    // === GRAMMAR PRODUCTIONS ====================================================================
    fn declar(&mut self) -> Result<(), String> {
        self.stat_decl()?;
        self.match_tok(TokenType::Semi)?;

        Ok(())
    }

    fn program(&mut self) -> Result<(), String> {
        self.match_tok(TokenType::Begin)?;
        while self.check_tok(TokenType::End).is_err() {
            self.stat()?;
        }
        self.match_tok(TokenType::End)?;
        self.match_tok(TokenType::Dot)?;
        Ok(())
    }

    fn stat(&mut self) -> Result<(), String> {
        if self.stat_assign().is_err() {
            self.expression()?;
        }
        self.match_tok(TokenType::Semi)?;
        Ok(())
    }

    fn stat_assign(&mut self) -> Result<(), String> {
        let cur_token = self.scan.cur_token.clone();    // Copy this for later
        self.match_tok(TokenType::Ident)?;
        self.gen.op("OP_PUSH");

        let addr_val = self.scan.symbol_table.get_addr(&cur_token).expect("ERR: Undeclared variable!");
        self.gen.data(addr_val.to_string(), "u32", 4);

        self.match_tok(TokenType::OpAssign)?;
        self.expression()?;
        self.gen.op("OP_STORE");
        Ok(())
    }

    fn stat_decl(&mut self) -> Result<(), String> {
        self.match_tok(TokenType::Var)?;

        let cur_token = self.scan.cur_token.clone();
        self.match_tok(TokenType::Ident)?;

        let addr_val = self.gen.data_addr;
        self.scan.symbol_table.set_addr(&cur_token, addr_val);
        self.gen.data_addr += 4;

        while self.check_tok(TokenType::Semi).is_err() {
            self.match_tok(TokenType::Comma)?;

            let cur_token = self.scan.cur_token.clone();
            self.match_tok(TokenType::Ident)?;

            let addr_val = self.gen.data_addr;
            self.scan.symbol_table.set_addr(&cur_token, addr_val);
            self.gen.data_addr += 4;
        }

        Ok(())
    }

    fn expression(&mut self) -> Result<(), String> {
        self.term()?;
        while
        self.check_tok(TokenType::OpPlus).is_ok() ||
        self.check_tok(TokenType::OpMinus).is_ok()
        {
            match self.scan.cur_token.token_type {
                TokenType::OpPlus => {
                    self.match_tok(self.scan.cur_token.token_type)?;
                    self.term()?;
                    self.gen.op("OP_ADD");
                },
                TokenType::OpMinus => {
                    self.match_tok(self.scan.cur_token.token_type)?;
                    self.term()?;
                    self.gen.op("OP_SUB");
                },
                _ => ()
            }
        };
        Ok(())
    }

    fn term(&mut self) -> Result<(), String> {
        self.factor()?;
        while
        self.check_tok(TokenType::OpMult).is_ok() ||
        self.check_tok(TokenType::OpDivi).is_ok()
        {
            match self.scan.cur_token.token_type {
                TokenType::OpMult => {
                    self.match_tok(TokenType::OpMult)?;
                    self.factor()?;
                    self.gen.op("OP_MULT");
                },
                TokenType::OpDivi => {
                    self.match_tok(TokenType::OpDivi)?;
                    self.factor()?;
                    self.gen.op("OP_DIVI");
                },
                _ => ()
            };
        };

        Ok(())
    }

    fn factor(&mut self) -> Result<(), String> {
        let tok = &self.scan.cur_token;
        match tok.token_type {
            TokenType::IntLit => {
                // Push immediate the integer literal onto the stack.
                self.gen.op("OP_PUSH");
                self.gen.data(tok.token_value.clone(), "u32", 4);

                self.match_tok(TokenType::IntLit)?;
            },
            TokenType::Ident => {
                // Push immediate the address for the variable onto the stack.
                self.gen.op("OP_PUSH");
                let tok_addr = self.scan.symbol_table.get_addr(&tok).expect("ERR: Variable is undeclared!");
                self.gen.data(tok_addr.to_string(), "u32", 4);

                // Add the LOAD operation which takes the top of the stack and puts it into address
                // at DATA segment.
                self.gen.op("OP_LOAD");

                self.match_tok(TokenType::Ident)?;
            },
            TokenType::OpMinus => {
                self.match_tok(TokenType::OpMinus)?;
                self.factor()?;
                self.gen.op("OP_NEG");
            },
            TokenType::LParen => {
                self.match_tok(TokenType::LParen)?;
                self.expression()?;
                self.match_tok(TokenType::RParen)?;
            },
            _ => {
                let errmsg = parser_error("TK_INTLIT, TK_IDENT, TK_MINUS, or TK_LPAREN", self.scan.cur_token.clone());
                return Err(errmsg)
            }
        };

        Ok(())
    }
}
