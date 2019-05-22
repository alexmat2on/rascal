use std::collections::HashMap;
use crate::tokens::Token;
use crate::tokens::TokenType;

#[derive(Debug)]
pub struct SymbTab {
    pub table: HashMap<String, Token>
}

impl SymbTab {
    pub fn new(pairs: Vec<Token>) -> SymbTab {
        let mut table = HashMap::new();
        for p in pairs {
            table.insert(p.token_value.clone(), p);
        }

        SymbTab { table }
    }

    pub fn add(&mut self, tok: Token) {
        // Add the token to the symbol table only if it doesn't already exist
        self.table.entry(tok.token_value.clone()).or_insert(tok);
    }

    pub fn get(&mut self, tok: Token) -> Token {
        self.table.get(&tok.token_value).expect("Token not found").clone()
    }

    pub fn set_idents_to(&mut self, ttype: TokenType, starting_addr: u32, dsize: u32) -> u32 {
        // Return number of idents this function has found.
        let mut count = 0;
        let mut addr = starting_addr;

        for token in &mut self.table.clone().values_mut() {
            if token.token_type == TokenType::Ident {
                count += 1;
                let mut new = token.clone();
                new.token_type = ttype;
                new.token_addr = Some(addr);

                self.set_entry(&token.clone(), &new);

                addr += dsize;
            }
        }

        count
    }

    pub fn set_idents_to_arr(&mut self, ttype: TokenType, starting_addr: u32, dsize: u32, lo: u32, hi: u32) -> u32 {
        // Return number of idents this function has found.
        let mut count = 0;
        let mut addr = starting_addr;

        for token in &mut self.table.clone().values_mut() {
            if token.token_type == TokenType::Ident {
                count += 1;
                let mut new = token.clone();
                new.token_type = ttype;
                new.token_addr = Some(addr);
                new.low = Some(lo);
                new.high = Some(hi);

                self.set_entry(&token.clone(), &new);

                addr += dsize;
            }
        }

        count
    }

    pub fn set_entry(&mut self, tok: &Token, new: &Token) {
        let updated = new.clone();
        *self.table.entry(tok.token_value.clone()).or_insert_with(|| tok.clone()) = updated;
    }

    pub fn get_addr(&mut self, tok: &Token) -> Option<u32> {
        self.table[&tok.token_value.clone()].token_addr
    }
}
