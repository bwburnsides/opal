use crate::ast::expr::{Expression, ExpressionData};
use crate::ast::{UntypedArgument, UntypedStatement};
use crate::error::ParseError;

pub struct Untyped;

impl ExpressionData<Untyped> for Untyped {
    type Grouped = ();
    type Integer = ();
    type String = ();
    type Character = ();
    type Name = ();
    type Bool = ();
    type Block = ();
    type Array = ();
    type Assign = ();
    type Call = ();
    type FieldAccess = ();
    type Binary = ();
    type Unary = ();
    type Propagate = ();
    type Case = ();
    type For = ();
    type Continue = ();
    type Break = ();
    type Return = ();

    type BlockBody = UntypedStatement;
    type ArgumentKind = UntypedArgument;

    type Other = ParseError;
}

pub type UntypedExpression = Expression<Untyped>;
