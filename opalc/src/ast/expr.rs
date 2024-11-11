use super::*;

pub trait ExpressionData<Phase> {
    type Grouped;
    type Integer;
    type String;
    type Character;
    type Name;
    type Bool;
    type Block;
    type Array;
    type Assign;
    type Call;
    type FieldAccess;
    type Binary;
    type Unary;
    type Propagate;
    type Case;
    type For;
    type Continue;
    type Break;
    type Return;

    type BlockBody;
    type ArgumentKind;

    type Other;
}

pub struct Expression<P>
where
    P: ExpressionData<P>,
{
    span: Span,
    kind: ExpressionKind<P>,
}

impl<P> Expression<P>
where
    P: ExpressionData<P>,
{
    pub fn new(span: Span, kind: ExpressionKind<P>) -> Self {
        Self { span, kind }
    }
}

pub enum ExpressionKind<P>
where
    P: ExpressionData<P>,
{
    Grouped {
        expr: Box<Expression<P>>,
        extra: P::Grouped,
    },

    Integer {
        value: u32,
        extra: P::Integer,
    },

    String {
        data: String,
        extra: P::String,
    },

    Character {
        data: char,
        extra: P::Character,
    },

    // TODO: Should be a Path
    Name {
        name: String,
        extra: P::Name,
    },

    Bool {
        data: bool,
        extra: P::Bool,
    },

    Block {
        statements: Vec<P::BlockBody>,
        extra: P::Block,
    },

    Array {
        elements: Vec<Expression<P>>,
        extra: P::Array,
    },

    Assign {
        left: Box<Expression<P>>,
        operator: BinaryOperator,
        right: Box<Expression<P>>,
        extra: P::Assign,
    },

    Call {
        function: Box<Expression<P>>,
        arguments: Vec<P::ArgumentKind>,
        extra: P::Call,
    },

    FieldAccess {
        field_span: Span,
        field: Spanned<String>,
        receiver: Box<Expression<P>>,
        extra: P::FieldAccess,
    },

    Binary {
        left: Box<Expression<P>>,
        operator: BinaryOperator,
        right: Box<Expression<P>>,
        extra: P::Binary,
    },

    Unary {
        operator: UnaryOperator,
        expr: Box<Expression<P>>,
        extra: P::Unary,
    },

    Propagate {
        expr: Box<Expression<P>>,
        extra: P::Propagate,
    },

    Case {
        subjects: Box<Expression<P>>,
        clauses: Vec<Arm<Expression<P>>>,
        extra: P::Case,
    },

    For {
        target: Spanned<String>,
        iterable: Box<Expression<P>>,
        body: Vec<P::BlockBody>,
        extra: P::For,
    },

    Continue(P::Continue),

    Break(P::Break),

    Return {
        expr: Option<Box<Expression<P>>>,
        extra: P::Return,
    },

    Other(P::Other),
}
