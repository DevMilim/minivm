use crate::{Lexer, Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParserErrors {
    NotNumber,
    UnexpectedToken(TokenKind),
    CurrentNull,
    PeekNull,
    ExpectedIdentifier,
    UnexpectedEOF,
    Todo,
}

impl Parser {
    pub fn new_from_source(source: &str) -> Self {
        let lex = Lexer::new(source);
        let tokens = lex.lex_all().unwrap();
        Self { tokens, cursor: 0 }
    }
    pub fn parse_program() {}
    fn advance(&mut self) {
        self.cursor += 1;
    }
    fn peek(&self) -> Option<&TokenKind> {
        self.tokens.get(self.cursor + 1).and_then(|t| Some(&t.kind))
    }
    fn expect(&mut self, expected: TokenKind) -> Result<(), ParserErrors> {
        let current = self.current()?;
        if current == expected {
            self.advance();
            Ok(())
        } else {
            Err(ParserErrors::UnexpectedToken(current))
        }
    }
    fn eat(&mut self, expected: TokenKind) -> bool {
        if self.current() == Ok(expected) {
            self.advance();
            true
        } else {
            false
        }
    }
    fn current(&self) -> Result<TokenKind, ParserErrors> {
        self.tokens
            .get(self.cursor)
            .and_then(|t| Some(t.kind.clone()))
            .ok_or(ParserErrors::CurrentNull)
    }
}
