use crate::model::types;

pub enum Symbol {
    Type(types::Type),
    Variable {
        ty: types::Type,
        is_assignable: bool,
    },
    Constant(types::Type),
}
