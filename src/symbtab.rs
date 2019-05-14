use std::collections::HashMap;
use crate::tokens::Token;
use crate::tokens::TokenType;

pub struct SymbEntry {
    tokentype: TokenType,
}

impl SymbEntry {
    pub fn new(tokentype: TokenType) -> SymbEntry {
        SymbEntry {tokentype}
    }
}

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
        self.table.insert(tok.token_value, SymbEntry::new(tok.token_type));
    }

    // pub fn get(&self, key: &String) -> &SymbEntry {
    //     self.table.get(key).expect("Invalid symbol key.")
    // }
    //
    // pub fn overwrite(&mut self, key: String, new_val: SymbEntry) {
    //
    // }
}
