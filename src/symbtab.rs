use std::collections::HashMap;
use crate::tokens::Token;

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

    pub fn set_entry(&mut self, tok: &Token, new: &Token) {
        let updated = new.clone();
        *self.table.entry(tok.token_value.clone()).or_insert_with(|| tok.clone()) = updated;
    }

    pub fn get_addr(&mut self, tok: &Token) -> Option<u32> {
        self.table.get(&tok.token_value.clone()).unwrap().token_addr
    }
}
