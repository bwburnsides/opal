use crate::span::{Span, Spanned};
use either::Either;

pub type Identifier = Spanned<String>;

pub type Jewel = Vec<Item>;

pub enum Item {
    Function(FunctionItem),
    TypeAlias(TypeAliasItem),
    Struct(StructItem),
    Enum(EnumItem),
    Const(ConstItem),
    Static(StaticItem),
}

#[derive(Debug)]
pub struct FunctionItem {
    name: Identifier,
    parameters: Vec<Parameter>,
    return_type: TypeRepr,
    body: BlockExpression,
    span: Span,
}

#[derive(Debug)]
pub struct Parameter {
    is_mutable: bool,
    name: Identifier,
    ty: TypeRepr,
    span: Span,
}

#[derive(Debug)]
pub struct TypeAliasItem {
    name: Identifier,
    ty: TypeRepr,
    span: Span,
}

#[derive(Debug)]
pub struct StructItem {
    name: Identifier,
    fields: Vec<Field>,
    span: Span,
}

#[derive(Debug)]
pub struct Field {
    name: Identifier,
    ty: TypeRepr,
    span: Span,
}

#[derive(Debug)]
pub struct EnumItem {
    name: Identifier,
    variants: Vec<Identifier>,
    span: Span,
}

#[derive(Debug)]
pub struct ConstItem {
    name: Identifier,
    ty: TypeRepr,
    value: Expression,
    span: Span,
}

#[derive(Debug)]
pub struct StaticItem {
    name: Identifier,
    ty: TypeRepr,
    value: Expression,
    span: Span,
}

#[derive(Debug)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub span: Span,
}

impl Expression {
    pub fn new(kind: ExpressionKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug)]
pub enum ExpressionKind {
    WithoutBlock(ExpressionWithoutBlock),
    WithBlock(ExpressionWithBlock),
}

#[derive(Debug)]
pub enum ExpressionWithoutBlock {
    Character(char),
    String(String),
    Integer(u32),
    True,
    False,

    Path {
        is_global: bool,
        segments: Vec<Identifier>,
    },
    Borrow {
        is_mutable: bool,
        expr: Box<Expression>,
    },
    Dereference(Box<Expression>),
    ErrorPropagation(Box<Expression>),
    Negation(NegateOperator, Box<Expression>),
    ArithmeticOrLogical(
        Box<Expression>,
        ArithmeticOrLogicalOperator,
        Box<Expression>,
    ),
    Comparison(Box<Expression>, ComparisonOperator, Box<Expression>),
    LazyBoolean(Box<Expression>, LazyBooleanOperator, Box<Expression>),
    Assignment(Box<Expression>, Box<Expression>),
    CompoundAssignment(Box<Expression>, CompoundAssignmentOperator, Box<Expression>),

    Grouped(Box<Expression>),
    Array(Vec<Expression>),
    Index(Box<Expression>, Box<Expression>),
    Call(Box<Expression>, Vec<Expression>),
    Field(Box<Expression>, Identifier),
    Return(Option<Box<Expression>>),
}

#[derive(Debug)]
pub enum ExpressionWithBlock {
    Block(BlockExpression),
    If(IfExpression),
    When {
        subject: Box<Expression>,
        arms: Vec<WhenArm>,
    },
    For(Identifier, Box<Expression>, BlockExpression),
    While(Box<Expression>, BlockExpression),
}

pub type BlockExpression = Vec<Statement>;

#[derive(Debug)]
pub struct IfExpression {
    predicate: Box<Expression>,
    then_block: Spanned<BlockExpression>,
    else_block: Option<Spanned<Either<BlockExpression, Box<IfExpression>>>>,
}

#[derive(Debug)]
pub struct Statement {
    kind: StatementKind,
    span: Span,
}

#[derive(Debug)]
pub enum StatementKind {
    Let {
        name: Identifier,
        ty: TypeRepr,
        initializer: Expression,
    },
    ExpressionWithBlock(ExpressionWithBlock),
    ExpressionWithoutBlock(ExpressionWithoutBlock),
    Continue,
    Break,
}

#[derive(Debug)]
pub struct TypeRepr {
    kind: TypeReprKind,
    span: Span,
}

#[derive(Debug)]
pub enum TypeReprKind {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    Bool,
    Char,
    Str,
    Unit,
    Array(Box<TypeRepr>, usize),
    Reference(bool, Box<TypeRepr>),
    Parenthesized(Box<TypeRepr>),
    Path(bool, Identifier),
}

#[derive(Debug)]
pub struct WhenArm {
    case: TypeRepr,
    guard: Option<Box<Expression>>,
    block: Spanned<ExpressionWithBlock>,
    span: Span,
}

#[derive(Debug)]
pub enum NegateOperator {
    Arithmetic,
    Logical,
}

#[derive(Debug)]
pub enum ArithmeticOrLogicalOperator {
    Plus,
    Minus,
    Times,
    Divide,
    And,
    Or,
    LShift,
    RShift,
}

#[derive(Debug)]
pub enum ComparisonOperator {
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
}

#[derive(Debug)]
pub enum LazyBooleanOperator {
    Or,
    And,
}

#[derive(Debug)]
pub enum CompoundAssignmentOperator {
    Plus,
    Minus,
    Times,
    Divide,
    And,
    Or,
    LShift,
    RShift,
}
