use std::convert::Infallible;

use super::*;
use expr::*;

pub struct Typed;

impl ExpressionData<Typed> for Typed {
    type Grouped = ty::Ty;
    type Integer = ty::Ty;
    type String = ();
    type Character = ();
    type Name = ty::Ty;
    type Block = ();  // Get type from last value in Vec
    type Array = ty::Ty;
    type Assign = ();  // Always ()
    type Call = ty::Ty;
    type FieldAccess = ty::Ty;
    type Binary = ty::Ty;
    type Unary = ty::Ty;
    type Propagate = ty::Ty;
    type Case = ty::Ty;
    type For = ty::Ty;
    type Continue = ();  // Always !
    type Break = ();  // Always !
    type Return = ();  // Always !

    type BlockBody = TypedStatement;

    type Other = Infallible;
}

pub type TypedExpression = Expression<Typed>;
