use crate::{
    Keyword, Literal,
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
    UnterminatedString(usize),
    InvalidEscape(char, usize),
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
                        "_" => TokenKind::Underscore,
                        "fn" => TokenKind::Keyword(Keyword::Fn),
                        "let" => TokenKind::Keyword(Keyword::Let),
                        "const" => TokenKind::Keyword(Keyword::Const),
                        "static" => TokenKind::Keyword(Keyword::Static),
                        "struct" => TokenKind::Keyword(Keyword::Struct),
                        "enum" => TokenKind::Keyword(Keyword::Enum),
                        "trait" => TokenKind::Keyword(Keyword::Trait),
                        "impl" => TokenKind::Keyword(Keyword::Impl),
                        "type" => TokenKind::Keyword(Keyword::Type),
                        "mod" => TokenKind::Keyword(Keyword::Mod),
                        "use" => TokenKind::Keyword(Keyword::Use),
                        "crate" => TokenKind::Keyword(Keyword::Crate),
                        "extern" => TokenKind::Keyword(Keyword::Extern),
                        "if" => TokenKind::Keyword(Keyword::If),
                        "else" => TokenKind::Keyword(Keyword::Else),
                        "match" => TokenKind::Keyword(Keyword::Match),
                        "loop" => TokenKind::Keyword(Keyword::Loop),
                        "while" => TokenKind::Keyword(Keyword::While),
                        "for" => TokenKind::Keyword(Keyword::For),
                        "in" => TokenKind::Keyword(Keyword::In),
                        "break" => TokenKind::Keyword(Keyword::Break),
                        "continue" => TokenKind::Keyword(Keyword::Continue),
                        "return" => TokenKind::Keyword(Keyword::Return),
                        "pub" => TokenKind::Keyword(Keyword::Pub),
                        "mut" => TokenKind::Keyword(Keyword::Mut),
                        "ref" => TokenKind::Keyword(Keyword::Ref),
                        "move" => TokenKind::Keyword(Keyword::Move),
                        "unsafe" => TokenKind::Keyword(Keyword::Unsafe),
                        "where" => TokenKind::Keyword(Keyword::Where),
                        "dyn" => TokenKind::Keyword(Keyword::Dyn),
                        "async" => TokenKind::Keyword(Keyword::Async),
                        "await" => TokenKind::Keyword(Keyword::Await),
                        "super" => TokenKind::Keyword(Keyword::Super),
                        "as" => TokenKind::Keyword(Keyword::As),
                        "self" => TokenKind::Keyword(Keyword::SelfValue),
                        "Self" => TokenKind::Keyword(Keyword::SelfType),
                        "true" => TokenKind::Keyword(Keyword::True),
                        "false" => TokenKind::Keyword(Keyword::False),
                        _ => TokenKind::Identifier(ident),
                    };
                    token
                }
                c if c.is_numeric() => {
                    let _value = self.number()?;
                    TokenKind::Literal(Literal::Int {
                        value: 0,
                        suffix: None,
                    })
                }
                c if c == '"' => {
                    let value = self.read_string()?;
                    TokenKind::Literal(Literal::StringLit(value))
                }
                '-' => {
                    self.advance();
                    if self.eat('=') {
                        TokenKind::MinusAssign
                    } else if self.eat('>') {
                        TokenKind::Arrow
                    } else {
                        TokenKind::Minus
                    }
                }
                ';' => {
                    self.advance();
                    TokenKind::Semicolon
                }

                ',' => {
                    self.advance();
                    TokenKind::Comma
                }
                ':' => {
                    self.advance();
                    if self.eat(':') {
                        TokenKind::DoubleColon
                    } else {
                        TokenKind::Colon
                    }
                }
                '(' => {
                    self.advance();
                    TokenKind::LParen
                }
                ')' => {
                    self.advance();
                    TokenKind::RParen
                }
                '{' => {
                    self.advance();
                    TokenKind::LBrace
                }
                '}' => {
                    self.advance();
                    TokenKind::RBrace
                }

                '[' => {
                    self.advance();
                    TokenKind::LBracket
                }
                ']' => {
                    self.advance();
                    TokenKind::RBracket
                }

                '+' => {
                    self.advance();
                    if self.eat('=') {
                        TokenKind::PlusAssign
                    } else {
                        TokenKind::Plus
                    }
                }

                '.' => {
                    self.advance();
                    if self.eat('.') {
                        if self.eat('=') {
                            TokenKind::DotDotEq
                        } else {
                            TokenKind::DotDot
                        }
                    } else {
                        TokenKind::Dot
                    }
                }
                '*' => {
                    self.advance();
                    if self.eat('=') {
                        TokenKind::MultiplyAssign
                    } else {
                        TokenKind::Multiply
                    }
                }
                '/' => {
                    self.advance();
                    if self.eat('=') {
                        TokenKind::DivideAssign
                    } else {
                        TokenKind::Divide
                    }
                }
                '=' => {
                    self.advance();
                    if self.eat('=') {
                        TokenKind::EqualEqual
                    } else if self.eat('>') {
                        TokenKind::FatArrow
                    } else {
                        TokenKind::Assign
                    }
                }
                '<' => {
                    self.advance();
                    if self.eat('=') {
                        TokenKind::LessEqual
                    } else {
                        TokenKind::Less
                    }
                }
                '>' => {
                    self.advance();
                    if self.eat('=') {
                        TokenKind::GreaterEqual
                    } else {
                        TokenKind::Greater
                    }
                }
                '!' => {
                    self.advance();
                    if self.eat('=') {
                        TokenKind::NotEqual
                    } else {
                        TokenKind::Not
                    }
                }
                '&' => {
                    self.advance();
                    if self.eat('&') {
                        TokenKind::And
                    } else {
                        TokenKind::Ampersand
                    }
                }
                '^' => {
                    self.advance();
                    if self.eat('=') {
                        TokenKind::XorAssign
                    } else {
                        TokenKind::Caret
                    }
                }
                '_' => {
                    self.advance();
                    TokenKind::Underscore
                }
                '|' => {
                    self.advance();
                    if self.eat('|') {
                        TokenKind::OrOr
                    } else {
                        TokenKind::Pipe
                    }
                }
                '#' => {
                    self.advance();
                    TokenKind::Hash
                }
                '\'' => {
                    self.advance();
                    let ident = self.read_ident()?;
                    TokenKind::Lifetime(ident)
                }
                '%' => {
                    self.advance();
                    if self.eat('=') {
                        TokenKind::PercentAssign
                    } else {
                        TokenKind::Percent
                    }
                }
                _ => return Err(LexerError::InvalidChar(char)),
            };
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
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        Ok(ident)
    }
    fn read_string(&mut self) -> Result<String, LexerError> {
        let start = self.cursor;
        self.advance(); // aspa de abertura
        let mut value = String::new();
        loop {
            match self.current() {
                None => return Err(LexerError::UnterminatedString(start)),
                Some('"') => {
                    self.advance();
                    return Ok(value);
                }
                Some('\\') => {
                    self.advance();
                    match self.current() {
                        Some('n') => value.push('\n'),
                        Some('t') => value.push('\t'),
                        Some('r') => value.push('\r'),
                        Some('0') => value.push('\0'),
                        Some('\\') => value.push('\\'),
                        Some('"') => value.push('"'),
                        Some(c) => return Err(LexerError::InvalidEscape(c, self.cursor)),
                        None => return Err(LexerError::UnterminatedString(start)),
                    }
                    self.advance();
                }
                Some(c) => {
                    value.push(c);
                    self.advance();
                }
            }
        }
    }
    fn eat(&mut self, expected: char) -> bool {
        if self.current() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }
    fn current(&self) -> Option<char> {
        self.chars.get(self.cursor).copied()
    }
}
