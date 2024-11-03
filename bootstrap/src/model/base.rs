/// This intends to be a "pass-indepedent" model of the language. What this means is that
/// it represents the language constructs in a way that is agnostic to what the type of analysis
/// currently being performed is. The core construction here is the `TreeData` trait, which allows
/// arbitrary data to be attached to each kind of expression, and which allows existing constructors
/// to be disabled or new construcots to be added, per pass. Here are some examples in how this is
/// hopefully going to be useful.
/// 
/// 1. This model can be parameterized with a marker-type `T` satisfying `TreeData<T>` in order to
///     attach resolved types to each expression node. This allows for semantic analysis and parsing
///     to use distinct types without redefining the entire language model to provide only minor
///     variations in the fields.
/// 
/// 2. The model can be desugared to remove specific forms during different passes. For example,
///     IfIs expressions can be desugared to Case expressions prior to semantic analysis. Additionally,
///     Case expressions can be replaced with Decision trees prior to lowering.
/// 
/// There are other tricks that I am hoping to employ here. There are some nodes that are not at the
/// expression level that store some type representation in them; for example, the type annotation of
/// a `let` statement. Ideally, that type representation can be transformed from a syntax type representation
/// to a reified type checked representation during type checking. Also, many nodes carry Identifiers and
/// paths that may or may not be unique across the entire Geode (project tree). During name resolution,
/// these paths can be replaced with some sort of "ResolvedName" representation.
/// 
/// Both of these things can hopefully be achieved by parameterizing the tree over new type parameters
/// for "type representation" and "name representation", then allowing them to flow down to their use sites.
use crate::span::{Spanned, Span};
use crate::model::ttg::TreeData;

pub struct Geode<P> where P: TreeData<P> {
    name: P::NameRepresentation,
    modules: Vec<Module<P>>,
}

pub struct Module<P> where P: TreeData<P> {
    name: P::NameRepresentation,
    items: Vec<Item<P>>,
}

pub type Item<P> = Spanned<ItemKind<P>>;

pub enum ItemKind<P> where P: TreeData<P> {
    Use(UseTree<P>),
    Function(FunctionItem<P>),
    TypeAlias(TypeAliasItem<P>),
    Struct(StructItem<P>),
    Enum(EnumItem<P>),
    Const(ConstItem<P>),
    Static(StaticItem<P>),
}

pub struct UseTree<P> where P: TreeData<P> {
    prefix: P::PathRepresentation,
    kind: UseTreeKind<P>,
}

pub enum UseTreeKind<P> where P: TreeData<P> {
    Simple(Option<P::NameRepresentation>),  // `use prefix` or `use prefix as name`
    Nested(Vec<UseTree<P>>, Span),  // `use prefix::{...}`
    Glob,  // `use prefix::*``
}

pub struct FunctionItem<P> where P: TreeData<P> {
    name: P::NameRepresentation,
    parameters: Vec<Parameter<P>>,
    return_type: Option<P::TypeRepresentation>,
    body: BlockExpression<P>,
}

pub struct TypeAliasItem<P> where P: TreeData<P> {
    name: P::NameRepresentation,
    ty: P::TypeRepresentation,
}

pub struct StructItem<P> where P: TreeData<P> {
    name: P::NameRepresentation,
    fields: Vec<Field<P>>,
}

pub struct EnumItem<P> where P: TreeData<P> {
    name: P::NameRepresentation,
    variants: Vec<Variant<P>>
}

pub struct ConstItem<P> where P: TreeData<P> {
    name: P::NameRepresentation,
    ty: P::TypeRepresentation,
    value: Expression<P>,
}

pub struct StaticItem<P> where P: TreeData<P> {
    name: P::NameRepresentation,
    ty: P::TypeRepresentation,
    value: Expression<P>,
}

pub struct Parameter<P> where P: TreeData<P> {
    mutability: Mutability,
    ty: P::TypeRepresentation,
    span: Span
}

pub struct Field<P> where P: TreeData<P> {
    name: P::NameRepresentation,
    ty: P::TypeRepresentation,
    span: Span,
}

pub enum Variant<P> where P: TreeData<P> {
    Unit(P::NameRepresentation),
    Tuple(P::NameRepresentation, Vec<P::TypeRepresentation>),
    Struct(P::NameRepresentation, Vec<Field<P>>),
}

pub type Expression<Phase> = Spanned<ExpressionKind<Phase>>;

pub type BlockExpression<Phase> = Vec<Option<Statements<Phase>>>;

pub enum ExpressionKind<P> where P: TreeData<P> {
    ExpressionWithBlock(ExpressionWithBlock<P>),
    ExpressionWithoutBlock(ExpressionWithoutBlock<P>)
}

pub enum ExpressionWithBlock<P> where P: TreeData<P> {
    Case(Box<Expression<P>>, Vec<Arm<P>>, P::Case),
    IfIs(Box<Expression<P>>, Pattern, Box<Expression<P>>, Option<Box<Expression<P>>>, P::IfIs),
    For(P::NameRepresentation, Box<Expression<P>>, BlockExpression<P>, P::For),
    Block(BlockExpression<P>, P::Block),
}

pub enum ExpressionWithoutBlock<P> where P: TreeData<P> {
    ErrorPropagation(Box<Expression<P>>, P::ErrorPropagation),
    Return(Option<Box<Expression<P>>>, P::Return),
    Break(Option<Box<Expression<P>>>, P::Break),
    Continue(P::Continue),
    Grouped(Box<Expression<P>>, P::Grouped),
    Path(P::PathRepresentation, P::Path),
    Literal(Literal, P::Literal),
    Array(Vec<Expression<P>>, P::Array),
    Prefix(PrefixOperator, Box<Expression<P>>, P::Prefix),
    Binary(Box<Expression<P>>, BinaryOperator, Box<Expression<P>>, P::Binary),
    Call(Box<Expression<P>>, Vec<Expression<P>>, P::Call),
    Field(Box<Expression<P>>, P::NameRepresentation, P::Field),
    Index(Box<Expression<P>>, Box<Expression<P>>, P::Index),
    Other(P::Other),
}

pub enum Statement<P> where P: TreeData<P> {
    Empty,
    Let(Let<P>),
    Expression(Expression<P>),
}

pub enum Statements<P> where P: TreeData<P> {
    Leading(Statement<P>, Vec<Statement<P>>, Option<ExpressionWithoutBlock<P>>),
    Block(ExpressionWithoutBlock<P>)
}

pub struct Let<P> where P: TreeData<P> {
    name: P::NameRepresentation,
    mutability: Mutability,
    // TODO: Some new parameter to force this required after type checking.
    // Or maybe we just won't need this after name resolution and we'll disable the constructor.
    ty: Option<P::TypeRepresentation>,
    initializer: Option<Expression<P>>,
    span: Span,
}

pub enum Literal {
    Character(char),
    String(String),
    Integer(u32),
    True,
    False,
    Unit,
    Never,
}

pub struct Pattern;

pub struct Arm<P> where P: TreeData<P> {
    pattern: Spanned<Pattern>,
    guard: Option<Expression<P>>,
    expression: Expression<P>,
}

pub struct Path<P: TreeData<P>>(Vec<P::NameRepresentation>);

pub enum PrefixOperator {
    Borrow,
    MutableBorrow,
    DeReference,
    ArithmeticNegate,
    LogicalNegate,
}

pub enum BinaryOperator {
    Plus,
    Minus,
    Times,
    Divide,
    And,
    Or,
    LShift,
    RShift,
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
    LazyOr,
    LazyAnd,
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

#[derive(Debug, PartialEq, Clone)]
pub enum Mutability {
    Mutable,
    Immutable
}
