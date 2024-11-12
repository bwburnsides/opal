use crate::ast::{ExpressionKind, UnaryOperator, UntypedExpression};
use crate::lexer::Token;

use crate::parser::precedence::Precedence;
use crate::parser::{parse_arms, parse_block, TokenStream};
use crate::span::{Span, Spanned};

impl Precedence<OperatorPrecedence> for TokenStream {
    fn check_precedence(&self) -> OperatorPrecedence {
        todo!()
    }
}

#[derive(PartialEq, PartialOrd)]
pub enum OperatorPrecedence {
    Minimum,
}

pub fn parse_expression(
    precedence: OperatorPrecedence,
    tokens: &mut TokenStream,
) -> Result<UntypedExpression, ()> {
    let mut left = parse_prefix(tokens)?;

    while precedence < tokens.check_precedence() {
        left = parse_infix(left, tokens)?;
    }

    Ok(left)
}

fn parse_prefix(tokens: &mut TokenStream) -> Result<UntypedExpression, ()> {
    use Token::*;

    match tokens.pop() {
        LeftParen => {
            let inner = parse_expression(OperatorPrecedence::Minimum, tokens)?;
            match tokens.pop() {
                RightParen => Ok(UntypedExpression::new(
                    Span(0, 0),
                    ExpressionKind::Grouped {
                        expr: Box::new(inner),
                        extra: (),
                    },
                )),
                _ => Err(()),
            }
        }
        IntLit(val, _) => Ok(UntypedExpression::new(
            Span(0, 0),
            ExpressionKind::Integer {
                value: val,
                extra: (),
            },
        )),
        StringLit(st) => Ok(UntypedExpression::new(
            Span(0, 0),
            ExpressionKind::String {
                data: st,
                extra: (),
            },
        )),
        CharLit(ch) => Ok(UntypedExpression::new(
            Span(0, 0),
            ExpressionKind::Character {
                data: ch,
                extra: (),
            },
        )),
        Name(name) => {
            let mut segments = vec![Spanned(name, Span(0, 0))];

            loop {
                if tokens.did_pop(ColonColon) {
                    segments.push(Spanned(tokens.expect_name()?, Span(0, 0)));
                } else {
                    break;
                }
            }

            Ok(UntypedExpression::new(
                Span(0, 0),
                ExpressionKind::Path {
                    is_root: false,
                    segments,
                    extra: (),
                },
            ))
        }
        ColonColon => {
            let mut segments = vec![Spanned(tokens.expect_name()?, Span(0, 0))];

            loop {
                if tokens.did_pop(ColonColon) {
                    segments.push(Spanned(tokens.expect_name()?, Span(0, 0)));
                } else {
                    break;
                }
            }

            Ok(UntypedExpression::new(
                Span(0, 0),
                ExpressionKind::Path {
                    is_root: true,
                    segments,
                    extra: (),
                },
            ))
        }
        True => Ok(UntypedExpression::new(
            Span(0, 0),
            ExpressionKind::Bool {
                data: true,
                extra: (),
            },
        )),
        False => Ok(UntypedExpression::new(
            Span(0, 0),
            ExpressionKind::Bool {
                data: false,
                extra: (),
            },
        )),
        LeftBrace => todo!("Block"),
        LeftSquare => todo!("Array"),
        Case => {
            let scrutinee = parse_expression(OperatorPrecedence::Minimum, tokens)?;
            tokens.expect(LeftBrace)?;
            let arms = parse_arms(tokens)?;
            tokens.expect(RightBrace)?;
            Ok(UntypedExpression::new(
                Span(0, 0),
                ExpressionKind::Case {
                    subjects: Box::new(scrutinee),
                    clauses: arms,
                    extra: (),
                },
            ))
        }
        For => {
            tokens.expect(For)?;
            let target = tokens.expect_name()?;
            tokens.expect(In)?;
            let expr = parse_expression(OperatorPrecedence::Minimum, tokens)?;
            let body = parse_block(tokens)?;
            Ok(UntypedExpression::new(
                Span(0, 0),
                ExpressionKind::For {
                    target: Spanned(target, Span(0, 0)),
                    iterable: Box::new(expr),
                    body,
                    extra: (),
                },
            ))
        }
        Continue => Ok(UntypedExpression::new(
            Span(0, 0),
            ExpressionKind::Continue(()),
        )),
        Break => Ok(UntypedExpression::new(
            Span(0, 0),
            ExpressionKind::Break(()),
        )),
        Return => {
            let expr = match tokens.peek() {
                LeftParen
                | IntLit(_, _)
                | StringLit(_)
                | CharLit(_)
                | Name(_)
                | ColonColon
                | True
                | False
                | LeftBrace
                | LeftSquare
                | Plus
                | Minus
                | Star
                | Amper
                | Bang
                | Case
                | For
                | Continue
                | Break
                | Return => Some(Box::new(parse_expression(
                    OperatorPrecedence::Minimum,
                    tokens,
                )?)),
                _ => None,
            };

            Ok(UntypedExpression::new(
                Span(0, 0),
                ExpressionKind::Return { expr, extra: () },
            ))
        }
        op => Ok(UntypedExpression::new(
            Span(0, 0),
            ExpressionKind::Unary {
                operator: UnaryOperator::try_from(op)?,
                expr: Box::new(parse_expression(OperatorPrecedence::Minimum, tokens)?),
                extra: (),
            },
        )),
    }
}

fn parse_infix(_left: UntypedExpression, tokens: &TokenStream) -> Result<UntypedExpression, ()> {
    use Token::*;

    match tokens.peek() {
        Plus | Minus | Star | Slash | Less | Greater | LessEqual | GreaterEqual | Equal
        | EqualEqual | NotEqual | LtLt | GtGt => todo!("Binary Operator"),

        LeftParen => todo!("Call"),
        Dot => todo!("Field Access"), // TODO: Prevent space between receiver and member? (avoid: recv . mem)
        Question => todo!("Error Propagate"),

        _ => Err(()),
    }
}
