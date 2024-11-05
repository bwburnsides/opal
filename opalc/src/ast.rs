mod expr;
mod untyped;
mod typed;

use untyped::UntypedExpression;
use typed::TypedExpression;

mod ty {
    pub struct Ty;
}

pub struct Span(usize, usize);
pub struct Spanned<T>(T, Span);

pub enum Item<T, Expr> {
    Function(Function<T, Expr>),
    TypeAlias(TypeAlias<T>),
    DataType(AlgebraicType<T>),
    Constant(Constant<T, Expr>),
    Static {
        span: Span,
        name: Spanned<String>,
        annotation: Ty,
        initializer: Option<Expr>,
        mutability: Mutability,
        ty: T,
    }
}

pub struct TypeAlias<T> {
    span: Span,
    name: Spanned<String>,
    annotation: Ty,
    ty: T,
}

pub struct Function<T, Expr> {
    span: Span,
    name: Spanned<String>,
    parameters: Vec<Parameter<T>>,
    body: Vec<Statement<T, Expr>>,
    return_annotation: Option<Ty>,
    return_ty: T,
}

pub struct Parameter<T> {
    span: Span,
    name: Spanned<String>,
    annotation: Ty,
    anonymity: Anonymity,
    ty: T,
}

pub struct Argument<T> {
    span: Span,
    name: Spanned<String>,
    label: Option<Spanned<String>>,
    ty: T,
}

pub enum Ty {
    Name(Spanned<String>),
    Array {
        base: Box<Ty>,
        size: u32,
    },
    Reference {
        base: Box<Ty>,
        mutability: Mutability,
    }
}


pub enum Statement<T, Expr> {
    Expression(Expr),
    Constant(Constant<T, Expr>),
    Let {
        span: Span,
        name: Spanned<String>,
        annotation: Option<Ty>,
        initializer: Option<Expr>,
        mutability: Mutability,
        ty: T,
    },
}

pub type UntypedStatement = Statement<(), UntypedExpression>;
pub type TypedStatement = Statement<ty::Ty, TypedExpression>;

pub struct AlgebraicType<T> {
    span: Span,
    name: Spanned<String>,
    kind: AlgebraicTypeKind<T>,
}

pub enum AlgebraicTypeKind<T> {
    Sum(Vec<Variant<T>>),
    Product(Vec<Field<T>>),
}

pub enum Variant<T> {
    Unit(Spanned<String>),
    Tuple {
        name: Spanned<String>,
        span: Span,
        items: Vec<Ty>,
        typed_items: Vec<T>,
    },
    Record {
        name: Spanned<String>,
        fields: Vec<Field<T>>,
    }
}

pub struct Field<T> {
    span: Span,
    name: Spanned<String>,
    annotation: Ty,
    ty: T,
}

pub struct Constant<T, Expr> {
    span: Span,
    name: Spanned<String>,
    annotation: Ty,
    initializer: Expr,
    ty: T,
}

pub enum Mutability {
    Mutable,
    Immutable,
}

pub enum Anonymity {
    Anonymous,
    Named,
}

struct Clause;
struct BinaryOperator;
struct UnaryOperator;
