#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

impl Token {
    pub fn new(kind: TokenKind, len: usize) -> Token {
        Token { kind, len }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Word,
    Literal(LiteralKind),
    Comment,
    Newline,
    Whitespace,
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    OpenBrack,
    CloseBrack,
    LessThan,
    GreatThan,
    Ampersand,
    Bar,
    Equal,
    Plus,
    Asterisk,
    Period,
    Comma,
    Caret,
    Question,
    Colon,
    Hyphen,
    Bang,
    Slash,
    Semicolon,
    Eof,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiteralKind {
    Int(Base),
    Char,
    Str,

    InvalidCharWithoutContent,
    InvalidCharNewlineBeforeTermination,
    InvalidCharIllegalEscapeSequence,
    InvalidCharUnexpectedCharacterAtTermination,
    InvalidEOFBeforeTermination,
    InvalidIntWithLeadingZero,
    InvalidBinIntWithoutDigits,
    InvalidHexIntWithoutDigits,
    InvalidStrNewlineBeforeTermination,
    InvalidStrIllegalEscapeSequence,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Base {
    Bin = 2,
    Dec = 10,
    Hex = 16,
}

enum DecimalDigit {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

impl TryFrom<char> for DecimalDigit {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(Self::Zero),
            '1' => Ok(Self::One),
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            _ => Err(()),
        }
    }
}