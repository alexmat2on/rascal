use std::collections::HashMap;
use std::fmt;

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

    pub fn set_entry(&mut self, tok: &Token, new: &Token) {
        let current = self.table.get(&tok.token_value.clone()).unwrap();
        let mut updated = new.clone();
        updated.token_addr = current.token_addr.clone();

        self.table.insert(tok.token_value.clone(), updated);
        println!("THE SET EJTRY!: {:?}", self.table);
    }

    pub fn set_addr(&mut self, tok: &Token, addr: u32) {
        let default = tok.clone();
        let mut updated = tok.clone();
        // updated.token_type = 
        updated.token_addr = Some(addr);

        *self.table.entry(tok.token_value.clone()).or_insert(default) = updated;
    }

    pub fn get_addr(&mut self, tok: &Token) -> Option<u32> {
        self.table.get(&tok.token_value.clone()).unwrap().token_addr
    }
}

// impl fmt::Display for SymbTab {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // for ent in self.table.iter() {
//         //     write!(f, "ENT: {:?}", ent);
//         // }
//         // match *self {
//         //     CharGroup::INVLD => write!(f, "INVLD"),
//         //     CharGroup::WHITE => write!(f, "WHITE"),
//         //     CharGroup::PUNCT => write!(f, "PUNCT"),
//         //     CharGroup::DIGIT => write!(f, "DIGIT"),
//         //     CharGroup::ALPHA => write!(f, "ALPHA"),
//         // }
//     }
// }
