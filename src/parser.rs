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
*       <stat-decl> -> var <namelist>;
*       <namelist> -> id, <namelist> | id
*
* The language grammar specification -- IN CLASS
*       <prog> -> <decls> <body>. EOF
*       <decls> -> var <namelist> <decl-tail> | const <namelist> <decl-tail> |... type, label, procedure, function
*       <namelist> -> id<namelist-tail>; : <type>
*       <namelist-tail> NULL | ,<namelist>
*       <decl-tail> -> NULL | ;<decls>
*       <body> -> <begin-st>
*       <begin-st> -> begin <stats> <end>
*       <stats> -> NULL | <stats><stat-tail>
*       <stat-tail> -> ;<stats>
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
        self.decls()?;
        self.body()?;
        self.match_tok(TokenType::Dot)?;
        self.match_tok(TokenType::Eof)?;
        self.gen.op("OP_EXIT");
        Ok(())
    }

    // === HELPERS ================================================================================
    fn check_tok(&mut self, tok: TokenType) -> Result<(), String> {
        if tok != self.scan.cur_token.token_type {
            let errmsg = parser_error(tok.to_str(), self.scan.cur_token.clone());
            Err(errmsg)
        } else {
            Ok(())
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
    fn decls(&mut self) -> Result<(), String> {
        while
        self.check_tok(TokenType::Var).is_ok()
        {
            self.match_tok(TokenType::Var)?;
            self.namelist()?;
            self.decl_tail()?;
        }
        Ok(())
    }

    fn namelist(&mut self) -> Result<(), String> {
        let cur_token = self.scan.cur_token.clone();
        self.match_tok(TokenType::Ident)?;

        // set the cur_token to AVar in symbol table and give the DATA address
        let mut update_tok = cur_token.clone();
        update_tok.token_type = TokenType::AVar;
        update_tok.token_addr = Some(self.gen.data_addr);
        self.scan.symbol_table.set_entry(&cur_token, &update_tok);

        // update the latest DATA adddress
        self.gen.data_addr += 4;

        self.namelist_tail()?;
        Ok(())
    }

    fn namelist_tail(&mut self) -> Result<(), String> {
        if self.scan.cur_token.token_type == TokenType::Comma {
            self.match_tok(TokenType::Comma)?;
            self.namelist()?;
        }
        Ok(())
    }

    fn decl_tail(&mut self) -> Result<(), String> {
        if self.scan.cur_token.token_type == TokenType::Semi {
            self.match_tok(TokenType::Semi)?;
            self.decls()?;
        }
        Ok(())
    }

    fn body(&mut self) -> Result<(), String> {
        self.begin_st()?;
        Ok(())
    }

    fn stats(&mut self) -> Result<(), String> {
        let tok = self.scan.cur_token.clone();
        while
        self.check_tok(TokenType::AVar).is_ok() ||
        self.check_tok(TokenType::Repeat).is_ok() ||
        self.check_tok(TokenType::Write).is_ok()
        {
            match tok.token_type {
                TokenType::AVar => self.assign_st()?,
                TokenType::Repeat => self.repeat_st()?,
                TokenType::Write => self.write_st()?,
                _ => panic!("???")
            }
            self.stats_tail()?;
        }
        Ok(())
    }

    fn stats_tail(&mut self) -> Result<(), String> {
        self.match_tok(TokenType::Semi)?;
        self.stats()?;
        Ok(())
    }

    // === STATEMENT HANDLERS =====================================================================
    fn begin_st(&mut self) -> Result<(), String> {
        self.match_tok(TokenType::Begin)?;
        self.stats()?;
        self.match_tok(TokenType::End)?;
        Ok(())
    }

    fn assign_st(&mut self) -> Result<(), String> {
        let cur_token = self.scan.cur_token.clone();    // Copy this for later
        self.match_tok(TokenType::AVar)?;
        self.gen.op("OP_PUSH");

        let addr_val = self.scan.symbol_table.get_addr(&cur_token).expect("ERR: Undeclared variable!");
        self.gen.data(addr_val.to_string(), "u32", 4);

        self.match_tok(TokenType::OpAssign)?;
        self.expression()?;
        self.gen.op("OP_STORE");
        Ok(())
    }

    fn repeat_st(&mut self) -> Result<(), String> {
        self.match_tok(TokenType::Repeat)?;
        let label = self.gen.i_ptr;
        println!("Whats label? {}\n", label);
        self.stats()?;
        self.match_tok(TokenType::Until)?;
        // self.condition()?;
        self.expression()?;

        self.gen.op("OP_PUSH");
        self.gen.data(label.to_string(), "u32", 4);
        self.gen.op("OP_JTRUE");

        Ok(())
    }

    fn write_st(&mut self) -> Result<(), String> {
        self.match_tok(TokenType::Write)?;
        self.match_tok(TokenType::LParen)?;
        self.expression()?;
        self.match_tok(TokenType::RParen)?;

        self.gen.op("OP_WRITE");
        Ok(())
    }

    // === EXPRESSION PARSERS =====================================================================
    fn expression(&mut self) -> Result<(), String> {
        self.term()?;
        while
        self.check_tok(TokenType::OpPlus).is_ok() ||
        self.check_tok(TokenType::OpMinus).is_ok()
        {
            let tok = self.scan.cur_token.clone();
            self.match_tok(tok.token_type)?;
            self.term()?;
            self.gen.op(tok.to_op());
        };
        Ok(())
    }

    fn term(&mut self) -> Result<(), String> {
        self.factor()?;
        while
        self.check_tok(TokenType::OpMult).is_ok() ||
        self.check_tok(TokenType::OpDivi).is_ok()
        {
            let tok = self.scan.cur_token.clone();
            self.match_tok(tok.token_type)?;
            self.factor()?;
            self.gen.op(tok.to_op());
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
            TokenType::AVar => {
                // Push immediate the address for the variable onto the stack.
                self.gen.op("OP_PUSH");
                let tok_addr = self.scan.symbol_table.get_addr(&tok).expect("ERR: Variable is undeclared!");
                self.gen.data(tok_addr.to_string(), "u32", 4);

                // Add the LOAD operation which takes the top of the stack and puts it into address
                // at DATA segment.
                self.gen.op("OP_LOAD");

                self.match_tok(TokenType::AVar)?;
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
