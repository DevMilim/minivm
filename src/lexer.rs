use crate::tokens::Token;

pub struct Lexer {
    pub code: Vec<char>,
    pub cursor: usize,
}

impl Lexer {
    pub fn new(code: &str) -> Self {
        Self {
            code: code.chars().collect(),
            cursor: 0,
        }
    }
    pub fn lex_all() -> Vec<Token> {
        let mut tokens = Vec::new();
        tokens
    }
}
