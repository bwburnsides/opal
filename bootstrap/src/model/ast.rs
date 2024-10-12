use crate::span::{Span, Spanned};
use either::Either;

use super::BasicToken;
use crate::model::*;

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

#[derive(Debug, Clone)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub span: Span,
}

impl Expression {
    pub fn new(kind: ExpressionKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, Clone)]
pub enum ExpressionKind {
    WithoutBlock(ExpressionWithoutBlock),
    WithBlock(ExpressionWithBlock),
}

#[derive(Debug, Clone)]
pub enum ExpressionWithoutBlock {
    Character(char),
    String(String),
    Integer(u32),
    True,
    False,
    Unit,

    Path {
        is_global: bool,
        name: Identifier,
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
    Assignment(Box<Expression>, AssignmentOperator, Box<Expression>),
    Grouped(Box<Expression>),
    Array(Vec<Expression>),
    Index(Box<Expression>, Box<Expression>),
    Call(Box<Expression>, Vec<Expression>),
    Field(Box<Expression>, Identifier),
    Return(Option<Box<Expression>>),
    Break(Option<Box<Expression>>),
    Continue,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct IfExpression {
    predicate: Box<Expression>,
    then_block: Spanned<BlockExpression>,
    else_block: Option<Spanned<Either<BlockExpression, Box<IfExpression>>>>,
}

#[derive(Debug, Clone)]
pub struct Statement {
    kind: StatementKind,
    span: Span,
}

#[derive(Debug, Clone)]
pub enum Mutability {
    Mutable,
    Immutable,
}

#[derive(Debug, Clone)]
pub enum StatementKind {
    Let {
        name: Identifier,
        mutability: Mutability,
        ty: Option<TypeRepr>,
        initializer: Option<Expression>,
    },
    ExpressionWithBlock(ExpressionWithBlock),
    ExpressionWithoutBlock(ExpressionWithoutBlock),
}

#[derive(Debug, Clone)]
pub struct TypeRepr {
    kind: TypeReprKind,
    span: Span,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct WhenArm {
    case: TypeRepr,
    guard: Option<Box<Expression>>,
    block: Spanned<ExpressionWithBlock>,
    span: Span,
}

#[derive(Debug, Clone)]
pub enum NegateOperator {
    Arithmetic,
    Logical,
}

#[derive(Debug, Clone)]
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

impl TryFrom<Token> for ArithmeticOrLogicalOperator {
    type Error = ();

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Basic(basic) => match basic {
                BasicToken::Plus => Ok(Self::Plus),
                BasicToken::Hyphen => Ok(Self::Minus),
                BasicToken::Asterisk => Ok(Self::Times),
                BasicToken::FSlash => Ok(Self::Divide),
                BasicToken::Ampersand => Ok(Self::And),
                BasicToken::Bar => Ok(Self::Or),
                BasicToken::LAngle2 => Ok(Self::LShift),
                BasicToken::RAngle2 => Ok(Self::RShift),
                _ => Err(()),
            },
            _ => Err(()),
        }
    }
}

impl From<ArithmeticOrLogicalOperator> for BasicToken {
    fn from(value: ArithmeticOrLogicalOperator) -> Self {
        match value {
            ArithmeticOrLogicalOperator::Plus => Self::Plus,
            ArithmeticOrLogicalOperator::Minus => Self::Hyphen,
            ArithmeticOrLogicalOperator::Times => Self::Asterisk,
            ArithmeticOrLogicalOperator::Divide => Self::FSlash,
            ArithmeticOrLogicalOperator::And => Self::Ampersand,
            ArithmeticOrLogicalOperator::Or => Self::Bar,
            ArithmeticOrLogicalOperator::LShift => Self::LAngle2,
            ArithmeticOrLogicalOperator::RShift => Self::RAngle2,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ComparisonOperator {
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
}

impl TryFrom<Token> for ComparisonOperator {
    type Error = ();

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Basic(basic) => match basic {
                BasicToken::Equal2 => Ok(Self::Eq),
                BasicToken::BangEqual => Ok(Self::Ne),
                BasicToken::RAngle => Ok(Self::Gt),
                BasicToken::LAngle => Ok(Self::Lt),
                BasicToken::RAngleEqual => Ok(Self::Ge),
                BasicToken::LAngleEqual => Ok(Self::Le),
                _ => Err(()),
            },
            _ => Err(()),
        }
    }
}

impl From<ComparisonOperator> for BasicToken {
    fn from(value: ComparisonOperator) -> Self {
        match value {
            ComparisonOperator::Eq => Self::Equal2,
            ComparisonOperator::Ne => Self::BangEqual,
            ComparisonOperator::Gt => Self::RAngle,
            ComparisonOperator::Lt => Self::LAngle,
            ComparisonOperator::Ge => Self::RAngleEqual,
            ComparisonOperator::Le => Self::LAngleEqual,
        }
    }
}

#[derive(Debug, Clone)]
pub enum LazyBooleanOperator {
    Or,
    And,
}

impl TryFrom<Token> for LazyBooleanOperator {
    type Error = ();

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Basic(basic) => match basic {
                BasicToken::Bar2 => Ok(Self::Or),
                BasicToken::Ampersand2 => Ok(Self::And),
                _ => Err(()),
            },
            _ => Err(()),
        }
    }
}

impl From<LazyBooleanOperator> for BasicToken {
    fn from(value: LazyBooleanOperator) -> Self {
        match value {
            LazyBooleanOperator::And => Self::Ampersand2,
            LazyBooleanOperator::Or => Self::Bar2,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AssignmentOperator {
    Equal,
    PlusEqual,
    MinusEqual,
    TimesEqual,
    DivideEqual,
    AndEqual,
    OrEqual,
    LShiftEqual,
    RShiftEqual,
}

impl TryFrom<Token> for AssignmentOperator {
    type Error = ();

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Basic(basic) => match basic {
                BasicToken::Equal2 => Ok(Self::Equal),
                BasicToken::PlusEqual => Ok(Self::PlusEqual),
                BasicToken::HyphenEqual => Ok(Self::MinusEqual),
                BasicToken::AsteriskEqual => Ok(Self::TimesEqual),
                BasicToken::FSlashEqual => Ok(Self::DivideEqual),
                BasicToken::AmpersandEqual => Ok(Self::AndEqual),
                BasicToken::BarEqual => Ok(Self::OrEqual),
                BasicToken::LAngle2Equal => Ok(Self::LShiftEqual),
                BasicToken::RAngle2Equal => Ok(Self::RShiftEqual),
                _ => Err(()),
            },
            _ => Err(()),
        }
    }
}

impl From<AssignmentOperator> for BasicToken {
    fn from(value: AssignmentOperator) -> Self {
        match value {
            AssignmentOperator::Equal => Self::Equal2,
            AssignmentOperator::PlusEqual => Self::PlusEqual,
            AssignmentOperator::MinusEqual => Self::HyphenEqual,
            AssignmentOperator::TimesEqual => Self::AsteriskEqual,
            AssignmentOperator::DivideEqual => Self::FSlashEqual,
            AssignmentOperator::AndEqual => Self::AmpersandEqual,
            AssignmentOperator::OrEqual => Self::BarEqual,
            AssignmentOperator::LShiftEqual => Self::LAngle2Equal,
            AssignmentOperator::RShiftEqual => Self::RAngle2Equal,
        }
    }
}
