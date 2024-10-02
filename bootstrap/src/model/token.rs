#[derive(Clone)]
pub enum Keyword {
    Fn,
    Type,
    Struct,
    Enum,
    Const,
    Static,
    Continue,
    Break,
    True,
    False,
    Return,
    If,
    Else,
    When,
    Is,
    Mut,
    In,
}

#[derive(Clone)]
pub enum Literal {
    Char(char),
    String(String),
    Integer(usize),
    True,
    False,
}

#[derive(Clone)]
pub enum Basic {
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBrack,
    RBrack,

    Hyphen,
    RLightArrow,
    HyphenEqual,

    Equal,
    Equal2,
    RHeavyArrow,

    Newline,
    Comma,
    Semicolon,
    Question,
    Period,

    Exclamation,
    ExclamationEqual,

    Colon,
    Colon2,

    Bar,
    Bar2,
    BarEqual,

    Plus,
    PlusEqual,

    Ampersand,
    Ampersand2,
    AmpersandEqual,

    Asterisk,
    AsteriskEqual,

    FSlash,
    FSlashEqual,

    LAngle,
    LAngle2,
    LAngleEqual,

    LAngle2Equal,

    RAngle,
    RAngle2,
    RAngleEqual,

    RAngle2Equal,
}

#[derive(Clone)]
pub enum Token {
    Keyword(Keyword),
    Literal(Literal),
    Ident(String),
    Basic(Basic),
    Poison,
}
