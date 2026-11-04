#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    Identifier(String),
    Keyword(Keyword),
    Number(String),
    String(String),

    Semicolon, //';'
    Dot,       //'.'
    Comma,     //','
    Colon,     //':'

    LParen,   //'('
    RParen,   //')'
    LBrace,   //'{'
    RBrace,   //'}'
    LBracket, //'['
    RBracket, //']'

    Plus,     //'+'
    Minus,    //'-'
    Multiply, //'*'
    Divide,   //'/'
    Assign,   //'='
    Less,     //'<'
    Greater,  //'>'

    PlusAssign,     //"+="
    MinusAssign,    //"-="
    MultiplyAssign, //"*="
    DivideAssign,   //"/="
    RestAssign,     //"%="
    AndAssign,      //"&="
    OrAssign,       //"|="
    XorAssign,      //"^="
    LShiftAssign,   //"<<="
    RShiftAssign,   //">>="

    EqualEqual,   //"=="
    NotEqual,     //"!="
    LessEqual,    //"<="
    GreaterEqual, //">="
    And,          //"&&"
    Or,           //"|"
    Not,          //"!"
    Modulus,      //"%"
    LShift,       //"<<"
    RShift,       //">>"
    DoubleColon,  // "::"
    DotDot,       //'..'

    Ampersand, //'&'

    EOF,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Keyword {
    Fn,
    Let,
}
