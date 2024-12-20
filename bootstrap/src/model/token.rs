use crate::stream::EndMarked;

#[derive(Debug, PartialEq, Clone)]
pub enum KeywordToken {
    Fn,
    Type,
    Struct,
    Enum,
    Static,
    Const,
    Let,
    Continue,
    Break,
    True,
    False,
    Case,
    If,
    Is,
    For,
    Return,
    Mut,
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    Bool,
    Char,
    Str,
    Use,
    As,
}

impl std::fmt::Display for KeywordToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string().to_lowercase())
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
            "let" => Ok(Let),
            "continue" => Ok(Continue),
            "break" => Ok(Break),
            "True" => Ok(True),
            "False" => Ok(False),
            "Unit" => Ok(Unit),
            "when" => Ok(When),
            "for" => Ok(For),
            "return" => Ok(Return),
            "mut" => Ok(Mut),
            "u8" => Ok(U8),
            "i8" => Ok(I8),
            "u16" => Ok(U16),
            "i16" => Ok(I16),
            "u32" => Ok(U32),
            "i32" => Ok(I32),
            "bool" => Ok(Bool),
            "char" => Ok(Char),
            "str" => Ok(Str),
            "use" => Ok(Use),
            "as" => Ok(As),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum BasicToken {
    LBrace,
    RBrace,

    LParen,
    RParen,

    LBrack,
    RBrack,

    LAngle,
    LAngle2,
    RAngle,
    RAngle2,

    Ampersand,
    Ampersand2,

    Bar,
    Bar2,

    Equal,
    PlusEqual,
    AsteriskEqual,
    HyphenEqual,
    FSlashEqual,
    AmpersandEqual,
    BarEqual,
    LAngle2Equal,
    RAngle2Equal,

    Equal2,
    BangEqual,
    RAngleEqual,
    LAngleEqual,

    Plus,
    Asterisk,

    Period,
    Comma,
    Caret,
    Question,

    LightRArrow,
    Colon,
    Colon2,
    Hyphen,
    Bang,
    FSlash,
    Semicolon,
}

impl std::fmt::Display for BasicToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BasicToken::*;

        match self {
            LBrace => write!(f, "{{"),
            RBrace => write!(f, "}}"),

            LParen => write!(f, "("),
            RParen => write!(f, ")"),

            LBrack => write!(f, "["),
            RBrack => write!(f, "]"),

            LAngle => write!(f, "<"),
            LAngle2 => write!(f, "<<"),
            RAngle => write!(f, ">"),
            RAngle2 => write!(f, ">>"),

            Ampersand => write!(f, "&"),
            Ampersand2 => write!(f, "&&"),

            Bar => write!(f, "|"),
            Bar2 => write!(f, "||"),

            Equal => write!(f, "="),
            PlusEqual => write!(f, "+="),
            AsteriskEqual => write!(f, "*="),
            HyphenEqual => write!(f, "-="),
            FSlashEqual => write!(f, "/="),
            AmpersandEqual => write!(f, "&="),
            BarEqual => write!(f, "|="),
            LAngle2Equal => write!(f, "<<="),
            RAngle2Equal => write!(f, ">>="),

            Equal2 => write!(f, "=="),
            BangEqual => write!(f, "!="),
            RAngleEqual => write!(f, ">="),
            LAngleEqual => write!(f, "<="),

            Period => write!(f, "."),
            Caret => write!(f, "^"),
            Question => write!(f, "?"),

            Comma => write!(f, ","),
            LightRArrow => write!(f, "->"),
            Colon => write!(f, ":"),
            Colon2 => write!(f, "::"),
            Asterisk => write!(f, "*"),
            Hyphen => write!(f, "-"),
            Bang => write!(f, "!"),
            Plus => write!(f, "+"),
            FSlash => write!(f, "/"),
            Semicolon => write!(f, ";"),
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

pub struct IdentifierToken;
pub struct IntegerLiteralToken;

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
