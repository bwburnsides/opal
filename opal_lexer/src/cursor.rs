use std::str::Chars;

pub const EOF: char = '\0';

pub struct Cursor<'a> {
    remaining: usize,
    chars: Chars<'a>,
    #[cfg(debug_assertions)]
    pub prev: char,
}


impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
            remaining: input.len(),
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
        Some(ch)
    }

    #[cfg(not(debug_assertions))]
    pub fn pop(&mut self) -> Option<char> {
        self.chars.next()
    }

    pub fn pop_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.peek()) && !self.is_empty() {
            self.pop();
        }
    }

    pub fn consumed(&mut self) -> usize {
        let rv = self.remaining - self.chars.as_str().len();
        self.remaining = self.chars.as_str().len();
        rv
    }
}
