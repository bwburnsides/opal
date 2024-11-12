pub struct Span(pub usize, pub usize);

pub struct Spanned<T>(pub T, pub Span);

impl<T> Spanned<T> {
    pub fn from_pair(item: T, pair: (usize, usize)) -> Self {
        Self(item, Span(pair.0, pair.1))
    }
}
