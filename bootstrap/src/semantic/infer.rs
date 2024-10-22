use crate::model::Type;

pub fn infer_integer_literal_type(val: u32) -> Type {
    if val < 2_u32.pow(8) {
        Type::U8
    } else if val < 2_u32.pow(16) {
        Type::U16
    } else {
        Type::U32
    }
}
