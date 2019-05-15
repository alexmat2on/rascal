use std::collections::HashMap;
use crate::tokens::Token;
use crate::tokens::TokenType;

#[derive(Debug)]
pub struct SymbEntry {
    pub tokentype: TokenType,
    pub address: Option<u32>,
}

impl SymbEntry {
    pub fn new(tokentype: TokenType) -> SymbEntry {
        SymbEntry {tokentype, address: None}
    }
}

#[derive(Debug)]
pub struct SymbTab {
    pub table: HashMap<String, SymbEntry>
}

impl SymbTab {
    pub fn new(pairs: Vec<Token>) -> SymbTab {
        let mut table = HashMap::new();
        for p in pairs {
            table.insert(p.token_value, SymbEntry::new(p.token_type));
        }

        SymbTab { table }
    }

    pub fn add(&mut self, tok: Token) {
        // Add the token to the symbol table only if it doesn't already exist
        self.table.entry(tok.token_value).or_insert(SymbEntry::new(tok.token_type));
    }

    pub fn set_entry(&mut self, tok: &Token, new: &Token) {
        let current = self.table.get(&tok.token_value.clone()).unwrap();
        let mut updated = SymbEntry::new(new.token_type);
        updated.address = current.address.clone();

        self.table.insert(tok.token_value.clone(), updated);
    }

    pub fn set_addr(&mut self, tok: &Token, addr: u32) {
        let default = SymbEntry::new(tok.token_type);
        let mut updated = SymbEntry::new(tok.token_type);
        updated.address = Some(addr);

        *self.table.entry(tok.token_value.clone()).or_insert(default) = updated;
    }

    pub fn get_addr(&mut self, tok: &Token) -> Option<u32> {
        self.table.get(&tok.token_value.clone()).unwrap().address
    }
}
