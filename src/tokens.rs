pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

pub struct Span {
    pub start: u32,
    pub end: u32,
    pub file: String,
}

pub enum TokenKind {
    Identifier(String),
    Number(String),

    Plus,
    Minus,
    Star,
    Slash,

    LeftParen,
    RightParen,

    Let,
    Fn,

    EOF,
}
