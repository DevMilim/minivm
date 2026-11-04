use crate::{
    Keyword,
    tokens::{Span, Token, TokenKind},
};

#[derive(Debug)]
pub struct Lexer {
    pub chars: Vec<char>,
    pub cursor: usize,
}

#[derive(Debug)]
pub enum LexerError {
    InvalidChar(char),
    FirstCharIdentNotAccept,
}

impl Lexer {
    pub fn new(code: &str) -> Self {
        Self {
            chars: code.chars().collect(),
            cursor: 0,
        }
    }
    pub fn lex_all(mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        loop {
            let token = self.lex_token()?;

            let mut is_eof = false;
            if token.kind == TokenKind::EOF {
                is_eof = true
            }

            println!("Token: {:?}", token);
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        Ok(tokens)
    }
    fn advance(&mut self) {
        self.cursor += 1;
    }
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn lex_token(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace();
        let start = self.cursor;
        if let Some(char) = self.current() {
            let token = match char {
                c if c.is_alphabetic() || c == '_' => {
                    let ident = self.read_ident()?;
                    let token = match ident.as_str() {
                        "fn" => TokenKind::Keyword(Keyword::Fn),
                        "let" => TokenKind::Keyword(Keyword::Let),
                        _ => TokenKind::Identifier(ident),
                    };
                    Ok(token)
                }
                c if c.is_numeric() => {
                    let value = self.number()?;
                    Ok(TokenKind::Number(value))
                }
                c if c == '"' => {
                    let value = self.read_string();
                    Ok(TokenKind::String(value))
                }
                ';' => {
                    self.advance();
                    Ok(TokenKind::Semicolon)
                }

                '.' => {
                    if self.peek() == Some(&'.') {
                        self.advance();
                        self.advance();
                        Ok(TokenKind::DotDot)
                    } else {
                        self.advance();
                        Ok(TokenKind::Dot)
                    }
                }
                ',' => {
                    self.advance();
                    Ok(TokenKind::Comma)
                }
                ':' => {
                    if self.peek() == Some(&':') {
                        self.advance();
                        self.advance();
                        Ok(TokenKind::DoubleColon)
                    } else {
                        self.advance();
                        Ok(TokenKind::Colon)
                    }
                }
                '(' => {
                    self.advance();
                    Ok(TokenKind::LParen)
                }
                ')' => {
                    self.advance();
                    Ok(TokenKind::RParen)
                }
                '{' => {
                    self.advance();
                    Ok(TokenKind::LBrace)
                }
                '}' => {
                    self.advance();
                    Ok(TokenKind::RBrace)
                }

                '[' => {
                    self.advance();
                    Ok(TokenKind::LBracket)
                }
                ']' => {
                    self.advance();
                    Ok(TokenKind::RBracket)
                }

                '+' => {
                    if self.peek() == Some(&'=') {
                        self.advance();
                        self.advance();
                        Ok(TokenKind::PlusAssign)
                    } else {
                        self.advance();
                        Ok(TokenKind::Plus)
                    }
                }
                '-' => {
                    if self.peek() == Some(&'=') {
                        self.advance();
                        self.advance();
                        Ok(TokenKind::MinusAssign)
                    } else {
                        self.advance();
                        Ok(TokenKind::Minus)
                    }
                }
                '*' => {
                    if self.peek() == Some(&'=') {
                        self.advance();
                        self.advance();
                        Ok(TokenKind::MultiplyAssign)
                    } else {
                        self.advance();
                        Ok(TokenKind::Multiply)
                    }
                }
                '/' => {
                    if self.peek() == Some(&'=') {
                        self.advance();
                        self.advance();
                        Ok(TokenKind::DivideAssign)
                    } else {
                        self.advance();
                        Ok(TokenKind::Divide)
                    }
                }
                '=' => {
                    if self.peek() == Some(&'=') {
                        self.advance();
                        self.advance();
                        Ok(TokenKind::EqualEqual)
                    } else {
                        self.advance();
                        Ok(TokenKind::Assign)
                    }
                }
                '<' => {
                    if self.peek() == Some(&'=') {
                        self.advance();
                        self.advance();
                        Ok(TokenKind::LessEqual)
                    } else if self.peek() == Some(&'<') {
                        self.advance();
                        if self.peek() == Some(&'=') {
                            self.advance();
                            self.advance();
                            Ok(TokenKind::LShiftAssign)
                        } else {
                            self.advance();
                            Ok(TokenKind::LShift)
                        }
                    } else {
                        self.advance();
                        Ok(TokenKind::Less)
                    }
                }
                '>' => {
                    if self.peek() == Some(&'=') {
                        self.advance();
                        self.advance();
                        Ok(TokenKind::GreaterEqual)
                    } else if self.peek() == Some(&'>') {
                        self.advance();
                        if self.peek() == Some(&'=') {
                            self.advance();
                            self.advance();
                            Ok(TokenKind::RShiftAssign)
                        } else {
                            self.advance();
                            Ok(TokenKind::RShift)
                        }
                    } else {
                        self.advance();
                        Ok(TokenKind::Greater)
                    }
                }
                '!' => {
                    if self.peek() == Some(&'=') {
                        self.advance();
                        self.advance();
                        Ok(TokenKind::NotEqual)
                    } else {
                        self.advance();
                        Ok(TokenKind::Not)
                    }
                }
                _ => Err(LexerError::InvalidChar(char)),
            }?;
            let end = self.cursor;
            Ok(Token {
                kind: token,
                span: Span { start, end },
            })
        } else {
            let end = self.cursor;
            Ok(Token {
                kind: TokenKind::EOF,
                span: Span { start, end },
            })
        }
    }

    fn peek(&self) -> Option<&char> {
        self.chars.get(self.cursor + 1)
    }
    fn number(&mut self) -> Result<String, LexerError> {
        let mut value = String::new();
        while let Some(ch) = self.current() {
            if ch.is_digit(10) {
                value.push(ch);
                self.advance();
            } else if ch == '.' && !value.contains('.') {
                if let Some(next) = self.peek() {
                    if next.is_digit(10) {
                        value.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(value)
    }
    fn read_ident(&mut self) -> Result<String, LexerError> {
        let mut ident = String::new();
        if let Some(ch) = self.current() {
            if ch.is_alphabetic() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                return Err(LexerError::FirstCharIdentNotAccept);
            }
        }
        while let Some(ch) = self.current() {
            if ch.is_alphanumeric() {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        Ok(ident)
    }
    fn read_string(&mut self) -> String {
        let mut value = String::new();
        self.advance();
        while let Some(ch) = self.current() {
            if ch == '"' {
                self.advance();
                break;
            } else {
                value.push(ch);
                self.advance();
            }
        }
        value
    }
    fn current(&self) -> Option<char> {
        self.chars.get(self.cursor).copied()
    }
}
