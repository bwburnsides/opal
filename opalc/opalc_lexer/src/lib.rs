use std::iter::Iterator;

pub mod token;
pub mod cursor;

pub use cursor::*;
pub use token::*;

use TokenKind::*;
use LiteralKind::*;

pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut cursor =  Cursor::new(input);
    std::iter::from_fn(move || {
        let token = cursor.token();
        if token.kind != Eof { Some(token) } else { None }
    })
}

impl Cursor<'_> {
    pub fn token(&mut self) -> Token {
        let first = match self.pop() {
            None => return Token::new(Eof, 0),
            Some(ch) => ch,
        };

        let kind = match first {
            '#' => self.comment(),
            '"' => self.string(),
            '\'' => self.character(),
            '\n' => Newline,
            ';' => Semicolon,
            ',' => Comma,
            '.' => Period,
            '(' => OpenParen,
            ')' => CloseParen,
            '{' => OpenBrace,
            '}' => CloseBrace,
            '[' => OpenBrack,
            ']' => CloseBrack,
            '?' => Question,
            ':' => Colon,
            '=' => Equal,
            '!' => Bang,
            '<' => LessThan,
            '>' => GreatThan,
            '-' => Hyphen,
            '&' => Ampersand,
            '|' => Bar,
            '+' => Plus,
            '*' => Asterisk,
            '^' => Caret,
            ' ' | '\t' | '\r' => Whitespace,
            ch @ '0'..='9' => self.integer(
                DecimalDigit::try_from(ch).expect(
                    "Expected match arm to guard against non-decimal digit characters."
                )
            ),
            'a'..='z' | 'A'..='Z' | '_' => self.word(),
            EOF => Eof,
            _ => Unknown
        };

        Token::new(kind, self.consumed())
    }

    fn word(&mut self) -> TokenKind {
        loop {
            match self.peek() {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => self.pop(),
                _ => {
                    return Word
                }
            };
        }
    }

    fn comment(&mut self) -> TokenKind {
        // See comment in `Cursor::integer`
        debug_assert!(self.prev == '#');

        loop {
            match self.peek() {
                '\n' => {
                    self.pop();
                    break Comment
                },
                _ => {
                    self.pop();
                }
            }
        }
    }

    fn string(&mut self) -> TokenKind {
        // See comment in `Cursor::integer`
        debug_assert!(self.prev == '\"');

        // Unlike character literals, string literals are allowed to be empty.
        loop {
            match self.peek() {
                '\"' => {
                    self.pop();
                    break Literal(Str)
                },
                '\n' => break Literal(InvalidStrNewlineBeforeTermination),
                '\\' => {
                    self.pop();
                    match self.peek() {
                        '0' => self.pop(),
                        '\'' => self.pop(),
                        '"' => self.pop(),
                        'n' => self.pop(),
                        't' => self.pop(),
                        '\\' => self.pop(),
                        _ => break Literal(InvalidStrIllegalEscapeSequence),
                    }
                },
                EOF if self.is_empty() => break Literal(InvalidEOFBeforeTermination),
                _ => self.pop(),
            };
        }
    }

    fn character(&mut self) -> TokenKind {
        // See comment in `Cursor::integer``.
        debug_assert!(self.prev == '\'');

        // Need to handle empty string case,
        // normal non-empty case, and escape cases.

        // TODO: Escapes are actually kind of complicated with respect
        // to the number of characters they're represented with in the
        // source file, the corresponding representation of each of those
        // characters in this file, and the value that must be encoded in
        // the output. Write a bunch of tests...

        match self.peek() {
            '\'' => return Literal(InvalidCharWithoutContent),
            '\n' => return Literal(InvalidCharNewlineBeforeTermination),
            '\\' => {
                self.pop();
                match self.peek() {
                    '0' => self.pop(),
                    '\'' => self.pop(),
                    '"' => self.pop(),
                    'n' => self.pop(),
                    't' => self.pop(),
                    '\\' => self.pop(),
                    _ => return Literal(InvalidCharIllegalEscapeSequence),
                }
            },            
            EOF if self.is_empty() => return Literal(InvalidEOFBeforeTermination),
            _ => self.pop(),
        };

        // Check for closing quote
        match self.peek() {
            '\'' => {
                self.pop();
                Literal(Char)
            },
            '\n' => Literal(InvalidCharNewlineBeforeTermination),
            EOF if self.is_empty() => Literal(InvalidEOFBeforeTermination),
            _ => {
                Literal(InvalidCharUnexpectedCharacterAtTermination)
            }
        }
    }

    fn integer(&mut self, first: DecimalDigit) -> TokenKind {
        // Ensure that the token stream truly contained an integer
        // digit to defend against the case of this being called
        // with an arbitrary character. We don't want to produce
        // a token if the source doesn't actually contain it.
        debug_assert!('0' <= self.prev && self.prev <= '9');

        // Man, wouldn't refinement types be great?
        use DecimalDigit::*;

        match first {
            Zero => match self.peek() {
                'b' => {
                    self.pop();
                    match consume_bin_digits(self) {
                        None => Literal(InvalidBinIntWithoutDigits),
                        Some(kind) => kind,
                    }
                },
                'x' => {
                    self.pop();
                    match consume_hex_digits(self) {
                        None => Literal(InvalidHexIntWithoutDigits),
                        Some(kind) => kind,
                    }
                },
                '1'..='9' | '_' => Literal(InvalidIntWithLeadingZero),
                _ => Literal(Int(Base::Dec)),
            },
            _ => consume_dec_digits(self)
        }
    }
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

fn consume_bin_digits(cursor: &mut Cursor) -> Option<TokenKind> {
    let mut is_empty = true;

    loop {
        match cursor.peek() {
            '_' => {
                // The literal `0b_` is invalid, so don't allow presense
                // of underscore to alter `is_empty` value.
                cursor.pop();
            }
            '0' | '1' => {
                cursor.pop();
                is_empty = false;
            }
            _ => break,
        }
    }

    match is_empty {
        true => None,
        false => Some(Literal(Int(Base::Bin)))
    }
}

fn consume_dec_digits(cursor: &mut Cursor) -> TokenKind {
    // TODO: Think about whether you need an `is_empty` flag like in the
    // hex and bin cases when its not 1:00 AM. At (current) call site,
    // the cursor will always be at 1..=9. But we need to be defensive
    // against future call sites. Think about debug assertions too.
    loop {
        match cursor.peek() {
            '_' => {
                cursor.pop();
            }
            '0'..='9' => {
                cursor.pop();
            }
            _ => break Literal(Int(Base::Dec)),
        }
    }
}

fn consume_hex_digits(cursor: &mut Cursor) -> Option<TokenKind> {
    let mut is_empty = true;

    loop {
        match cursor.peek() {
            '_' => {
                // The literal `0x_` is invalid, so don't allow presense
                // of underscore to alter `is_empty` value.
                cursor.pop();
            }
            '0'..='9' | 'a'..='f' | 'A'..='F' => {
                cursor.pop();
                is_empty = false;
            }
            _ => break,
        }
    }

    match is_empty {
        true => None,
        false => Some(Literal(Int(Base::Hex)))
    }
}