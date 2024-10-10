use crate::span::Span;

#[derive(Debug)]
pub struct Error {
    span: Span,
    message: String,
    details: Option<String>,
}

impl Error {
    pub fn new(span: Span, message: String) -> Self {
        Self {
            span,
            message,
            details: None,
        }
    }

    pub fn with_details(span: Span, message: String, details: String) -> Self {
        Self {
            span,
            message,
            details: Some(details),
        }
    }
}
