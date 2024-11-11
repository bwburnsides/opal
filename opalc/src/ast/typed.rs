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
    type Bool = ();
    type Block = (); // Get type from last value in Vec
    type Array = ty::Ty;
    type Assign = (); // Always ()
    type Call = ty::Ty;
    type FieldAccess = ty::Ty; // Gleam doesn't have this in their typed AST for some reason
    type Binary = ty::Ty;
    type Unary = ty::Ty;
    type Propagate = ty::Ty;
    type Case = ty::Ty;
    type For = ty::Ty;
    type Continue = (); // Always !
    type Break = (); // Always !
    type Return = (); // Always !

    type BlockBody = TypedStatement;
    type ArgumentKind = TypedArgument;

    type Other = Infallible;
}

pub type TypedExpression = Expression<Typed>;
