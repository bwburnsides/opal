use crate::model::*;
use crate::parse::expr::*;
use crate::parse::lex;
use crate::parse::stmt::*;
use crate::span::{Span, Spanned};
use crate::stream::*;

#[test]
fn lex_0() {
    let mut tokens = lex::tokenize("0").unwrap();
    assert_eq!(tokens.len(), 2);

    assert_eq!(
        tokens.pop(),
        Spanned::new(Token::Literal(LiteralToken::Integer(0)), Span::new(0, 1))
    );

    assert_eq!(tokens.pop(), Spanned::new(Token::Eof, Span::new(1, 2)));
}

#[test]
fn lex_1() {
    let mut tokens = lex::tokenize("1").unwrap();

    assert_eq!(tokens.len(), 2);

    assert_eq!(
        tokens.pop(),
        Spanned::new(Token::Literal(LiteralToken::Integer(1)), Span::new(0, 1))
    );

    assert_eq!(tokens.pop(), Spanned::new(Token::Eof, Span::new(1, 2)));
}

#[test]
fn lex_11() {
    let mut tokens = lex::tokenize("11").unwrap();

    assert_eq!(tokens.len(), 2);

    assert_eq!(
        tokens.pop(),
        Spanned::new(Token::Literal(LiteralToken::Integer(11)), Span::new(0, 2))
    );

    assert_eq!(tokens.pop(), Spanned::new(Token::Eof, Span::new(2, 3)));
}

#[test]
fn lex_10() {
    let mut tokens = lex::tokenize("10").unwrap();

    assert_eq!(tokens.len(), 2);

    assert_eq!(
        tokens.pop(),
        Spanned::new(Token::Literal(LiteralToken::Integer(10)), Span::new(0, 2))
    );

    assert_eq!(tokens.pop(), Spanned::new(Token::Eof, Span::new(2, 3)));
}

#[test]
fn lex_01() {
    let error = lex::tokenize("01").unwrap_err();

    assert_eq!(error.span, Span::new(1, 2));
}

#[test]
fn lex_hex_0() {
    let mut tokens = lex::tokenize("0x0").unwrap();

    assert_eq!(tokens.len(), 2);

    assert_eq!(
        tokens.pop(),
        Spanned::new(Token::Literal(LiteralToken::Integer(0x0)), Span::new(0, 3))
    );

    assert_eq!(tokens.pop(), Spanned::new(Token::Eof, Span::new(3, 4)));
}

#[test]
fn lex_hex_a() {
    let mut tokens = lex::tokenize("0xA").unwrap();

    assert_eq!(tokens.len(), 2);

    assert_eq!(
        tokens.pop(),
        Spanned::new(Token::Literal(LiteralToken::Integer(0xA)), Span::new(0, 3))
    );

    assert_eq!(tokens.pop(), Spanned::new(Token::Eof, Span::new(3, 4)));
}

#[test]
fn lex_hex_deadbeef() {
    let mut tokens = lex::tokenize("0xDEADBEEF").unwrap();

    assert_eq!(tokens.len(), 2);

    assert_eq!(
        tokens.pop(),
        Spanned::new(
            Token::Literal(LiteralToken::Integer(0xDEADBEEF)),
            Span::new(0, 10)
        )
    );

    assert_eq!(tokens.pop(), Spanned::new(Token::Eof, Span::new(10, 11)));
}

#[test]
fn lex_identifier() {
    let mut tokens = lex::tokenize("foo").unwrap();

    assert_eq!(tokens.len(), 2);

    assert_eq!(
        tokens.pop(),
        Spanned::new(Token::Identifier("foo".to_owned()), Span::new(0, 3))
    );

    assert_eq!(tokens.pop(), Spanned::new(Token::Eof, Span::new(3, 4)));
}

#[test]
fn lex_keyword() {
    let mut tokens = lex::tokenize("True").unwrap();

    assert_eq!(tokens.len(), 2);

    assert_eq!(
        tokens.pop(),
        Spanned::new(Token::Keyword(KeywordToken::True), Span::new(0, 4))
    );

    assert_eq!(tokens.pop(), Spanned::new(Token::Eof, Span::new(4, 5)));
}

#[test]
fn lex_string() {
    let mut tokens = lex::tokenize("\"Brady\"").unwrap();

    assert_eq!(tokens.len(), 2);

    assert_eq!(
        tokens.pop(),
        Spanned::new(
            Token::Literal(LiteralToken::String("Brady".to_owned())),
            Span::new(0, 7)
        )
    );
}

#[test]
fn parse_integer_literal() {
    let mut tokens: Stream<Token> = vec![Spanned::empty(Token::Literal(LiteralToken::Integer(4)))]
        .into_iter()
        .collect();

    let expr = expression(&mut tokens).unwrap();

    if let ExpressionKind::WithoutBlock(ExpressionWithoutBlock::Integer(4)) = expr.item {
        // assert!(true);
    } else {
        panic!();
    }
}

#[test]
fn parse_add_expr() {
    let mut tokens: Stream<Token> = vec![
        Spanned::empty(Token::Literal(LiteralToken::Integer(4))),
        Spanned::empty(Token::Basic(BasicToken::Plus)),
        Spanned::empty(Token::Literal(LiteralToken::Integer(2))),
    ]
    .into_iter()
    .collect();

    let expr = expression(&mut tokens).unwrap();

    if let ExpressionKind::WithoutBlock(ExpressionWithoutBlock::ArithmeticOrLogical(
        left,
        op,
        right,
    )) = expr.item
    {
        assert_eq!(op, ArithmeticOrLogicalOperator::Plus);

        match left.item {
            ExpressionKind::WithoutBlock(ExpressionWithoutBlock::Integer(4)) => { /* */ }
            _ => panic!(),
        }

        match right.item {
            ExpressionKind::WithoutBlock(ExpressionWithoutBlock::Integer(2)) => { /*assert!(true)*/ },
            _ => panic!(),
        }
    } else {
        panic!();
    }
}

#[test]
fn parse_add_assoc_expr() {
    let mut tokens: Stream<Token> = vec![
        Spanned::empty(Token::Literal(LiteralToken::Integer(4))),
        Spanned::empty(Token::Basic(BasicToken::Plus)),
        Spanned::empty(Token::Literal(LiteralToken::Integer(2))),
        Spanned::empty(Token::Basic(BasicToken::Plus)),
        Spanned::empty(Token::Literal(LiteralToken::Integer(1))),
    ]
    .into_iter()
    .collect();

    let expr = expression(&mut tokens).unwrap();

    if let ExpressionKind::WithoutBlock(ExpressionWithoutBlock::ArithmeticOrLogical(
        left,
        op,
        right,
    )) = expr.item
    {
        assert_eq!(op, ArithmeticOrLogicalOperator::Plus);

        match left.item {
            ExpressionKind::WithoutBlock(ExpressionWithoutBlock::ArithmeticOrLogical(
                left_inner,
                op_inner,
                right_inner,
            )) => {
                assert_eq!(op_inner, ArithmeticOrLogicalOperator::Plus);

                if let ExpressionKind::WithoutBlock(ExpressionWithoutBlock::Integer(4)) =
                    left_inner.item
                {
                    if let ExpressionKind::WithoutBlock(ExpressionWithoutBlock::Integer(2)) =
                        right_inner.item
                    {
                        // assert!(true)
                    } else {
                        panic!()
                    }
                } else {
                    panic!()
                }
            }
            _ => panic!(),
        }

        match right.item {
            ExpressionKind::WithoutBlock(ExpressionWithoutBlock::Integer(1)) => {},
            _ => panic!(),
        }
    } else {
        panic!();
    }
}

#[test]
fn parse_assign_expr() {
    let mut tokens: Stream<Token> = vec![
        Spanned::empty(Token::Identifier("foo".to_owned())),
        Spanned::empty(Token::Basic(BasicToken::Equal)),
        Spanned::empty(Token::Literal(LiteralToken::Integer(4))),
    ]
    .into_iter()
    .collect();

    let expr = expression(&mut tokens).unwrap();

    if let ExpressionKind::WithoutBlock(ExpressionWithoutBlock::Assignment(left, op, right)) =
        expr.item
    {
        assert_eq!(op, AssignmentOperator::Equal);

        if let ExpressionKind::WithoutBlock(ExpressionWithoutBlock::Path(Path {
            is_global: false,
            mut segments,
        })) = left.item
        {
            assert_eq!(segments.len(), 1);
            assert_eq!(segments.pop().unwrap().item, "foo".to_owned());

            if let ExpressionKind::WithoutBlock(ExpressionWithoutBlock::Integer(4)) = right.item {
                // assert!(true);
            } else {
                panic!();
            }
        } else {
            panic!()
        }
    } else {
        panic!()
    }
}

// #[test]
// fn parse_assign_assoc_expr() {
//     let mut tokens: Stream<Token> = vec![
//         Spanned::empty(Token::Identifier("foo".to_owned())),
//         Spanned::empty(Token::Basic(BasicToken::Equal)),
//         Spanned::empty(Token::Identifier("bar".to_owned())),
//         Spanned::empty(Token::Basic(BasicToken::Equal)),
//         Spanned::empty(Token::Literal(LiteralToken::Integer(4)))
//     ].into_iter().collect();

//     let expr = expression(&mut tokens).unwrap();

//     if let ExpressionKind::WithoutBlock(ExpressionWithoutBlock::Assignment(left, op, right)) = expr.kind {
//         assert_eq!(op, AssignmentOperator::Equal);

//         if let ExpressionKind::WithoutBlock(ExpressionWithoutBlock::Assignment(left_inner, op_inner, right_inner)) =
//     }
// }

#[test]
fn parse_let_statement_bare() {
    use BasicToken::*;
    use KeywordToken::*;
    use Token::*;

    let mut tokens: Stream<Token> = vec![
        Spanned::empty(Keyword(Let)),
        Spanned::empty(Identifier("foo".to_owned())),
        Spanned::empty(Basic(Semicolon)),
    ]
    .into_iter()
    .collect();

    let stmt = statement(&mut tokens).unwrap();

    match stmt {
        Statement::Let(inner) => {
            assert_eq!(inner.name.item, "foo".to_owned());
            assert_eq!(inner.mutability, Mutability::Immutable);
            assert_eq!(inner.ty, None);
            assert_eq!(inner.initializer, None);
        }
        _ => panic!(),
    }
}

#[test]
fn parse_let_statement_bare_mut() {
    use BasicToken::*;
    use KeywordToken::*;
    use Token::*;

    let mut tokens: Stream<Token> = vec![
        Spanned::empty(Keyword(Let)),
        Spanned::empty(Keyword(Mut)),
        Spanned::empty(Identifier("foo".to_owned())),
        Spanned::empty(Basic(Semicolon)),
    ]
    .into_iter()
    .collect();

    let stmt = statement(&mut tokens).unwrap();

    match stmt {
        Statement::Let(inner) => {
            assert_eq!(inner.name.item, "foo".to_owned());
            assert_eq!(inner.mutability, Mutability::Mutable);
            assert_eq!(inner.ty, None);
            assert_eq!(inner.initializer, None);
        }
        _ => panic!(),
    }
}

#[test]
fn parse_let_statement_typed() {
    use BasicToken::*;
    use KeywordToken::*;
    use Token::*;

    let mut tokens: Stream<Token> = vec![
        Spanned::empty(Keyword(Let)),
        Spanned::empty(Identifier("foo".to_owned())),
        Spanned::empty(Basic(Colon)),
        Spanned::empty(Identifier("Foo".to_owned())),
        Spanned::empty(Basic(Semicolon)),
    ]
    .into_iter()
    .collect();

    let stmt = statement(&mut tokens).unwrap();

    match stmt {
        Statement::Let(inner) => {
            assert_eq!(inner.name.item, "foo".to_owned());
            assert_eq!(inner.mutability, Mutability::Immutable);
            assert_eq!(
                inner.ty,
                Some(Spanned::empty(TypeReprKind::Path(Path::new(
                    false,
                    vec![Spanned::empty("Foo".to_owned())],
                ))))
            );
            assert_eq!(inner.initializer, None);
        }
        _ => panic!(),
    }
}

#[test]
fn parse_let_statement_typed_mut() {
    use BasicToken::*;
    use KeywordToken::*;
    use Token::*;

    let mut tokens: Stream<Token> = vec![
        Spanned::empty(Keyword(Let)),
        Spanned::empty(Keyword(Mut)),
        Spanned::empty(Identifier("foo".to_owned())),
        Spanned::empty(Basic(Colon)),
        Spanned::empty(Identifier("Foo".to_owned())),
        Spanned::empty(Basic(Semicolon)),
    ]
    .into_iter()
    .collect();

    let stmt = statement(&mut tokens).unwrap();

    match stmt {
        Statement::Let(inner) => {
            assert_eq!(inner.name.item, "foo".to_owned());
            assert_eq!(inner.mutability, Mutability::Mutable);
            assert_eq!(
                inner.ty,
                Some(Spanned::empty(TypeReprKind::Path(Path::new(
                    false,
                    vec![Spanned::empty("Foo".to_owned())],
                ))))
            );
            assert_eq!(inner.initializer, None);
        }
        _ => panic!(),
    }
}

// #[test]
// fn parse_let_statement_initialized() {
//     use BasicToken::*;
//     use KeywordToken::*;
//     use LiteralToken::*;
//     use Token::*;

//     let mut tokens: Stream<Token> = vec![
//         Spanned::empty(Keyword(Let)),
//         Spanned::empty(Identifier("foo".to_owned())),
//         Spanned::empty(Basic(Equal)),
//         Spanned::empty(Literal(Integer(5))),
//         Spanned::empty(Basic(Semicolon)),
//     ]
//     .into_iter()
//     .collect();

//     let stmt = statement(&mut tokens).unwrap();

//     match stmt.item {
//         StatementKind::Let(inner) => {
//             assert_eq!(inner.name.item, "foo".to_owned());
//             assert_eq!(inner.mutability, Mutability::Immutable);
//             assert_eq!(
//                 inner.ty,
//                 Some(Spanned::empty(TypeReprKind::Path(Path::new(
//                     false,
//                     Spanned::empty("Foo".to_owned()),
//                     Vec::new(),
//                 ))))
//             );
//             assert_eq!(inner.initializer, None);
//         }
//         _ => assert!(false),
//     }
// }
