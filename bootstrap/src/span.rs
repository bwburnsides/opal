#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: usize,
    pub stop: usize,
}

impl Span {
    pub fn increment(&self) -> Self {
        Self {
            start: self.stop + 1,
            stop: self.start + 2,
        }
    }

    pub fn between(start: Self, stop: Self) -> Self {
        Self {
            start: start.start,
            stop: stop.stop,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub item: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(item: T, span: Span) -> Self {
        Self { item, span }
    }
}

impl<T> From<(usize, T)> for Spanned<T> {
    fn from(value: (usize, T)) -> Spanned<T> {
        Spanned {
            item: value.1,
            span: Span {
                start: value.0,
                stop: value.0 + 1,
            },
        }
    }
}
