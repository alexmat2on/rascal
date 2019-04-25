use crate::tokens::Token;

pub fn scanner_error(msg: String, value : String, lineno : usize, colno : usize) -> String {
    let msg = format!(
        "Scanner Error: {} [ {} ] ({}:{})",
        msg,
        value,
        lineno,
        colno,
    );

    msg.to_string()
}

pub fn parser_error(expected: &str, got: Token) -> String {
    let msg = format!(
        "Parser Error: Expected {} but found {} ({}:{})",
        expected,
        got.token_type.to_str(),
        got.src_info.0,
        got.src_info.1
    );

    msg.to_string()
}
