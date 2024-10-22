use crate::span::{Spanned, Span};
use super::{Expression, ExpressionKind, Type};


pub struct TypedExpression {
    pub ty: Type,
    pub kind: ExpressionKind,
    pub span: Span,
}
