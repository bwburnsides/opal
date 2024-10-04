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
    Let,
    Continue,
    Break,
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
            Let => write!(f, "let"),
            Continue => write!(f, "continue"),
            Break => write!(f, "break"),
        }
    }
}

impl TryFrom<String> for OpalKeyword {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        use OpalKeyword::*;
        use Token::*;

        match value.as_str() {
            "fn" => Ok(Fn),
            "type" => Ok(Type),
            "struct" => Ok(Struct),
            "enum" => Ok(Enum),
            "static" => Ok(Static),
            "const" => Ok(Const),
            "if" => Ok(If),
            _ => Err(()),
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

#[derive(Debug, PartialEq, Clone)]
pub enum OpalLiteral {
    Integer(u32),
    String(String),
    Character(char),
}

impl std::fmt::Display for OpalLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use OpalLiteral::*;

        match self {
            Integer(lit) => write!(f, "{}", lit),
            String(st) => write!(f, "\"{}\"", st),
            Character(ch) => write!(f, "'{}'", ch),
        }
    }
}

pub struct OpalIdentifier;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(OpalKeyword),
    Identifier(String),
    Basic(OpalBasic),
    Literal(OpalLiteral),
    Eof,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Token::*;

        match self {
            Keyword(kw) => write!(f, "keyword `{}`", kw),
            Identifier(name) => write!(f, "identifier \"{}\"", name),
            Basic(basic) => write!(f, "token \"{}\"", basic),
            Literal(lit) => write!(f, "literal \'{}\'", lit),
            Eof => write!(f, "end of file"),
        }
    }
}

impl EndMarked for Token {
    const END: Self = Token::Eof;
}
