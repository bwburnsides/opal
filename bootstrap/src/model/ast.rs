use crate::span::Spanned;
use crate::model::ttg::TreeData;
use crate::model::base::{Mutability, Path};

pub enum Syntax {}

impl TreeData<Syntax> for Syntax {
    type Case = ();
    type IfIs = ();
    type For = ();
    type ErrorPropagation = ();
    type Return = ();
    type Break = ();
    type Continue = ();
    type Block = ();
    type Grouped = ();
    type Path = ();
    type Literal = ();
    type Array = ();
    type Prefix = ();
    type Binary = ();
    type Call = ();
    type Field = ();
    type Index = ();
    type Other = ();

    type NameRepresentation = Spanned<String>;
    type PathRepresentation = Vec<Self::NameRepresentation>;
    type TypeRepresentation = Type;
}

pub type Type = Spanned<TypeKind>;

pub enum TypeKind {
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
    Array(Box<Type>, u32),
    Reference(Mutability, Box<Type>),
    Parenthesized(Box<Type>),
    Path(<Syntax as TreeData<Syntax>>::PathRepresentation),
}
