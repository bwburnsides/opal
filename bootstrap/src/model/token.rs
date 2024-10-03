use crate::stream::EndMarked;

#[derive(Debug, PartialEq, Clone)]
pub enum OpalKeyword {
    Fn,
    Type,
    Struct,
    Enum,
    Static,
    Const,
    If,
}

impl std::fmt::Display for OpalKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use OpalKeyword::*;

        match self {
            Fn => write!(f, "fn"),
            Type => write!(f, "type"),
            Struct => write!(f, "struct"),
            Enum => write!(f, "enum"),
            Static => write!(f, "static"),
            Const => write!(f, "const"),
            If => write!(f, "if"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OpalBasic {
    LBrace,
    RBrace,
    Comma,
    LParen,
    RParen,
    LightRArrow,
}

impl std::fmt::Display for OpalBasic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use OpalBasic::*;

        match self {
            LBrace => write!(f, "{{"),
            RBrace => write!(f, "}}"),
            Comma => write!(f, ","),
            LParen => write!(f, "("),
            RParen => write!(f, ")"),
            LightRArrow => write!(f, "->"),
        }
    }
}

pub struct OpalIdentifier;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(OpalKeyword),
    Identifier(String),
    Basic(OpalBasic),
    Eof,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Token::*;

        match self {
            Keyword(kw) => write!(f, "keyword `{}`", kw),
            Identifier(name) => write!(f, "identifier \"{}\"", name),
            Basic(basic) => write!(f, "token \"{}\"", basic),
            Eof => write!(f, "end of file"),
        }
    }
}

impl EndMarked for Token {
    const END: Self = Token::Eof;
}
