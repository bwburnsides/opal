use std::convert::Infallible;

use super::*;
use expr::*;

pub struct Untyped;

impl ExpressionData<Untyped> for Untyped {
    type Grouped = ();
    type Integer = ();
    type String = ();
    type Character = ();
    type Name = ();
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

    type Other = Infallible;
}

pub type UntypedExpression = Expression<Untyped>;
