pub type VariableId = usize;
pub type LabelId = usize;

pub enum Statement {
    Assign {
        target: VariableId,
        source: VariableId,
    },
    Goto(LabelId),
    GotoIf(VariableId, LabelId),
    BinaryOp {
        op: BinaryOp,
        target: VariableId,
        left: VariableId,
        right: VariableId,
    },
    BinaryOpImmediate {
        op: BinaryOp,
        target: VariableId,
        left: VariableId,
        right: usize,
    },
    UnaryOp {
        op: UnaryOp,
        target: VariableId,
        right: VariableId,
    }
}

pub enum BinaryOp {
    Add,
    Mult,
    Sub,
}

pub enum UnaryOp {
    Negate
}