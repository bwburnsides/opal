use crate::model::*;
use crate::parse::expression;
use crate::parse::item::type_repr;
use crate::parse::ParseResult;
use crate::span::Span;
use crate::stream::PeekFor;
use crate::stream::Stream;

pub fn statement(tokens: &mut Stream<Token>) -> ParseResult<Statement> {
    use KeywordToken::*;
    use Token::*;

    match tokens.peek() {
        Keyword(Let) => let_statement(tokens).map(Statement::Let),

        _ => todo!(),
    }
}

pub fn let_statement(tokens: &mut Stream<Token>) -> ParseResult<Let> {
    // Let Statements are surprisingly syntactically complex...
    //
    // let mut? foo [: Type]? [= init]? ;

    use BasicToken::*;
    use KeywordToken as Kw;

    let start = tokens.peek_for(Kw::Let, format!("Expected to find {}", Kw::Let))?;
    let mutability = match tokens.peek_for(Kw::Mut, String::from("")) {
        Ok(_) => Mutability::Mutable,
        Err(_) => Mutability::Immutable,
    };

    let name = tokens.peek_for(IdentifierToken, String::from("Expected identifier"))?;

    // Optionally ": Type"
    let maybe_type = match tokens.peek_for(Colon, String::from("")) {
        Ok(_) => Some(type_repr(tokens)?),
        Err(_) => None, // There is no type annotation. The type will be inferred...
    };

    // Optionally "= init"
    let maybe_init = match tokens.peek_for(Equal, String::from("")) {
        Ok(_) => Some(expression(tokens)?),
        Err(_) => None, // No initializer. Eventually we'll have to verify that the variable is initialized before use.
    };

    let end = tokens.peek_for(
        Semicolon,
        format!(
            "Expected {Semicolon} following {} declaration of {}",
            Kw::Let,
            name.item
        ),
    )?;

    Ok(Let::new(
        name,
        mutability,
        maybe_type,
        maybe_init,
        Span::between(start.span, end.span),
    ))
}
