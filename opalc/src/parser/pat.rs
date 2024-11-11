use crate::ast::Pattern;
use crate::parser::precedence::Precedence;
use crate::parser::TokenStream;

impl Precedence<PatternPrecedence> for TokenStream {
    fn check_precedence(&self) -> PatternPrecedence {
        todo!()
    }
}

#[derive(PartialEq, PartialOrd)]
pub enum PatternPrecedence {
    Mininum,
}

pub fn parse_pattern(
    precedence: PatternPrecedence,
    tokens: &mut TokenStream,
) -> Result<Pattern, ()> {
    todo!()
}
