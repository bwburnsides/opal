pub mod cursor;
pub mod token;

pub use cursor::*;
pub use token::IntegerBase::*;
pub use token::*;

use token::Token::*;

#[derive(PartialEq, Debug)]
pub struct Spanned<T> {
    item: T,
    start: usize,
    stop: usize,
}

impl<T> Spanned<T> {
    pub fn new(item: T, start: usize, stop: usize) -> Self {
        Self { item, start, stop }
    }

    pub fn from_pair(item: T, pair: (usize, usize)) -> Self {
        Self {
            item,
            start: pair.0,
            stop: pair.1,
        }
    }
}

pub fn tokenize(input: &str) -> impl Iterator<Item = Spanned<Token>> + '_ {
    let mut cursor = Cursor::new(input);
    std::iter::from_fn(move || {
        let token = cursor.token();
        if token.item != EndOfFile {
            Some(token)
        } else {
            None
        }
    })
}

impl Cursor<'_> {
    pub fn token(&mut self) -> Spanned<Token> {
        self.start();

        let first = match self.pop() {
            None => return Spanned::from_pair(EndOfFile, self.spans()),
            Some(ch) => ch,
        };

        let token = match first {
            ch @ '0'..='9' => self.integer(ch.try_into().unwrap()),
            ch @ ('a'..='z' | 'A'..='Z' | '_') => self.word(ch),
            '\'' => self.character(),
            '"' => self.string(),
            '#' => {
                self.skip_comment();
                return self.token();
            }
            '(' => LeftParen,
            ')' => RightParen,
            '[' => LeftSquare,
            ']' => RightSquare,
            '{' => LeftBrace,
            '}' => RightBrace,
            '+' => Plus,
            '-' => match self.peek() {
                '>' => {
                    self.pop();
                    RArrow
                }
                _ => Minus,
            },
            '*' => Star,
            '/' => Slash,
            '<' => match self.peek() {
                '<' => {
                    self.pop();
                    LtLt
                }
                '=' => {
                    self.pop();
                    LessEqual
                }
                _ => Less,
            },
            '>' => match self.peek() {
                '>' => {
                    self.pop();
                    GtGt
                }
                '=' => {
                    self.pop();
                    GreaterEqual
                }
                _ => Greater,
            },
            ':' => Colon,
            ',' => Comma,
            '!' => match self.peek() {
                '=' => {
                    self.pop();
                    NotEqual
                }
                _ => Bang,
            },
            '?' => Question,
            '=' => match self.peek() {
                '=' => {
                    self.pop();
                    EqualEqual
                }
                _ => Equal,
            },
            '|' => match self.peek() {
                '|' => {
                    self.pop();
                    BarBar
                }
                _ => Bar,
            },
            '&' => match self.peek() {
                '&' => {
                    self.pop();
                    AmperAmper
                }
                _ => Amper,
            },
            '.' => Dot,
            '\n' => NewLine,
            '\0' => EndOfFile,
            _ => Unknown,
        };

        Spanned::from_pair(token, self.spans())
    }

    fn integer(&mut self, first: Digit) -> Token {
        debug_assert!('0' <= self.prev && self.prev <= '9');

        match first {
            Digit::Zero => match self.peek() {
                'b' => todo!("bin"),
                'x' => todo!("hex"),
                '0'..='9' => todo!("Multi-digit integer literal cannot start with 0"),
                _ => IntLit(0, Dec),
            },
            _ => todo!("dec base"),
        }
    }

    fn word(&mut self, _first: char) -> Token {
        debug_assert!(
            ('a' <= self.prev && self.prev <= 'z')
                || ('A' <= self.prev && self.prev <= 'Z')
                || (self.prev == '_')
        );

        let mut data = String::new();
        loop {
            match self.peek() {
                ch @ ('a'..='z' | 'A'..='Z' | '0'..='9' | '_') => data.push(ch),
                _ => match data.as_str() {
                    "u8" => break U8,
                    "u16" => break U16,
                    "u32" => break U32,
                    "i8" => break I8,
                    "i16" => break I16,
                    "i32" => break I32,
                    "char" => break Char,
                    "str" => break Str,
                    "bool" => break Bool,
                    "true" => break True,
                    "false" => break False,
                    "use" => break Use,
                    "as" => break As,
                    "fn" => break Fn,
                    "const" => break Const,
                    "let" => break Let,
                    "mut" => break Mut,
                    "anon" => break Anon,
                    "type" => break Type,
                    "case" => break Case,
                    "for" => break For,
                    "in" => break In,
                    "break" => break Break,
                    "continue" => break Continue,
                    "return" => break Return,
                    _ => break Name(data),
                },
            }
        }
    }

    fn character(&mut self) -> Token {
        debug_assert!(self.prev == '\'');

        let next = match self.pop() {
            None => todo!("EOF before char data"),
            Some(ch) => ch,
        };

        let data = match next {
            '\'' => todo!("Empty char literal"),
            '\\' => todo!("Escape sequences"),
            // Base case is normal char
            ch => ch,
        };

        let end = match self.pop() {
            None => todo!("EOF before char terminated"),
            Some(ch) => ch,
        };

        match end {
            '\'' => CharLit(data),
            _ => todo!("Expected terminator, got data"),
        }
    }

    fn string(&mut self) -> Token {
        debug_assert!(self.prev == '"');
        todo!()
    }

    fn skip_comment(&mut self) {
        match self.pop() {
            None => {}
            Some('\n') => {}
            Some(_) => {}
        }
    }
}

enum Digit {
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

impl TryFrom<char> for Digit {
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
