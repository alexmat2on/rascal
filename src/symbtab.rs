use std::collections::HashMap;
use crate::tokens::TokenType;

pub struct SymbTab {
    pub table: HashMap<String, SymbEntry>
}

pub struct SymbEntry {
    tokentype: TokenType,
}

impl SymbTab<String, TokenType> {
    pub fn new<T, U>(pairs: Vec<(T, U)>) -> SymbTab<T, U> {
        let table = HashMap::new();
        for p in pairs {
            table.insert(p.0, p.1)
        }
        SymbTab { table: HashMap::new() }
    }
}
