use std::fmt::Display;

use crate::span::*;

pub trait PeekFor<T, U> {
    fn peek_for<S: Into<String> + Display>(&mut self, kind: T, error: S) -> U;
}

pub trait EndMarked: Clone + PartialEq {
    const END: Self;
}

#[derive(Debug, Clone)]
pub struct Stream<T> {
    spans: Vec<Spanned<T>>,
    last_span: Span,
}

impl<T: EndMarked> Stream<T> {
    pub fn peek(&self) -> T {
        self.spanned_peek().item
    }

    pub fn peek_span(&self) -> Span {
        self.spanned_peek().span
    }

    pub fn pop(&mut self) -> Spanned<T> {
        let peeked = self.spanned_peek();

        if peeked.item == T::END {
            self.end_spanned()
        } else {
            self.spans.pop();
            peeked
        }
    }

    pub fn end_span(&self) -> Span {
        self.end_spanned().span
    }

    fn end_spanned(&self) -> Spanned<T> {
        Spanned::new(T::END, Span::increment(&self.last_span))
    }

    fn spanned_peek(&self) -> Spanned<T> {
        match self.spans.last() {
            None => self.end_spanned(),
            Some(spanned) => spanned.clone(),
        }
    }
}

impl<'a> From<&'a str> for Stream<char> {
    fn from(text: &'a str) -> Stream<char> {
        text.chars().enumerate().map(Spanned::from).collect()
    }
}

impl<T> FromIterator<Spanned<T>> for Stream<T> {
    fn from_iter<I: IntoIterator<Item = Spanned<T>>>(iter: I) -> Self {
        let mut spans: Vec<_> = iter.into_iter().collect();
        let last_span = spans.last().unwrap().span;
        spans.reverse();

        Self { spans, last_span }
    }
}
