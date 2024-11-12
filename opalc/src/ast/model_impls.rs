use crate::ast::*;
use crate::lexer::Token;
use crate::span::*;

impl<T> TypeAlias<T> {
    pub fn new(span: Span, name: Spanned<String>, annotation: Ty, ty: T) -> Self {
        Self {
            span,
            name,
            annotation,
            ty,
        }
    }
}

impl<T, Expr> Function<T, Expr> {
    pub fn new(
        span: Span,
        name: Spanned<String>,
        parameters: Vec<Parameter<T>>,
        body: Vec<Statement<T, Expr>>,
        return_annotation: Option<Ty>,
        return_ty: T,
    ) -> Self {
        Self {
            span,
            name,
            parameters,
            body,
            return_annotation,
            return_ty,
        }
    }
}

impl<T> Parameter<T> {
    pub fn new(
        span: Span,
        name: Spanned<String>,
        annotation: Ty,
        anonymity: Anonymity,
        mutability: Mutability,
        ty: T,
    ) -> Self {
        Self {
            span,
            name,
            annotation,
            anonymity,
            mutability,
            ty,
        }
    }
}

impl<T, Expr> Argument<T, Expr> {
    pub fn new(span: Span, label: Option<Spanned<String>>, value: Expr, ty: T) -> Self {
        Self {
            span,
            label,
            value,
            ty,
        }
    }
}

impl<T> AlgebraicType<T> {
    pub fn new(span: Span, name: Spanned<String>, kind: AlgebraicTypeKind<T>, ty: T) -> Self {
        Self {
            span,
            name,
            kind,
            ty,
        }
    }
}

impl<T> Field<T> {
    pub fn new(span: Span, name: Spanned<String>, annotation: Ty, ty: T) -> Self {
        Self {
            span,
            name,
            annotation,
            ty,
        }
    }
}

impl<Expr> Arm<Expr> {
    pub fn new(pattern: Pattern, expression: Expr) -> Self {
        Self {
            pattern,
            expression,
        }
    }
}

impl<T, Expr> Constant<T, Expr> {
    pub fn new(
        span: Span,
        name: Spanned<String>,
        annotation: Ty,
        initializer: Expr,
        ty: T,
    ) -> Self {
        Self {
            span,
            name,
            annotation,
            initializer,
            ty,
        }
    }
}

impl<T, Expr> Static<T, Expr> {
    pub fn new(
        span: Span,
        name: Spanned<String>,
        annotation: Ty,
        initializer: Expr,
        ty: T,
    ) -> Self {
        Self {
            span,
            name,
            annotation,
            initializer,
            ty,
        }
    }
}

impl<T, Expr> Let<T, Expr> {
    pub fn new(
        span: Span,
        name: Spanned<String>,
        annotation: Option<Ty>,
        initializer: Option<Expr>,
        mutability: Mutability,
        ty: T,
    ) -> Self {
        Self {
            span,
            name,
            annotation,
            initializer,
            mutability,
            ty,
        }
    }
}

impl TryFrom<Token> for UnaryOperator {
    type Error = ();

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        use Token::*;

        match value {
            Plus => Ok(Self::Posit),
            Minus => Ok(Self::Negate),
            Star => Ok(Self::Dereference),
            Amper => Ok(Self::Borrow),
            Bang => Ok(Self::Invert),
            _ => Err(()),
        }
    }
}
