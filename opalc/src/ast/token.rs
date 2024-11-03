use crate::span::Symbol;

pub enum BinaryOperatorToken {
    Add,
    Subtract,
    Multiply,
    Divide,
    Caret,
    And,
    Or,
    ShiftLeft,
    ShiftRight,
}

pub enum Delimiter {
    Paren,
    Brace,
    Brack,
}

pub enum LiteralKind {
    Bool,
    Int,
    Char,
    Str,
}

pub struct Literal {
    pub kind: LiteralKind,
    pub symbol: Symbol,
}

pub enum KeywordKind {
    Use,
    As,
    Fn,
    Type,
    Case,
    For,
    Static,
    Const,
    Return,
    Break,
    Continue,
}

pub struct Keyword {
    pub kind: KeywordKind,
    pub symbol: Symbol,
}

pub enum TokenKind {
    Eq,
    Lt,
    Le,
    EqEq,
    Ne,
    Ge,
    Gt,
    AndAnd,
    OrOr,
    Not,
    Tilde,
    BinOp(BinaryOperatorToken),
    BinOpEq(BinaryOperatorToken),
    Dot,
    Comma,
    Semicolon,
    Colon,
    PathSep, // `::`
    RArrow,  // `->`
    Question,
    OpenDelim(Delimiter),
    CloseDelim(Delimiter),
    Literal(Literal),
    Keyword(Keyword),
    Ident(Symbol),
    Eof,
}