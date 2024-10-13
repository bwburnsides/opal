#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub start: usize,
    pub stop: usize,
}

impl Span {
    pub fn new(start: usize, stop: usize) -> Self {
        Self { start, stop }
    }

    pub fn between(start: Self, stop: Self) -> Self {
        Self {
            start: start.start,
            stop: stop.stop,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Spanned<T> {
    pub item: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(item: T, span: Span) -> Self {
        Self { item, span }
    }

    pub fn empty(item: T) -> Self {
        Self {
            item,
            span: Span::new(0, 0),
        }
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
