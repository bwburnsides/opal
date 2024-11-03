enum Constant {
    U8(u8),
    U16(u16),
    U32(u32),
    I8(i8),
    I16(i16),
    I32(i32),
}

enum Instruction {
    Assign(VariableId, VariableId),
    AssignConstant(VariableId, Constant),
}

enum Decision {
    Success(/* Body to be executed? */),
    
}