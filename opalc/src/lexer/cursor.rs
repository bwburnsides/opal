use std::str::Chars;

pub const EOF: char = '\0';

pub struct Cursor<'a> {
    consumed: usize,  // Number of characters consumed
    marker: usize,  // An arbitrary position marker set by caller as needed

    chars: Chars<'a>,
    #[cfg(debug_assertions)]
    pub prev: char,
}


impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
            consumed: 0,
            marker: 0,
            chars: input.chars(),
            #[cfg(debug_assertions)]
            prev: EOF,
        }
    }

    pub fn peek(&self) ->  char {
        self.chars.clone().next().unwrap_or(EOF)
    }

    pub fn is_empty(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    #[cfg(debug_assertions)]
    pub fn pop(&mut self) -> Option<char> {
        let ch = self.chars.next()?;
        self.prev = ch;
        self.consumed += 1;
        Some(ch)
    }

    #[cfg(not(debug_assertions))]
    pub fn pop(&mut self) -> Option<char> {
        let ch = self.chars.next()?;
        self.consumed += 1;
        Some(ch)
    }

    pub fn start(&mut self) {
        self.marker = self.consumed;
    }

    pub fn spans(&mut self) -> (usize, usize) {
        (self.marker, self.consumed)
    }
}
