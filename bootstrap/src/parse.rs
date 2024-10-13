mod expr;
mod item;
mod lex;

#[cfg(test)]
mod test;

pub use crate::parse::expr::*;
pub use crate::parse::lex::*;

use crate::error::Error;
use crate::model::*;
use crate::span::Spanned;
use crate::stream::{PeekFor, Stream};

type ParseResult<T> = Result<T, Error>;

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
