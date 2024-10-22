use crate::span::{Spanned, Span};

pub type Identifier = Spanned<String>;

#[derive(Debug, Clone)]
pub struct FunctionItem {
    pub name: Identifier
}

type Type = ();
type ExpressionKind = ();

type Expression = Spanned<ExpressionKind>;

impl Expression {
    fn expr(&self) -> &ExpressionKind {
        &self.item
    }

    fn span(&self) -> &Span {
        &self.span
    }
}

struct Typed<T> {
    ty: Type,
    item: T,
}

impl<T> Typed<T> {
    fn new() -> Self {
        todo!()
    }
}

type TypedExpression = Typed<Expression>;

impl TypedExpression {
    fn ty(&self) -> &Type {
        &self.ty
    }

    fn expr(&self) -> &ExpressionKind {
        &self.item.expr()
    }

    fn span(&self) -> &Span {
        &self.item.span()
    }
}