#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Identifier(String),
    Keyword(Keyword),
    Literal(Literal),

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
    PercentAssign,  //"%="
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
    Percent,      //"%"
    DoubleColon,  // "::"
    DotDot,       //'..'

    Arrow,      // "->"
    FatArrow,   // "=>"
    Question,   // "?"
    Hash,       // "#"
    Underscore, // "_"
    Pipe,       // "|"
    OrOr,       // "||"
    Caret,      // "^"
    DotDotEq,   // "..="
    Char(char),
    Lifetime(String),

    Ampersand, //'&'

    EOF,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Int {
        value: u128,
        suffix: Option<IntSuffix>,
    },
    Float {
        value: f64,
        suffix: Option<FloatSuffix>,
    },
    Char(char),
    Byte(u8),
    StringLit(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum IntSuffix {
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FloatSuffix {
    F32,
    F64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Keyword {
    Fn,
    Let,
    Const,
    Static,
    Struct,
    Enum,
    Trait,
    Impl,
    Type,
    Mod,
    Use,
    Crate,
    Extern,
    If,
    Else,
    Match,
    Loop,
    While,
    For,
    In,
    Break,
    Continue,
    Return,
    Pub,
    Mut,
    Ref,
    Move,
    Unsafe,
    Where,
    Dyn,
    Async,
    Await,
    Super,
    As,
    SelfValue, // self
    SelfType,  // Self
    True,
    False,
}
