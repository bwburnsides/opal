mod expr;
mod item;
mod lex;
mod stmt;

#[cfg(test)]
mod test;

pub use crate::parse::expr::*;
pub use crate::parse::item::item;
pub use crate::parse::lex::*;

use crate::error::Error;
use crate::model::*;
use crate::span::{Span, Spanned};
use crate::stream::{PeekFor, Stream};

pub type ParseResult<T> = Result<T, Error>;

impl PeekFor<BasicToken, ParseResult<Spanned<Token>>> for Stream<Token> {
    fn peek_for(&mut self, kind: BasicToken, error_message: String) -> ParseResult<Spanned<Token>> {
        match self.peek() {
            Token::Basic(basic) if basic == kind => Ok(self.pop()),
            otherwise => Err(Error::new(
                self.peek_span(),
                format!("{error_message}, but found {otherwise} instead"),
            )),
        }
    }
}

impl PeekFor<KeywordToken, ParseResult<Spanned<Token>>> for Stream<Token> {
    fn peek_for(
        &mut self,
        kind: KeywordToken,
        error_message: String,
    ) -> ParseResult<Spanned<Token>> {
        match self.peek() {
            Token::Keyword(kw) if kw == kind => Ok(self.pop()),
            otherwise => Err(Error::new(
                self.peek_span(),
                format!("{error_message}, but found {otherwise} instead"),
            )),
        }
    }
}

impl PeekFor<IdentifierToken, ParseResult<Identifier>> for Stream<Token> {
    fn peek_for(
        &mut self,
        _kind: IdentifierToken,
        error_message: String,
    ) -> ParseResult<Identifier> {
        match self.peek() {
            Token::Identifier(name) => {
                let spanned = self.pop();
                Ok(Identifier::new(name, spanned.span))
            }
            otherwise => Err(Error::new(
                self.peek_span(),
                format!("{error_message}, but found {otherwise} instead"),
            )),
        }
    }
}

impl PeekFor<IntegerLiteralToken, ParseResult<u32>> for Stream<Token> {
    fn peek_for(&mut self, _kind: IntegerLiteralToken, error_message: String) -> ParseResult<u32> {
        match self.peek() {
            Token::Literal(LiteralToken::Integer(val)) => {
                let spanned = self.pop();
                Ok(val)
            }
            otherwise => Err(Error::new(
                self.peek_span(),
                format!("{error_message}, but found {otherwise} instead"),
            )),
        }
    }
}

pub fn geode(name: String, tokens: &mut Stream<Token>) -> ParseResult<Geode> {
    let mut items = Vec::new();

    loop {
        match tokens.peek() {
            Token::Eof => break Ok(Geode::new(name, items)),
            _ => items.push(item(tokens)?),
        }
    }
}

pub fn path(tokens: &mut Stream<Token>) -> ParseResult<Spanned<Path>> {
    let start = tokens.peek_span();

    let is_global = tokens
        .peek_for(BasicToken::Colon2, String::from(""))
        .is_ok();

    let mut segments = Vec::new();

    loop {
        match tokens.peek_for(IdentifierToken, String::from("")) {
            Ok(ident) => segments.push(ident),
            Err(_) => {
                return Err(Error::new(
                    tokens.peek_span(),
                    String::from("Expected identifier while parsing path expression"),
                ))
            }
        };

        match tokens.peek_for(BasicToken::Colon2, String::from("")) {
            Ok(_) => { /* */ }
            Err(_) => break,
        };
    }

    Ok(Spanned::new(
        Path::new(is_global, segments),
        Span::between(start, tokens.peek_span()),
    ))
}
