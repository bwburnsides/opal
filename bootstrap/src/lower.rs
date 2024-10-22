use crate::model::typed_ast::*;
use crate::model::ast::*;

// Convert TAST to TAC IR Statement Stream

fn generate_ir(node: TypedExpression) {
    match node.kind {
        ExpressionKind::WithBlock(wb) => match wb {
            _ => todo!()
        },
        ExpressionKind::WithoutBlock(wob) => match wob {
            _ => todo!()
        }
    }
}
