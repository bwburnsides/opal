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
    Array(Box<Type>, u32),
    Reference(bool, Box<Type>),
    Function,
    String, // For user defined types
}
