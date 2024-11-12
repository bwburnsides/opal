mod expr;
mod model_impls;
mod typed;
mod untyped;

pub use expr::*;
pub use typed::*;
pub use untyped::*;

use crate::span::*;

mod ty {
    pub struct Ty;
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
    },
    Group(Box<Ty>),
    Unit,
    Never,
}

// ast::Ty is the syntactic representation of a type annotation.
// ty::Ty is the reified representation of a data type.

pub type UntypedItem = Item<(), UntypedExpression>;
pub type TypedItem = Item<ty::Ty, TypedExpression>;

pub enum Item<T, Expr> {
    Function(Function<T, Expr>),
    TypeAlias(TypeAlias<T>),
    DataType(AlgebraicType<T>),
    Constant(Constant<T, Expr>),
    Static(Static<T, Expr>),
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

pub type UntypedFunction = Function<(), UntypedExpression>;
pub type TypedFunction = Function<ty::Ty, TypedExpression>;

pub struct Parameter<T> {
    span: Span,
    name: Spanned<String>,
    annotation: Ty,
    anonymity: Anonymity,
    mutability: Mutability,
    ty: T,
}

pub type UntypedParameter = Parameter<()>;
pub type TypedParameter = Parameter<ty::Ty>;

pub struct Argument<T, Expr> {
    span: Span,
    label: Option<Spanned<String>>,
    value: Expr,
    ty: T,
}

pub type UntypedArgument = Argument<(), UntypedExpression>;
pub type TypedArgument = Argument<ty::Ty, TypedExpression>;

pub enum Statement<T, Expr> {
    Expression(Expr),
    Constant(Constant<T, Expr>),
    Let(Let<T, Expr>),
}

pub struct Let<T, Expr> {
    span: Span,
    name: Spanned<String>,
    annotation: Option<Ty>,
    initializer: Option<Expr>,
    mutability: Mutability,
    ty: T,
}

pub type UntypedStatement = Statement<(), UntypedExpression>;
pub type TypedStatement = Statement<ty::Ty, TypedExpression>;

pub struct AlgebraicType<T> {
    span: Span,
    name: Spanned<String>,
    kind: AlgebraicTypeKind<T>,
    ty: T,
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
    },
}

pub type UntypedVariant = Variant<()>;
pub type TypedVariant = Variant<ty::Ty>;

pub struct Field<T> {
    span: Span,
    name: Spanned<String>,
    annotation: Ty,
    ty: T,
}

pub type UntypedField = Field<()>;
pub type TypedField = Field<ty::Ty>;

pub struct Arm<Expr> {
    pattern: Pattern,
    expression: Expr,
    // TODO: Might want a ty: T here but then I'll need to uphold
    // ty == expression.type
}

pub type UntypedArm = Arm<UntypedExpression>;
pub type TypedArm = Arm<TypedExpression>;

// TODO: Does Pattern need a type field like expressions?
// Not exactly sure how patterns work yet.
pub enum Pattern {
    Bool(bool),
    Char(char),
    String(String),
    Integer(u32),
    Name(String),
    Wildcard,
    Path(Vec<String>),
    Struct(String, Vec<(String, String)>),
    Tuple(Vec<Pattern>),
    Enum(String, Vec<String>),
    Grouped(Box<Pattern>),
}

pub struct Constant<T, Expr> {
    span: Span,
    name: Spanned<String>,
    annotation: Ty,
    initializer: Expr,
    ty: T,
}

pub type UntypedConstant = Constant<(), UntypedExpression>;
pub type TypedConstant = Constant<ty::Ty, TypedExpression>;

pub struct Static<T, Expr> {
    span: Span,
    name: Spanned<String>,
    annotation: Ty,
    initializer: Expr,
    ty: T,
}

pub type UntypedStatic = Static<(), UntypedExpression>;
pub type TypedStatic = Static<ty::Ty, TypedExpression>;

pub enum Mutability {
    Mutable,
    Immutable,
}

pub enum Anonymity {
    Anonymous,
    Named,
}

pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub enum UnaryOperator {
    Posit,
    Negate,
    Dereference,
    Borrow,
    Invert,
}
