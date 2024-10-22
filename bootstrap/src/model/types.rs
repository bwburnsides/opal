use crate::model::Mutability;

#[derive(Debug, PartialEq)]
pub enum Type {
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
    Never,
    Array(Box<Type>, u32),
    Reference(Mutability, Box<Type>),
    Function,
    String, // User defined types. WIP.
}
