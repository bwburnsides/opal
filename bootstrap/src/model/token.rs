use either::Either::Left;

use crate::stream::EndMarked;

#[derive(Debug, PartialEq, Clone)]
pub enum KeywordToken {
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
    True,
    False,
    When,
    For,
    While,
    Return,
}

impl std::fmt::Display for KeywordToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use KeywordToken::*;

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
            True => write!(f, "True"),
            False => write!(f, "False"),
            When => write!(f, "when"),
            For => write!(f, "for"),
            While => write!(f, "while"),
            Return => write!(f, "return"),
        }
    }
}

impl TryFrom<String> for KeywordToken {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        use KeywordToken::*;

        match value.as_str() {
            "fn" => Ok(Fn),
            "type" => Ok(Type),
            "struct" => Ok(Struct),
            "enum" => Ok(Enum),
            "static" => Ok(Static),
            "const" => Ok(Const),
            "if" => Ok(If),
            "let" => Ok(Let),
            "continue" => Ok(Continue),
            "break" => Ok(Break),
            "True" => Ok(True),
            "False" => Ok(False),
            "when" => Ok(When),
            "for" => Ok(For),
            "while" => Ok(While),
            "return" => Ok(Return),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum BasicToken {
    LBrace,
    RBrace,
    Comma,
    LParen,
    RParen,
    LightRArrow,
    Colon2,
    Ampersand,
    Ampersand2,
    Asterisk,
    Hyphen,
    Exclamation,
    LBrack,
    RBrack,
    Equal,
    Plus,
    PlusEqual,
    Bar,
    Bar2,
    Equal2,
}

impl std::fmt::Display for BasicToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BasicToken::*;

        match self {
            LBrace => write!(f, "{{"),
            RBrace => write!(f, "}}"),
            Comma => write!(f, ","),
            LParen => write!(f, "("),
            RParen => write!(f, ")"),
            LightRArrow => write!(f, "->"),
            Colon2 => write!(f, "::"),
            Ampersand => write!(f, "&"),
            Ampersand2 => write!(f, "&&"),
            Asterisk => write!(f, "*"),
            Hyphen => write!(f, "-"),
            Exclamation => write!(f, "!"),
            LBrack => write!(f, "["),
            RBrack => write!(f, "]"),
            Equal => write!(f, "="),
            Equal2 => write!(f, "=="),
            Plus => write!(f, "+"),
            PlusEqual => write!(f, "+="),
            Bar2 => write!(f, "||"),
            Bar => write!(f, "|"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralToken {
    Integer(u32),
    String(String),
    Character(char),
}

impl std::fmt::Display for LiteralToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use LiteralToken::*;

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
    Keyword(KeywordToken),
    Identifier(String),
    Basic(BasicToken),
    Literal(LiteralToken),
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
