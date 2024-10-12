use crate::model::*;
use crate::parse::lex;
use crate::span::{Span, Spanned};

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
