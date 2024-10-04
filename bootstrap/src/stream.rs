use std::fmt::Display;

pub trait PeekFor<T, U> {
    fn peek_for<S: Into<String> + Display>(&mut self, kind: T, error: S) -> U;
}

pub trait EndMarked {
    const END: Self;
}

#[derive(Debug)]
pub struct Stream<T> {
    items: Vec<T>,
}

impl<T: Clone + EndMarked + PartialEq> Stream<T> {
    pub fn peek(&self) -> T {
        match self.items.last() {
            None => T::END,
            Some(item) => item.clone(),
        }
    }

    pub fn pop(&mut self) -> T {
        let peeked = self.peek().clone();

        if peeked == T::END {
            T::END
        } else {
            self.items.pop();
            peeked
        }
    }
}

impl<T: Clone, const N: usize> From<[T; N]> for Stream<T> {
    fn from(items: [T; N]) -> Self {
        let mut items = Vec::from(items);
        items.reverse();

        Self { items: items }
    }
}

impl<T> FromIterator<T> for Stream<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut items: Vec<_> = iter.into_iter().collect();
        items.reverse();
        Self { items: items }
    }
}
