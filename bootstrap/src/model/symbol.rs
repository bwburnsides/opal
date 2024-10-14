use crate::model::types;

pub enum Symbol {
    Type(types::Type),
    Variable(types::Type),
    Constant(types::Type),
}
