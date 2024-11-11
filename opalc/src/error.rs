pub struct ParseError {
    span: (usize, usize),
    kind: ParseErrorKind,
}

pub enum ParseErrorKind {}
