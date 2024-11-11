pub struct Span(pub usize, pub usize);
pub struct Spanned<T>(pub T, pub Span);
