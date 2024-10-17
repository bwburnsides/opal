// use crate::model::{Expression, ExpressionKind, ExpressionWithoutBlock, NegateOperator, Type};
// use crate::scope::Scope;
// use crate::check::infer_type::*;
// use crate::span::Spanned;

// pub fn check(scope: &Scope, expr: Expression) -> Option<Type> {
//     match expr.item {
//         ExpressionKind::WithoutBlock(wo_block) => {
//             use ExpressionWithoutBlock::*;
//             match wo_block {
//                 Character(_) => Some(Type::Char),
//                 String(_) => Some(Type::Str),
//                 Integer(val) => Some(infer_integer_literal_type(val)),
//                 True => Some(Type::Bool),
//                 False => Some(Type::Bool),
//                 Unit => Some(Type::Unit),
//                 Path(path) => {
//                     // For foo::bar::baz:
//                     //  - Lookup foo in current scope
//                     //  - Symbol should have own scope; look up bar in that scope
//                     //  - That symbol should have own scope; look up baz in that scope
//                     //  - Yield the type of baz
//                     //
//                     // For ::foo::bar::baz:
//                     //  - Lookup foo in global scope
//                     //  - Above process continues normally
//                     //
//                     // So either every scope should have a direct pointer to the global scope,
//                     // or we'll have to do a linked-list style daisy chained call all the way up
//                     // to the global scope.
//                     todo!()
//                 },
//                 Borrow {mutability, expr} => {
//                     let expr_type = check(scope, *expr)?;
//                     Some(Type::Reference(mutability, Box::new(expr_type)))
//                 },
//                 Dereference(expr) => {
//                     let expr_type = check(scope, *expr)?;
//                     if let Type::Reference(mutability, base) = expr_type {
//                         Some(*base)  // TODO: Handle mutability
//                     } else {
//                         None
//                     }
//                 },
//                 ErrorPropagation(_) => todo!("The scope needs to store information about the enclosing return context"),
//                 Negation(op, expr) => {
//                     match check(scope, *expr)? {
//                         Type::U8 => None,
//                         Type::I8 => Some(Type::I8),
//                         Type::U16 => None,
//                         Type::I16 => Some(Type::I16),
//                         Type::U32 => None,
//                         Type::I32 => Some(Type::I32),
//                         Type::Bool => Some(Type::Bool),
//                         Type::Reference(_, base) => None,
//                     }

//                 }
//                 _ => todo!()
//             }
//         },
//         ExpressionKind::WithBlock(w_block) => todo!(),
//     }
// }

// pub fn check_negation(scope: &Scope, op: NegateOperator, expr: &Spanned<ExpressionKind>) -> Option<Type> {
//     match check(scope, *expr)? {
//         Type::U8 => None,
//         Type::I8 => Some(Type::I8),
//         Type::U16 => None,
//         Type::I16 => Some(Type::I16),
//         Type::U32 => None,
//         Type::I32 => Some(Type::I32),
//         Type::Bool => Some(Type::Bool),
//         Type::Reference(_, base) => None,
//     }
// }
