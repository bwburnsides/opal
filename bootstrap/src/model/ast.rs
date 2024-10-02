use either::Either;

pub type ModuleItem = Vec<Item>;

#[derive(Debug)]
pub struct FunctionItem {
    name: String,
    parameters: Vec<Parameter>,
    return_type: Option<TypeRepr>,
    body: Option<BlockExpression>,
}

#[derive(Debug)]
pub struct TypeAliasItem {
    name: String,
    ty: TypeRepr,
}

#[derive(Debug)]
pub struct StructItem {
    name: String,
    fields: Vec<Field>,
}

#[derive(Debug)]
pub struct EnumItem {
    name: String,
    variants: Vec<String>,
}

#[derive(Debug)]
pub struct ConstantItem {
    name: String,
    ty: TypeRepr,
    value: Expression,
}

#[derive(Debug)]
pub struct StaticItem {
    name: String,
    ty: TypeRepr,
    value: Expression,
}

#[derive(Debug)]
pub enum Item {
    Module(ModuleItem),
    Func(FunctionItem),
    TypeAlias(TypeAliasItem),
    Struct(StructItem),
    Enum(EnumItem),
    Constant(ConstantItem),
    Static(StaticItem),
}

#[derive(Debug)]
pub struct Parameter {
    is_mutable: bool,
    name: String,
    ty: TypeRepr,
}

#[derive(Debug)]
pub struct Field {
    name: String,
    ty: TypeRepr,
}

#[derive(Debug)]
pub enum BuiltinTypeRepr {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    Bool,
    Char,
    Str,
}

#[derive(Debug)]
pub enum TypeRepr {
    Builtin(BuiltinTypeRepr),
    Array(Box<TypeRepr>, usize),
    Reference(bool, Box<TypeRepr>),
    Parenthesized(Box<TypeRepr>),
    Path(bool, Vec<String>),
}

#[derive(Debug)]
pub struct LetStatement {
    name: String,
    ty: TypeRepr,
    initializer: Expression,
}

#[derive(Debug)]
pub enum ExpressionStatement {
    WithoutBlock(ExpressionWithoutBlock),
    WithBlock(ExpressionWithBlock),
}

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    Expression(ExpressionStatement),
    Continue,
    Break,
}

#[derive(Debug)]
pub enum LiteralExpression {
    Char(char),
    String(String),
    Integer(usize),
    True,
    False,
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

#[derive(Debug)]
pub enum OperatorExpression {
    Borrow(bool, Box<Expression>),
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
}

#[derive(Debug)]
pub enum ExpressionWithoutBlock {
    Literal(LiteralExpression),
    Path(bool, Vec<String>),
    Operator(OperatorExpression),
    Grouped(Box<Expression>),
    Array(Vec<Box<Expression>>),
    Index(Box<Expression>, Box<Expression>),
    Call(Box<Expression>, Vec<Box<Expression>>),
    Field(Box<Expression>, String),
    Return(Option<Box<Expression>>),
}

pub type BlockExpression = Vec<Statement>;

#[derive(Debug)]
pub struct IfExpression {
    pub predicate: Box<Expression>,
    pub then_block: BlockExpression,
    pub else_block: Option<Either<BlockExpression, Box<IfExpression>>>,
}

#[derive(Debug)]
pub struct WhenArm {
    case: TypeRepr,
    guard: Option<Box<Expression>>,
    block: ExpressionWithBlock,
}

#[derive(Debug)]
pub struct WhenExpression {
    scrutinee: Box<Expression>,
    arms: Vec<WhenArm>,
}

#[derive(Debug)]
pub enum LoopExpression {
    For(String, Box<Expression>, BlockExpression),
    While(Box<Expression>, BlockExpression),
}

#[derive(Debug)]
pub enum ExpressionWithBlock {
    Block(BlockExpression),
    If(IfExpression),
    When(WhenExpression),
    Loop(LoopExpression),
}

#[derive(Debug)]
pub enum Expression {
    WithoutBlock(ExpressionWithoutBlock),
    WithBlock(ExpressionWithBlock),
}
