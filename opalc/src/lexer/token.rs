#[derive(PartialEq, Debug)]
pub enum IntegerBase {
    Bin,
    Dec,
    Hex,
}

#[derive(PartialEq, Debug)]
pub enum Token {
    Name(String),
    IntLit(u32, IntegerBase),
    StringLit(String),
    CharLit(char),

    LeftParen,
    RightParen,
    LeftSquare,
    RightSquare,
    LeftBrace,
    RightBrace,

    Plus,
    Minus,
    Star,
    Slash,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,

    Colon,
    Comma,
    Bang,
    Question,
    Equal,
    EqualEqual,
    NotEqual,
    Bar,
    BarBar,
    Amper,
    AmperAmper,
    LtLt,
    GtGt,
    Dot,
    RArrow,

    U8,
    U16,
    U32,
    I8,
    I16,
    I32,
    Char,
    Str,
    Bool,
    True,
    False,

    // TODO: Add other assignment operators.
    Use,
    As,

    Fn,
    Const,
    Let,
    Mut,
    Anon,
    Type,
    Static,

    Case,
    For,
    In,
    Break,
    Continue,
    Return,

    NewLine,
    EndOfFile,
    Unknown,
}
