use crate::span::Spanned;
use super::{Expression, ExpressionKind, Type};

pub type TypedExpression = Spanned<TypedExpressionKind>;

pub struct TypedExpressionKind {
    ty: Type,
    expression: ExpressionKind
}

