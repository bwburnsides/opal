#[derive(PartialEq, Debug)]
pub enum IntegerBase {
    Bin,
    Dec,
    Hex,
}

#[derive(PartialEq, Debug)]
pub enum Token {
    Name(String),
    IntLit {value: usize, base: IntegerBase},
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

    // TODO: Add other assignment operators.

    Use,
    As,
    
    Fn,
    Const,
    Let,
    Mut,
    Anon,
    Type,
    
    Case,
    For,
    Break,
    Continue,
    Return,

    NewLine,
    EndOfFile,
    Unknown,
}
