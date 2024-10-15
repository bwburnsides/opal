#![allow(unused_variables)]

use crate::error::Error;
use crate::model::{
    AssignmentOperator, Expression, ExpressionKind, KeywordToken, LiteralToken, Token,
};
use crate::parse;
use crate::parse::BasicToken;
use crate::parse::{ExpressionWithoutBlock, ParseResult};
use crate::span::{Span, Spanned};
use crate::stream::{PeekFor, Stream};

use crate::parse::IdentifierToken;

use super::{
    ArithmeticOrLogicalOperator, BlockExpression, ComparisonOperator, LazyBooleanOperator,
};

type ParserFunction = dyn Fn(Precedence, Expression, &mut Stream<Token>) -> ParseResult<Expression>;

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Precedence {
    Minimum,
    Return,
    Assignment,
    LazyOr,
    LazyAnd,
    Comparison,
    Or,
    Xor,
    And,
    Shift,
    Additive,
    Multiplicative,
    Unary,
    ErrorPropagation,
    FunctionCall,
    FieldExpression,
    Path,
}

impl Precedence {
    pub fn right_associative(self) -> Precedence {
        use Precedence::*;

        match self {
            Minimum => Minimum,
            Return => Minimum,
            Assignment => Return,
            LazyOr => Assignment,
            LazyAnd => LazyOr,
            Comparison => LazyAnd,
            Or => Comparison,
            Xor => Or,
            And => Xor,
            Shift => And,
            Additive => Shift,
            Multiplicative => Additive,
            Unary => Multiplicative,
            ErrorPropagation => Unary,
            FunctionCall => ErrorPropagation,
            FieldExpression => FunctionCall,
            Path => FieldExpression,
        }
    }

    pub fn infix_precedence(tokens: &Stream<Token>) -> Self {
        match Self::infix_precedence_and_parser(tokens) {
            None => Self::Minimum,
            Some((prec, _)) => prec,
        }
    }

    fn infix_precedence_and_parser(tokens: &Stream<Token>) -> Option<(Self, &ParserFunction)> {
        use BasicToken::*;
        use Token::*;

        match tokens.peek() {
            Basic(Period) => Some((Self::FieldExpression, &field)),

            Basic(LParen) => Some((Self::FunctionCall, &call)),
            Basic(LBrack) => Some((Self::FunctionCall, &index)),

            Basic(Asterisk) | Basic(FSlash) => Some((Self::Multiplicative, &arithmetic_or_logical)),
            Basic(Plus) | Basic(Hyphen) => Some((Self::Additive, &arithmetic_or_logical)),
            Basic(LAngle2) | Basic(RAngle2) => Some((Self::Shift, &arithmetic_or_logical)),

            Basic(Ampersand) => Some((Self::And, &arithmetic_or_logical)),
            Basic(Caret) => Some((Self::Xor, &arithmetic_or_logical)),
            Basic(Bar) => Some((Self::Or, &arithmetic_or_logical)),

            Basic(Equal2) | Basic(BangEqual) | Basic(LAngle) | Basic(RAngle)
            | Basic(LAngleEqual) | Basic(RAngleEqual) => Some((Self::Comparison, &comparison)),

            Basic(Ampersand2) => Some((Self::LazyAnd, &lazy_boolean)),
            Basic(Bar2) => Some((Self::LazyOr, &lazy_boolean)),

            Basic(Equal)
            | Basic(PlusEqual)
            | Basic(AsteriskEqual)
            | Basic(HyphenEqual)
            | Basic(FSlashEqual)
            | Basic(AmpersandEqual)
            | Basic(BarEqual)
            | Basic(LAngle2Equal)
            | Basic(RAngle2Equal) => Some((Self::Assignment, &assignment)),

            Basic(Question) => Some((Self::ErrorPropagation, &error_propagation)),

            _ => None,
        }
    }
}

impl From<ArithmeticOrLogicalOperator> for Precedence {
    fn from(value: ArithmeticOrLogicalOperator) -> Self {
        use ArithmeticOrLogicalOperator::*;

        match value {
            Plus => Self::Additive,
            Minus => Self::Additive,
            Times => Self::Multiplicative,
            Divide => Self::Multiplicative,
            And => Self::And,
            Or => Self::Or,
            LShift => Self::Shift,
            RShift => Self::Shift,
        }
    }
}

pub fn peek_expression(tokens: &Stream<Token>) -> bool {
    use BasicToken::*;
    use KeywordToken::*;
    use LiteralToken::*;
    use Token::*;

    match tokens.peek() {
        Basic(LParen) => true,
        Basic(Hyphen) => true,
        Basic(Bang) => true,
        Basic(Colon2) => true,
        Basic(Ampersand) => true, // TODO: Not parsing borrow expressions yet
        Basic(Asterisk) => true,  // TODO: Not parsing dereference expressions yet
        Identifier(_) => true,
        Keyword(Return) => true,
        Keyword(Break) => true,
        Keyword(Continue) => true,
        Keyword(True) => true,
        Keyword(False) => true,
        Keyword(Unit) => true,
        Literal(Integer(_)) => true,
        Literal(Character(_)) => true,
        Literal(String(_)) => true,
        _ => false,
    }
}

pub fn expression(tokens: &mut Stream<Token>) -> ParseResult<Expression> {
    pratt(Precedence::Minimum, tokens)
}

pub fn block_expression(tokens: &mut Stream<Token>) -> ParseResult<Spanned<BlockExpression>> {
    // <block-expression> |= LBRACE <statements>? RBRACE
    //
    // <statements> |= <statement>+
    //              |  <statement>+ <expr-without-block>
    //              |  <expr-without-block>
    //
    // TODO: For now, just going to parse as LBRACE <statement>* RBRACE

    use Token::*;
    use BasicToken::*;

    let start = tokens.peek_for(LBrace, format!("Expected {LBrace} to begin block expression"))?;

    let mut statements = Vec::new();

    loop {
        match tokens.peek() {
            Basic(RBrace) => {
                let end = tokens.pop();
                break Ok(Spanned::new(statements, Span::between(start.span, end.span)))
            },
            _ => statements.push(parse::stmt::statement(tokens)?)
        }
    }
}

fn pratt(precedence: Precedence, tokens: &mut Stream<Token>) -> ParseResult<Expression> {
    let mut left = prefix(tokens)?;

    while precedence < Precedence::infix_precedence(tokens) {
        left = infix(left, tokens)?;
    }

    Ok(left)
}

fn prefix(tokens: &mut Stream<Token>) -> ParseResult<Expression> {
    use BasicToken::*;
    use KeywordToken as Kw;
    use LiteralToken as Lit;
    use Token::*;

    let maybe_parser: Option<&dyn Fn(&mut Stream<Token>) -> ParseResult<Expression>> = match tokens
        .peek()
    {
        Basic(LParen) => Some(&group),
        Basic(Hyphen) | Basic(Bang) => Some(&negate_operator),
        Basic(LBrack) => Some(&array),
        Identifier(_) | Basic(Colon2) => Some(&path),
        Keyword(Kw::Return) | Keyword(Kw::Break) | Keyword(Kw::Continue) => Some(&unconditional),
        Keyword(Kw::True)
        | Keyword(Kw::False)
        | Keyword(Kw::Unit)
        | Literal(Lit::Integer(_))
        | Literal(Lit::Character(_))
        | Literal(Lit::String(_)) => Some(&literal),
        _ => None,
    };

    match maybe_parser {
        None => Err(Error::new(
            tokens.peek_span(),
            format!(
                "Expected to find prefix expression, but found {} instead",
                tokens.peek()
            ),
        )),
        Some(parser) => parser(tokens),
    }
}

fn infix(left: Expression, tokens: &mut Stream<Token>) -> ParseResult<Expression> {
    // TODO: Idk why I need to clone here...
    match Precedence::infix_precedence_and_parser(&tokens.clone()) {
        None => Err(Error::new(
            tokens.peek_span(),
            format!(
                "Expected to find infix expression operator, but found {} instead",
                tokens.peek()
            ),
        )),
        Some((prec, parser)) => parser(prec, left, tokens),
    }
}

fn assignment(
    precedence: Precedence,
    left: Expression,
    tokens: &mut Stream<Token>,
) -> ParseResult<Expression> {
    let operator = match AssignmentOperator::try_from(tokens.peek()) {
        Err(()) => {
            return Err(Error::new(
                tokens.peek_span(),
                "Expected assignment operator".to_owned(),
            ))
        }
        Ok(op) => op,
    };

    let basic = BasicToken::from(operator.clone());
    tokens.peek_for(
        basic.clone(),
        format!("Expected to find {basic} as assignment operator"),
    )?;

    let right = pratt(precedence.right_associative(), tokens)?;

    let left_span = left.span.clone();
    let right_span = right.span.clone();

    Ok(Expression::new(
        ExpressionKind::WithoutBlock(ExpressionWithoutBlock::Assignment(
            Box::new(left),
            operator,
            Box::new(right),
        )),
        Span::between(left_span, right_span),
    ))
}

fn comparison(
    precedence: Precedence,
    left: Expression,
    tokens: &mut Stream<Token>,
) -> ParseResult<Expression> {
    let operator = match ComparisonOperator::try_from(tokens.peek()) {
        Err(()) => {
            return Err(Error::new(
                tokens.peek_span(),
                "Expected comparison operator".to_owned(),
            ))
        }
        Ok(op) => op,
    };

    let basic = BasicToken::from(operator.clone());
    tokens.peek_for(
        basic.clone(),
        format!("Expected to find {basic} as binary operator"),
    )?;

    let right = pratt(precedence, tokens)?;

    let left_span = left.span.clone();
    let right_span = right.span.clone();

    Ok(Expression::new(
        ExpressionKind::WithoutBlock(ExpressionWithoutBlock::Comparison(
            Box::new(left),
            operator,
            Box::new(right),
        )),
        Span::between(left_span, right_span),
    ))
}

fn lazy_boolean(
    precedence: Precedence,
    left: Expression,
    tokens: &mut Stream<Token>,
) -> ParseResult<Expression> {
    let operator = match LazyBooleanOperator::try_from(tokens.peek()) {
        Err(()) => {
            return Err(Error::new(
                tokens.peek_span(),
                "Expected lazy boolean operator".to_owned(),
            ))
        }
        Ok(op) => op,
    };

    let basic = BasicToken::from(operator.clone());
    tokens.peek_for(
        basic.clone(),
        format!("Expected to find {basic} as binary operator"),
    )?;

    let right = pratt(precedence, tokens)?;

    let left_span = left.span.clone();
    let right_span = right.span.clone();

    Ok(Expression::new(
        ExpressionKind::WithoutBlock(ExpressionWithoutBlock::LazyBoolean(
            Box::new(left),
            operator,
            Box::new(right),
        )),
        Span::between(left_span, right_span),
    ))
}

fn call(
    _precedence: Precedence,
    left: Expression,
    tokens: &mut Stream<Token>,
) -> ParseResult<Expression> {
    use BasicToken::*;

    let left_span = left.span.clone();

    tokens.peek_for(
        LParen,
        format!("Expected to find {LParen} to begin call argument list"),
    )?;

    let mut args = Vec::new();
    let mut expect_rparen = false;

    loop {
        match tokens.peek_for(RParen, String::from("")) {
            Ok(rparen) => {
                return Ok(Expression::new(
                    ExpressionKind::WithoutBlock(ExpressionWithoutBlock::Call(
                        Box::new(left),
                        args,
                    )),
                    Span::between(left_span, tokens.peek_span()),
                ))
            }
            Err(_) => {
                if expect_rparen {
                    return Err(Error::new(
                        Span::between(left_span, tokens.peek_span()),
                        format!("Expected to find {RParen} to end call argument list"),
                    ));
                }
            }
        }

        args.push(expression(tokens)?);

        match tokens.peek_for(Comma, String::from("")) {
            Ok(_) => { /* */ }
            Err(_) => expect_rparen = true,
        }
    }
}

fn index(
    _precedence: Precedence,
    left: Expression,
    tokens: &mut Stream<Token>,
) -> ParseResult<Expression> {
    use BasicToken::*;

    let left_span = left.span.clone();

    tokens.peek_for(
        LBrack,
        format!("Expected to find {LBrack} as part of index expression"),
    )?;

    let index = expression(tokens)?;

    let rbrack = tokens.peek_for(
        RBrack,
        format!("Expected to find {RBrack} as part of index expression"),
    )?;

    Ok(Expression::new(
        ExpressionKind::WithoutBlock(ExpressionWithoutBlock::Index(
            Box::new(left),
            Box::new(index),
        )),
        Span::between(left_span, rbrack.span),
    ))
}

fn field(
    _precedence: Precedence,
    left: Expression,
    tokens: &mut Stream<Token>,
) -> ParseResult<Expression> {
    use BasicToken::*;

    let left_span = left.span.clone();

    tokens.peek_for(
        Period,
        format!("Expected to find {Period} as part of field expression"),
    )?;
    let ident = tokens.peek_for(
        IdentifierToken,
        format!("Expected to find identifier as part of field expression"),
    )?;

    Ok(Expression::new(
        ExpressionKind::WithoutBlock(ExpressionWithoutBlock::Field(Box::new(left), ident)),
        Span::between(left_span, tokens.peek_span()),
    ))
}

fn arithmetic_or_logical(
    precedence: Precedence,
    left: Expression,
    tokens: &mut Stream<Token>,
) -> ParseResult<Expression> {
    let operator = match ArithmeticOrLogicalOperator::try_from(tokens.peek()) {
        Err(()) => {
            return Err(Error::new(
                tokens.peek_span(),
                "Expected arithmetic or logical binary operator".to_owned(),
            ))
        }
        Ok(op) => op,
    };

    let basic = BasicToken::from(operator.clone());
    tokens.peek_for(
        basic.clone(),
        format!("Expected to find {basic} as binary operator"),
    )?;

    let right = pratt(precedence, tokens)?;

    let left_span = left.span.clone();
    let right_span = right.span.clone();

    Ok(Expression::new(
        ExpressionKind::WithoutBlock(ExpressionWithoutBlock::ArithmeticOrLogical(
            Box::new(left),
            operator,
            Box::new(right),
        )),
        Span::between(left_span, right_span),
    ))
}

fn group(tokens: &mut Stream<Token>) -> ParseResult<Expression> {
    use BasicToken::*;

    let lparen = tokens.peek_for(
        LParen,
        format!("Expected {LParen} to open parenthesized expression"),
    )?;

    let expr = expression(tokens)?;
    let rparen = tokens.peek_for(
        RParen,
        format!("Expected {RParen} to close parenthesized expression"),
    )?;

    let expr_span = expr.span.clone();

    Ok(Expression::new(
        ExpressionKind::WithoutBlock(ExpressionWithoutBlock::Grouped(Box::new(expr))),
        Span::between(lparen.span, rparen.span),
    ))
}

fn path(tokens: &mut Stream<Token>) -> ParseResult<Expression> {
    let spanned_path = parse::path(tokens)?;

    Ok(Expression::new(
        ExpressionKind::WithoutBlock(ExpressionWithoutBlock::Path(spanned_path.item)),
        spanned_path.span,
    ))
}

fn error_propagation(
    precedence: Precedence,
    left: Expression,
    tokens: &mut Stream<Token>,
) -> ParseResult<Expression> {
    use BasicToken::*;

    let found = tokens.peek_for(
        Question,
        format!("Expected to find error propagation operator {Question}"),
    )?;

    let left_span = left.span;

    Ok(Expression::new(
        ExpressionKind::WithoutBlock(ExpressionWithoutBlock::ErrorPropagation(Box::new(left))),
        Span::between(left_span, found.span),
    ))
}

fn unconditional(tokens: &mut Stream<Token>) -> ParseResult<Expression> {
    use KeywordToken::*;

    let start = tokens.peek_span();

    match tokens.peek_for(Return, String::from("")) {
        Ok(_) => match peek_expression(tokens) {
            true => {
                let expr = pratt(Precedence::Return, tokens)?;
                return Ok(Expression::new(
                    ExpressionKind::WithoutBlock(ExpressionWithoutBlock::Return(Some(Box::new(
                        expr,
                    )))),
                    Span::between(start, tokens.peek_span()),
                ));
            }
            false => {
                return Ok(Expression::new(
                    ExpressionKind::WithoutBlock(ExpressionWithoutBlock::Return(None)),
                    Span::between(start, tokens.peek_span()),
                ))
            }
        },
        Err(_) => { /* Try next unconditional expressions */ }
    };

    match tokens.peek_for(Break, String::from("")) {
        Ok(_) => match peek_expression(tokens) {
            true => {
                let expr = pratt(Precedence::Return, tokens)?;
                return Ok(Expression::new(
                    ExpressionKind::WithoutBlock(ExpressionWithoutBlock::Break(Some(Box::new(
                        expr,
                    )))),
                    Span::between(start, tokens.peek_span()),
                ));
            }
            false => {
                return Ok(Expression::new(
                    ExpressionKind::WithoutBlock(ExpressionWithoutBlock::Break(None)),
                    Span::between(start, tokens.peek_span()),
                ));
            }
        },
        Err(_) => { /* Try next unconditional expressions */ }
    };

    tokens.peek_for(Continue, format!("Expected to find {Continue} expression"))?;

    Ok(Expression::new(
        ExpressionKind::WithoutBlock(ExpressionWithoutBlock::Continue),
        Span::between(start, tokens.peek_span()),
    ))
}

fn negate_operator(tokens: &mut Stream<Token>) -> ParseResult<Expression> {
    todo!()
}

fn array(tokens: &mut Stream<Token>) -> ParseResult<Expression> {
    use BasicToken::*;

    let start = tokens.peek_span();
    tokens.peek_for(
        LBrack,
        format!("Expected {LBrack} to begin array literal expression"),
    )?;

    let mut elements = Vec::new();
    let mut expect_rbrack = false;

    loop {
        match tokens.peek_for(RBrack, String::from("")) {
            Ok(rbrack) => {
                return Ok(Expression::new(
                    ExpressionKind::WithoutBlock(ExpressionWithoutBlock::Array(elements)),
                    Span::between(start, tokens.peek_span()),
                ));
            }
            Err(_) => {
                if expect_rbrack {
                    return Err(Error::new(
                        Span::between(start, tokens.peek_span()),
                        format!("Expected to find {RBrack} to end array literal expression"),
                    ));
                }
            }
        }

        elements.push(expression(tokens)?);

        match tokens.peek_for(Comma, String::from("")) {
            Ok(_) => { /* */ }
            Err(_) => expect_rbrack = true,
        }
    }
}

fn literal(tokens: &mut Stream<Token>) -> ParseResult<Expression> {
    use KeywordToken::*;
    use LiteralToken as Lit;
    use Token::*;

    match tokens.peek() {
        Literal(Lit::Integer(val)) => {
            let start = tokens.pop().span;
            Ok(Expression::new(
                ExpressionKind::WithoutBlock(ExpressionWithoutBlock::Integer(val)),
                Span::between(start, tokens.peek_span())
            ))
        },
        Literal(Lit::Character(ch)) => {
            let start = tokens.pop().span;
            Ok(Expression::new(
                ExpressionKind::WithoutBlock(ExpressionWithoutBlock::Character(ch)),
                Span::between(start, tokens.peek_span())
            ))
        },
        Literal(Lit::String(st)) => {
            let start = tokens.pop().span;
            Ok(Expression::new(
                ExpressionKind::WithoutBlock(ExpressionWithoutBlock::String(st)),
                Span::between(start, tokens.peek_span())
            ))
        },
        Keyword(True) => {
            let start = tokens.pop().span;
            Ok(
                Expression::new(
                    ExpressionKind::WithoutBlock(ExpressionWithoutBlock::True),
                    Span::between(start, tokens.peek_span())
                )
            )
        },
        Keyword(False) => {
            let start = tokens.pop().span;
            Ok(
                Expression::new(
                    ExpressionKind::WithoutBlock(ExpressionWithoutBlock::False),
                    Span::between(start, tokens.peek_span())
                )
            )
        },
        Keyword(Unit) => {
            let start = tokens.pop().span;
            Ok(
                Expression::new(
                    ExpressionKind::WithoutBlock(
                        ExpressionWithoutBlock::Unit,
                    ),
                    Span::between(start, tokens.peek_span())
                )
            )
        }
        otherwise => Err(Error::new(
            tokens.peek_span(),
            format!("Expected to find integer literal, character literal, or string literal but found {otherwise} instead")
        ))
    }
}
