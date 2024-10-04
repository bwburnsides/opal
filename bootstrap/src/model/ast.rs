use either::Either;

pub type Jewel = Vec<Item>;

#[derive(Debug)]
pub struct FunctionItem {
    name: String,
    parameters: Vec<Parameter>,
    return_type: Option<TypeRepr>,
    body: BlockExpression,
}

impl FunctionItem {
    pub fn new(
        name: String,
        parameters: Vec<Parameter>,
        return_type: Option<TypeRepr>,
        body: BlockExpression,
    ) -> Self {
        Self {
            name: name,
            parameters: parameters,
            return_type: return_type,
            body: body,
        }
    }
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

impl StructItem {
    pub fn new(name: String, fields: Vec<Field>) -> Self {
        Self {
            name: name,
            fields: fields,
        }
    }
}

#[derive(Debug)]
pub struct EnumItem {
    name: String,
    variants: Vec<String>,
}

impl EnumItem {
    pub fn new(name: String, variants: Vec<String>) -> Self {
        Self {
            name: name,
            variants: variants,
        }
    }
}

#[derive(Debug)]
pub struct ConstItem {
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
    // Module(ModuleItem),
    Function(FunctionItem),
    TypeAlias(TypeAliasItem),
    Struct(StructItem),
    Enum(EnumItem),
    Constant(ConstItem),
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
    /* & */      Borrow(bool, Box<Expression>),
    /* * */      Dereference(Box<Expression>),
    /* expr */   ErrorPropagation(Box<Expression>),
    /* - OR ! */ Negation(NegateOperator, Box<Expression>),
    /* expr */   ArithmeticOrLogical(
        Box<Expression>,
        ArithmeticOrLogicalOperator,
        Box<Expression>,
    ),
    /* expr */   Comparison(Box<Expression>, ComparisonOperator, Box<Expression>),
    /* expr */   LazyBoolean(Box<Expression>, LazyBooleanOperator, Box<Expression>),
    /* expr */   Assignment(Box<Expression>, Box<Expression>),
    /* expr */   CompoundAssignment(Box<Expression>, CompoundAssignmentOperator, Box<Expression>),
}

#[derive(Debug)]
pub enum ExpressionWithoutBlock {
    Literal(LiteralExpression),
    Path(bool, Vec<String>),
    Operator(OperatorExpression),
    /* ( */      Grouped(Box<Expression>),
    /* [ */      Array(Vec<Box<Expression>>),
    /* expr */   Index(Box<Expression>, Box<Expression>),
    /* expr */   Call(Box<Expression>, Vec<Box<Expression>>),
    /* expr */   Field(Box<Expression>, String),
    /* return */ Return(Option<Box<Expression>>),
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
