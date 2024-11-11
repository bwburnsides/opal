pub mod expr;
pub mod pat;
pub mod precedence;
pub mod stream;

use pat::parse_pattern;
use pat::PatternPrecedence;

use crate::ast::*;
use crate::lexer::*;
use crate::parser::expr::{parse_expression, OperatorPrecedence};
use crate::span::*;
use crate::parser::stream::TokenStream;

impl TokenStream {
    pub fn expect(&mut self, token: Token) -> Result<Token, ()> {
        match self.pop() {
            tok if tok == token => Ok(tok),
            _ => Err(()),
        }
    }

    pub fn expect_name(&mut self) -> Result<String, ()> {
        match self.pop() {
            Token::Name(name) => Ok(name),
            _ => Err(()),
        }
    }

    pub fn did_pop(&mut self, token: Token) -> bool {
        match self.peek() {
            tok if *tok == token => {
                self.pop();
                true
            }
            _ => false,
        }
    }

    // Check for the provided token. Then:
    //  1. If present, pop the token, then call provided parser
    //  2. If not present, return None
    pub fn parse_if_token<T, P>(&mut self, token: Token, parser: P) -> Option<Result<T, ()>>
    where
        P: FnOnce(&mut Self) -> Result<T, ()>,
    {
        if *self.peek() == token {
            self.pop();
            Some(parser(self))
        } else {
            None
        }
    }
}

fn parse_geode() {}

fn parse_item(tokens: &mut TokenStream) -> Result<UntypedItem, ()> {
    use Token::*;

    match tokens.peek() {
        Fn => Ok(Item::Function(parse_function(tokens)?)),
        Type => parse_type_alias_or_algebraic_type(tokens),
        Const => Ok(Item::Constant(parse_constant(tokens)?)),
        Static => Ok(Item::Static(parse_static(tokens)?)),
        _ => Err(()),
    }
}

fn parse_function(tokens: &mut TokenStream) -> Result<UntypedFunction, ()> {
    use Token::*;

    let _start = tokens.expect(Fn)?;
    let _name = tokens.expect_name()?;
    tokens.expect(LeftParen)?;

    let params = parse_parameters(tokens)?;

    tokens.expect(RightParen)?;

    let maybe_ret_type = match tokens.parse_if_token(RArrow, parse_type) {
        Some(ty) => Some(ty?),
        None => None,
    };

    let body = parse_block(tokens)?;
    tokens.expect(NewLine)?;

    Ok(Function::new(
        Span(0, 0),
        Spanned(_name, Span(0, 0)),
        params,
        body,
        maybe_ret_type,
        (),
    ))
}

fn parse_parameters(tokens: &mut TokenStream) -> Result<Vec<UntypedParameter>, ()> {
    // Comma separated list of zero or more parameters, allows trailing comma
    // [anon] [mut] name: type
    use Token::*;

    let mut params = Vec::new();

    match tokens.peek() {
        Anon | Mut | Name(_) => {
            params.push(parse_parameter(tokens)?);
        }
        _ => return Ok(params),
    };

    loop {
        if !tokens.did_pop(Comma) {
            break Ok(params);
        }

        match tokens.peek() {
            Anon | Mut | Name(_) => params.push(parse_parameter(tokens)?),
            _ => break Ok(params),
        }
    }
}

fn parse_parameter(tokens: &mut TokenStream) -> Result<UntypedParameter, ()> {
    // [anon] [mut] name: type
    use Token::*;

    let anon = if tokens.did_pop(Anon) {
        Anonymity::Anonymous
    } else {
        Anonymity::Named
    };

    let muty = if tokens.did_pop(Mut) {
        Mutability::Mutable
    } else {
        Mutability::Immutable
    };

    let name = tokens.expect_name()?;
    tokens.expect(Colon)?;
    let ty = parse_type(tokens)?;

    Ok(Parameter::new(
        Span(0, 0),
        Spanned(name, Span(0, 0)),
        ty,
        anon,
        muty,
        (),
    ))
}

fn parse_type_alias_or_algebraic_type(tokens: &mut TokenStream) -> Result<UntypedItem, ()> {
    use Token::*;

    let _start = tokens.expect(Type)?;
    let name = tokens.expect_name()?;

    match tokens.pop() {
        Equal => {
            let ty = parse_type(tokens)?;
            tokens.expect(NewLine)?;
            Ok(Item::TypeAlias(TypeAlias::new(
                Span(0, 0),
                Spanned(name, Span(0, 0)),
                ty,
                (),
            )))
        }
        LeftBrace => match tokens.pop() {
            RightBrace => Ok(Item::DataType(AlgebraicType::new(
                Span(0, 0),
                Spanned(name, Span(0, 0)),
                AlgebraicTypeKind::Sum(vec![]),
                (),
            ))),
            NewLine => Ok(Item::DataType(AlgebraicType::new(
                Span(0, 0),
                Spanned(name, Span(0, 0)),
                AlgebraicTypeKind::Product(vec![]),
                (),
            ))),
            Name(_field_or_var_name) => match tokens.pop() {
                Colon => {
                    let _first_field_type = parse_type(tokens)?;
                    todo!("Struct")
                }
                Comma => todo!("Enum"),
                LeftParen => todo!("Enum"),
                LeftBrace => todo!("Enum"),
                _ => Err(()),
            },
            _ => Err(()),
        },
        _ => Err(()),
    }
}

fn parse_constant(tokens: &mut TokenStream) -> Result<UntypedConstant, ()> {
    use Token::*;

    let _start = tokens.expect(Const)?;
    let name = tokens.expect_name()?;
    tokens.expect(Colon)?;
    let ty = parse_type(tokens)?;
    tokens.expect(Equal)?;
    let expr = parse_expression(OperatorPrecedence::Minimum, tokens)?;
    tokens.expect(NewLine)?;

    Ok(Constant::new(
        Span(0, 0),
        Spanned(name, Span(0, 0)),
        ty,
        expr,
        (),
    ))
}

fn parse_static(tokens: &mut TokenStream) -> Result<UntypedStatic, ()> {
    let _start = tokens.expect(Token::Static)?;
    let name = tokens.expect_name()?;
    tokens.expect(Token::Colon)?;
    let ty = parse_type(tokens)?;
    tokens.expect(Token::Equal)?;
    let expr = parse_expression(OperatorPrecedence::Minimum, tokens)?;
    tokens.expect(Token::NewLine)?;

    Ok(Static::new(
        Span(0, 0),
        Spanned(name, Span(0, 0)),
        ty,
        expr,
        (),
    ))
}

fn parse_type(tokens: &mut TokenStream) -> Result<Ty, ()> {
    use Token::*;

    match tokens.pop() {
        Name(_) => todo!("Named type"),
        LeftSquare => todo!("Array type"),
        Amper => todo!("Reference"),
        LeftParen => todo!("Group or unit"),
        Bang => todo!("Never"),
        _ => Err(()),
    }
}

fn parse_block(tokens: &mut TokenStream) -> Result<Vec<UntypedStatement>, ()> {
    tokens.expect(Token::LeftBrace)?;

    let mut stmts = Vec::new();
    loop {
        if tokens.did_pop(Token::RightBrace) {
            break Ok(stmts);
        } else {
            stmts.push(parse_statement(tokens)?)
        }
    }
}

fn parse_statement(tokens: &mut TokenStream) -> Result<UntypedStatement, ()> {
    use Token::*;

    match tokens.peek() {
        Const => Ok(Statement::Constant(parse_constant(tokens)?)),
        Let => parse_let(tokens),
        _ => Ok(Statement::Expression(parse_expression(
            OperatorPrecedence::Minimum,
            tokens,
        )?)),
    }
}

fn parse_let(tokens: &mut TokenStream) -> Result<UntypedStatement, ()> {
    tokens.expect(Token::Let)?;

    let mutability = if tokens.did_pop(Token::Mut) {
        Mutability::Mutable
    } else {
        Mutability::Immutable
    };

    let name = tokens.expect_name()?;

    let annotation = if tokens.did_pop(Token::Colon) {
        Some(parse_type(tokens)?)
    } else {
        None
    };

    let initializer = if tokens.did_pop(Token::Equal) {
        Some(parse_expression(OperatorPrecedence::Minimum, tokens)?)
    } else {
        None
    };

    Ok(Statement::Let(Let::new(
        Span(0, 0),
        Spanned(name, Span(0, 0)),
        annotation,
        initializer,
        mutability,
        (),
    )))
}

fn parse_field(tokens: &mut TokenStream) -> Result<UntypedField, ()> {
    let name = tokens.expect_name()?;
    tokens.expect(Token::Colon)?;
    let ty = parse_type(tokens)?;

    Ok(Field::new(Span(0, 0), Spanned(name, Span(0, 0)), ty, ()))
}

fn parse_variant(tokens: &mut TokenStream) -> Result<UntypedVariant, ()> {
    let name = tokens.expect_name()?;

    match tokens.peek() {
        Token::LeftParen => {
            tokens.pop();
            todo!("Tuple variant");
        }
        Token::LeftBrace => todo!("Struct variant"),
        _ => Ok(Variant::Unit(Spanned(name, Span(0, 0)))),
    }
}

fn parse_arm(tokens: &mut TokenStream) -> Result<UntypedArm, ()> {
    let pat = parse_pattern(PatternPrecedence::Mininum, tokens)?;
    tokens.expect(Token::RArrow)?;
    let expr = parse_expression(OperatorPrecedence::Minimum, tokens)?;
    Ok(UntypedArm::new(pat, expr))
}

fn parse_argument(tokens: &mut TokenStream) -> Result<UntypedArgument, ()> {
    // [name: ] expr
    match tokens.pop() {
        Token::Name(name) => {
            match tokens.peek() {
                Token::Colon => {
                    tokens.pop();
                    let arg = parse_expression(OperatorPrecedence::Minimum, tokens)?;
                    Ok(Argument::new(
                        Span(0, 0),
                        Some(Spanned(name, Span(0, 0))),
                        arg,
                        (),
                    ))
                }
                _ => Ok(Argument::new(
                    Span(0, 0),
                    None,
                    Expression::new(Span(0, 0), ExpressionKind::Name { name, extra: () }),
                    (),
                )),
            }
        }
        _ => {
            let arg = parse_expression(OperatorPrecedence::Minimum, tokens)?;
            Ok(Argument::new(Span(0, 0), None, arg, ()))
        }
    }
}
