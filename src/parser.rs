/*
* PARSER module
*
* The Parser -- implemented as an LL(1) / top-down recursive descent parser --
* will read a token and apply a grammar production rule to it.
*
* The language grammar specification
*       <prog> -> <decls> <body>. EOF
*       <decls> -> var <namelist> <decl-tail> | const <namelist> <decl-tail> |... type, label, procedure, function
*       <namelist> -> id<namelist-tail>; : <type>
*       <namelist-tail> NULL | ,<namelist>
*       <decl-tail> -> NULL | ;<decls>
*       <body> -> <begin-st>
*       <begin-st> -> begin <stats> <end>
*       <stats> -> NULL | <repeat st><stat-tail> | <while st> ... <if st> ... ...
*       <stat-tail> -> ;<stats>
*       <repeat st> -> repeat <stats> until <condition>
*       <while st> -> while <condition> do <begin-st>
*       <if st> -> if <condition> then <stats> | if <condition> then <stats> else <stats>
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

        println!("The symbol table is: {:?}", self.scan.symbol_table);
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
            self.match_tok(TokenType::Colon)?;
            self.decl_type()?;
            self.decl_tail()?;
        }
        Ok(())
    }

    fn namelist(&mut self) -> Result<(), String> {
        let cur_token = self.scan.cur_token.clone();
        self.match_tok(TokenType::Ident)?;
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

    fn decl_type(&mut self) -> Result<(), String> {
        while
        self.check_tok(TokenType::Integer).is_ok() ||
        self.check_tok(TokenType::Array).is_ok() {
            match self.scan.cur_token.token_type {
                TokenType::Integer => {
                    self.match_tok(TokenType::Integer)?;

                    let num_int_vars = self.scan.symbol_table.set_idents_to(TokenType::AVar, self.gen.data_addr, 4);

                    self.gen.data_addr += num_int_vars * 4;
                },
                TokenType::Array => {
                    self.match_tok(TokenType::Array)?;
                    self.match_tok(TokenType::LBrack)?;

                    let lo = self.scan.cur_token.token_value.parse::<u32>().expect("Lo should be int lit.");
                    self.match_tok(TokenType::IntLit)?;

                    self.match_tok(TokenType::Range)?;

                    let hi = self.scan.cur_token.token_value.parse::<u32>().expect("Hi should be int lit.");
                    self.match_tok(TokenType::IntLit)?;

                    self.match_tok(TokenType::RBrack)?;
                    self.match_tok(TokenType::Of)?;

                    let size = (hi - lo + 1) * 4;
                    let num_arr_vars = self.scan.symbol_table.set_idents_to_arr(
                        TokenType::AnArrayVar,
                        self.gen.data_addr,
                        size,
                        lo,
                        hi
                    );
                    self.gen.data_addr += size;

                    self.decl_type()?;
                },
                _ => panic!("??"),
            }
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
        self.check_tok(TokenType::AnArrayVar).is_ok() ||
        self.check_tok(TokenType::Repeat).is_ok() ||
        self.check_tok(TokenType::While).is_ok() ||
        self.check_tok(TokenType::If).is_ok() ||
        self.check_tok(TokenType::Write).is_ok()
        {
            match tok.token_type {
                TokenType::AVar => self.assign_st()?,
                TokenType::AnArrayVar => self.assign_array_st()?,
                TokenType::Repeat => self.repeat_st()?,
                TokenType::While => self.while_st()?,
                TokenType::If => self.if_st()?,
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

    fn assign_array_st(&mut self) -> Result<(), String> {
        let arr_token = self.scan.cur_token.clone();    // Copy this for later
        self.match_tok(TokenType::AnArrayVar)?;
        self.match_tok(TokenType::LBrack)?;
        self.expression()?; // Parse the index value
        self.match_tok(TokenType::RBrack)?;
        self.match_tok(TokenType::OpAssign)?;

        println!("Lets check symbtab: {:?}", self.scan.symbol_table);

        // Convert the index value on the stack into an array element address!
        let lo = arr_token.low.expect("Array should have low value!");
        if lo != 0 {
            self.gen.op("OP_PUSH");
            self.gen.data(lo.to_string(), "u32", 4);
            self.gen.op("OP_SUB");
        }

        self.gen.op("OP_PUSH");
        self.gen.data("4".to_string(), "u32", 4);   // Push the size of array elements
        self.gen.op("OP_MULT");

        let addr = arr_token.token_addr.expect("Array should have an address by now!");
        self.gen.op("OP_PUSH");
        self.gen.data(addr.to_string(), "u32", 4); // Pushi the arrays base addr
        self.gen.op("OP_ADD");

        self.expression()?;
        self.gen.op("OP_STORE");
        Ok(())
    }

    fn repeat_st(&mut self) -> Result<(), String> {
        self.match_tok(TokenType::Repeat)?;
        let label = self.gen.i_ptr;

        self.stats()?;
        self.match_tok(TokenType::Until)?;
        self.expression()?; // Evaluate condition

        self.gen.op("OP_PUSH");
        self.gen.data(label.to_string(), "u32", 4);
        self.gen.op("OP_JTRUE");

        Ok(())
    }

    fn while_st(&mut self) -> Result<(), String> {
        self.match_tok(TokenType::While)?;
        let label = self.gen.i_ptr;

        self.expression()?; // Evaluate condition
        self.match_tok(TokenType::Do)?;


        self.gen.op("OP_PUSH");
        let hole = self.gen.i_ptr;
        self.gen.data("0".to_string(), "u32", 4);
        self.gen.op("OP_JFALSE");

        self.begin_st()?;

        self.gen.op("OP_PUSH");
        self.gen.data(label.to_string(), "u32", 4);
        self.gen.op("OP_JMP");

        let save = self.gen.i_ptr;
        self.gen.i_ptr = hole;

        self.gen.fill(save.to_string(), "u32", 4);
        self.gen.i_ptr = save;

        Ok(())
    }

    fn if_st(&mut self) -> Result<(), String> {
        self.match_tok(TokenType::If)?;
        self.expression()?; // Evaluate condition

        // Set up the jump with temporary 0 address
        self.gen.op("OP_PUSH");
        let hole = self.gen.i_ptr;
        self.gen.data("0".to_string(), "u32", 4);
        self.gen.op("OP_JFALSE");

        self.match_tok(TokenType::Then)?;
        self.begin_st()?;

        let hole2 = self.gen.i_ptr;
        if self.check_tok(TokenType::Else).is_ok() {
            self.gen.data("0".to_string(), "u32", 4);
            self.gen.op("OP_JMP");
        }

        // Now that statements are done, fill in the previous hole.
        let save = self.gen.i_ptr;
        self.gen.i_ptr = hole;
        self.gen.fill(save.to_string(), "u32", 4);
        self.gen.i_ptr = save;

        if self.check_tok(TokenType::Else).is_ok() {
            self.match_tok(TokenType::Else)?;
            self.begin_st()?;

            let save2 = self.gen.i_ptr;
            self.gen.i_ptr = hole2;
            self.gen.fill(save2.to_string(), "u32", 4);
            self.gen.i_ptr = save2;
        }
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
        self.check_tok(TokenType::OpMinus).is_ok() ||
        self.check_tok(TokenType::OpOr).is_ok()
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
        self.check_tok(TokenType::OpDivi).is_ok() ||
        self.check_tok(TokenType::OpAnd).is_ok() ||
        self.check_tok(TokenType::OpEqual).is_ok() ||
        self.check_tok(TokenType::OpNEqual).is_ok() ||
        self.check_tok(TokenType::OpLessThan).is_ok() ||
        self.check_tok(TokenType::OpLessThanEql).is_ok() ||
        self.check_tok(TokenType::OpGreatThan).is_ok() ||
        self.check_tok(TokenType::OpGreatThanEql).is_ok()
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
            TokenType::AnArrayVar => {
                let arr_token = self.scan.cur_token.clone();
                self.match_tok(TokenType::AnArrayVar)?;
                self.match_tok(TokenType::LBrack)?;
                self.expression()?; // Parse the index value
                self.match_tok(TokenType::RBrack)?;

                // Convert the index value on the stack into an array element address!
                let lo = arr_token.low.expect("Array should have low value!");
                if lo != 0 {
                    self.gen.op("OP_PUSH");
                    self.gen.data(lo.to_string(), "u32", 4);
                    self.gen.op("OP_SUB");
                }

                self.gen.op("OP_PUSH");
                self.gen.data("4".to_string(), "u32", 4);   // Push the size of array elements
                self.gen.op("OP_MULT");

                let addr = arr_token.token_addr.expect("Array should have an address by now!");
                self.gen.op("OP_PUSH");
                self.gen.data(addr.to_string(), "u32", 4); // Pushi the arrays base addr
                self.gen.op("OP_ADD");

                self.gen.op("OP_LOAD");
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
