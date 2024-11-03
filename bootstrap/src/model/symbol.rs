use crate::model::types;

#[derive(Debug, Clone)]
pub enum Symbol {
    Type(types::Type),
    Variable {
        ty: types::Type,
        is_assignable: bool,
    },
    Constant(types::Type),
}
