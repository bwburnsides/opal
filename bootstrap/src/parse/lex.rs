use crate::error::*;
use crate::model::*;
use crate::span::*;
use crate::stream::*;

type LexResult<T> = Result<T, Error>;

impl EndMarked for char {
    const END: char = '\0';
}

fn consume_hexadecimal_characters(stream: &mut Stream<char>) -> LexResult<Spanned<LiteralToken>> {
    let mut chars: Vec<char> = Vec::new();
    let mut span;

    match stream.peek() {
        '0'..='9' | 'A'..='F' | 'a'..='f' => {
            let popped = stream.pop();
            chars.push(popped.item);
            span = popped.span;
        },
        otherwise => return Err(Error::with_details(
            stream.peek_span(),
            format!(
                "Expected hexadecimal character while tokenizing integer literal, but found {otherwise} instead"
            ),
            "Hexadecimal character matches the pattern [0-9A-Fa-f]".to_string(),
        )),
    }

    loop {
        match stream.peek() {
            '0'..='9' | 'A'..='F' | 'a'..='f' => {
                let popped = stream.pop();
                chars.push(popped.item);
                span = Span::between(span, popped.span);
            }
            otherwise => {
                let maybe_int =
                    u32::from_str_radix(chars.into_iter().collect::<String>().as_str(), 16);
                match maybe_int {
                    Ok(int) => return Ok(
                        Spanned::new(
                            LiteralToken::Integer(int),
                            span
                        )
                    ),
                    Err(e) => return Err(Error::with_details(
                        stream.peek_span(),
                        format!(
                            "Expected hexadecimal character while tokenizing integer literal, but found {otherwise} instead"
                        ),
                        format!("Hexadecimal character matches the pattern [0-9A-Fa-f]\nThe following error was produced: {e:?}"),
                    ))
                };
            }
        }
    }
}

fn consume_binary_characters(stream: &mut Stream<char>) -> LexResult<Spanned<LiteralToken>> {
    let mut chars: Vec<char> = Vec::new();
    let mut span;

    match stream.peek() {
        '0' | '1' => {
            let popped = stream.pop();
            chars.push(popped.item);
            span = popped.span;
        },
        otherwise => {
            return Err(Error::with_details(
                stream.peek_span(),
                format!(
                "Expected binary character while tokenizing integer literal, but found {otherwise} instead",
            ),
                "Binary character matches the pattern [0-1]".to_string(),
            ))
        }
    }

    loop {
        match stream.peek() {
            '0' | '1' => {
                let popped = stream.pop();
                chars.push(popped.item);
                span = popped.span;
            }
            otherwise => {
                let maybe_int =
                    u32::from_str_radix(chars.into_iter().collect::<String>().as_str(), 2);
                match maybe_int {
                    Ok(int) => return Ok(Spanned::new(
                        LiteralToken::Integer(int),
                        span
                    )),
                    Err(e) => return Err(Error::with_details(
                        stream.peek_span(),
                        format!(
                            "Expected binary character while tokenizing integer literal, but found {} instead", otherwise
                        ),
                        format!("Binary character matches the pattern [0-1]\nThe following error was produced: {:?}", e),
                    ))
                };
            }
        }
    }
}

fn consume_decimal_characters(stream: &mut Stream<char>) -> LexResult<Spanned<LiteralToken>> {
    let mut chars: Vec<char> = Vec::new();
    let mut span;

    match stream.peek() {
        '0'..='9' => {
            let popped = stream.pop();
            chars.push(popped.item);
            span = popped.span;
        }
        otherwise => {
            return Err(Error::with_details(
                stream.peek_span(),
                format!(
                "Expected decimal character while tokenizing integer literal, but found {} instead",
                otherwise
            ),
                "Decimal character matches the pattern [0-9]".to_string(),
            ))
        }
    }

    loop {
        match stream.peek() {
            '0'..='9' => {
                let popped = stream.pop();
                chars.push(popped.item);
                span = popped.span;
            }
            otherwise => {
                let maybe_int = chars
                    .into_iter()
                    .collect::<String>()
                    .as_str()
                    .parse::<u32>();
                match maybe_int {
                    Ok(int) => return Ok(
                        Spanned::new(
                            LiteralToken::Integer(int),
                            span,
                        )
                    ),
                    Err(e) => return Err(Error::with_details(
                        stream.peek_span(),
                        format!(
                            "Expected decimal character while tokenizing integer literal, but found {} instead", otherwise
                        ),
                        format!("Decimal character matches the pattern [0-9]\nThe following error was produced: {:?}", e),
                    ))
                };
            }
        }
    }
}

fn tokenize_integer_literal(stream: &mut Stream<char>) -> LexResult<Spanned<Token>> {
    use LiteralToken::*;
    use Token::*;

    match stream.peek() {
        '0' => {
            let start = stream.pop().span;

            match stream.peek() {
                'x' => {
                    stream.pop();
                    let spanned_lit = consume_hexadecimal_characters(stream)?;
                    let stop = stream.peek_span();

                    Ok(Spanned::new(Literal(spanned_lit.item), Span::between(start, spanned_lit.span)))
                }
                'b' => {
                    stream.pop();
                    let spanned_lit = consume_binary_characters(stream)?;
                    let stop = stream.peek_span();

                    Ok(Spanned::new(Literal(spanned_lit.item), Span::between(start, spanned_lit.span)))
                }
                '0'..='9' => Err(Error::with_details(
                    stream.peek_span(),
                    format!("Expected decimal integer literal `0`, hexadecimal literal, or binary literal, but found {} instead", stream.peek()),
                    "Decimal integer literal cannot begin with leading zero".to_string(),
                )),
                _ => Ok(Spanned::new(Literal(Integer(0)), Span::between(start, start))),
            }
        }
        '1'..='9' => {
            let start = stream.peek_span();
            let spanned_lit = consume_decimal_characters(stream)?;
            let stop = stream.peek_span();

            Ok(Spanned::new(
                Literal(spanned_lit.item),
                Span::between(start, spanned_lit.span),
            ))
        }
        otherwise => Err(Error::with_details(
            stream.peek_span(),
            format!("Expected integer literal, but found {} instead", otherwise),
            "Integer literal is either `0`, begins with `0x`, `0b`, or [1-9]".to_string(),
        )),
    }
}

fn tokenize_word(stream: &mut Stream<char>) -> LexResult<Spanned<Token>> {
    use Token::*;

    match stream.peek() {
        // Look for a valid initial character of an identifier or keyword.
        'A'..='Z' | 'a'..='z' | '_' => {
            let first_spanned = stream.pop();
            let mut spanned_chars = Vec::new();

            // Consume valid follow-on characters of an identifier or keyword into a string.
            loop {
                match stream.peek() {
                    'A'..='Z' | 'a'..='z' | '_' | '0'..='9' => spanned_chars.push(stream.pop()),
                    _ => {
                        // Build the span and the string.
                        let mut span = first_spanned.span;
                        let mut chars = Vec::from([first_spanned.item]);

                        for spanned in spanned_chars {
                            span = Span::between(span, spanned.span);
                            chars.push(spanned.item);
                        }

                        let str: String = chars.iter().collect();

                        match KeywordToken::try_from(str.clone()) {
                            Ok(kw) => break Ok(Spanned::new(Keyword(kw), span)),
                            Err(_) => break Ok(Spanned::new(Identifier(str), span)),
                        }
                    }
                }
            }
        },
        // If not found, then an error has occurred.
        otherwise => {
            Err(Error::new(
                stream.peek_span(),
                format!(
                    "Expected alphabetical character or underscore while tokenizing identifier or keyword, but found {} instead",
                    otherwise
                ),
            ))
        }
    }
}

fn tokenize_escape_sequence(stream: &mut Stream<char>) -> LexResult<char> {
    match stream.peek() {
        '\\' => {
            stream.pop();
            match stream.peek() {
                '0' => Ok('\0'),
                'n' => Ok('\n'),
                't' => Ok('\t'),
                '\\' => Ok('\\'),
                '\'' => Ok('\''),
                '\"' => Ok('\"'),
                otherwise => Err(Error::new(
                    stream.peek_span(),
                    format!("Illegal escape sequence '\\{}' encountered; legal escape sequences are '\\0', '\\n', '\\t', '\\\\', '\\'', '\\\"'", otherwise),
                )),
            }
        }
        otherwise => Err(Error::new(
            stream.peek_span(),
            format!(
                "Expected '\\' denoting the start of an escape sequence, but found {}",
                otherwise
            ),
        )),
    }
}

fn tokenize_char_literal(stream: &mut Stream<char>) -> LexResult<Spanned<Token>> {
    use LiteralToken::*;
    use Token::*;

    match stream.peek() {
        '\'' => {
            let start = stream.pop().span;

            let ch = match stream.peek() {
                '\\' => tokenize_escape_sequence(stream)?,
                '\'' => return Err(Error::with_details(
                    stream.peek_span(),
                    "Expected to find non-single quote character following opening-single quote, but found single quote instead.".to_string(),
                    "Empty character literals are illegal. If the literal is meant to contain a single quote, consider escaping it: `'\\''`".to_string(),
                )),
                otherwise => otherwise,
            };

            stream.pop();

            match stream.peek() {
                '\'' => {
                    let stop = stream.pop().span;
                    Ok(Spanned::new(Literal(Character(ch)), Span::between(start, stop)))
                }
                otherwise => Err(Error::with_details(
                    stream.peek_span(),
                    format!("Expected to find closing single quote character to complete character literal, but found {} instead", otherwise),
                    "Character literals must contain one character between opening and closing single quotes".to_string(),
                )),
            }
        }
        otherwise => Err(Error::new(
            stream.peek_span(),
            format!("Expected to find opening single quote to begin character literal, but found {} instead", otherwise),
        )),
    }
}

fn tokenize_string_literal(stream: &mut Stream<char>) -> LexResult<Spanned<Token>> {
    use LiteralToken::*;
    use Token::*;

    match stream.peek() {
        '"' => {
            let start_span = stream.pop().span;
            let mut chars = Vec::new();

            loop {
                let ch = match stream.peek() {
                    '\\' => tokenize_escape_sequence(stream)?,
                    '"' => {
                        let stop_span = stream.pop().span;
                        return Ok(Spanned::new(Literal(String(chars.into_iter().collect())), Span::between(start_span, stop_span)))
                    },
                    otherwise => otherwise,
                };

                stream.pop();
                chars.push(ch);
            }
        }
        otherwise => Err(Error::new(
            stream.peek_span(),
            format!("Expected to find opening double quote to begin string literal, but found {} instead", otherwise)
        )),
    }
}

fn tokenize_basic(stream: &mut Stream<char>) -> LexResult<Spanned<Token>> {
    use BasicToken::*;
    use Token::*;

    match stream.peek() {
        '{' => Ok(Spanned::new(Basic(LBrace), stream.pop().span)),
        '}' => Ok(Spanned::new(Basic(RBrace), stream.pop().span)),
        ',' => Ok(Spanned::new(Basic(Comma), stream.pop().span)),
        '.' => Ok(Spanned::new(Basic(Period), stream.pop().span)),
        '^' => Ok(Spanned::new(Basic(Caret), stream.pop().span)),
        '(' => Ok(Spanned::new(Basic(LParen), stream.pop().span)),
        ')' => Ok(Spanned::new(Basic(RParen), stream.pop().span)),
        '&' => {
            let start = stream.pop().span;
            match stream.peek() {
                '&' => {
                    let stop = stream.pop().span;
                    Ok(Spanned::new(Basic(Ampersand2), Span::between(start, stop)))
                }
                '=' => {
                    let stop = stream.pop().span;
                    Ok(Spanned::new(
                        Basic(AmpersandEqual),
                        Span::between(start, stop),
                    ))
                }
                _ => Ok(Spanned::new(
                    Basic(Ampersand),
                    Span::between(start, stream.peek_span()),
                )),
            }
        }
        '*' => {
            let start = stream.pop().span;
            match stream.peek() {
                '=' => {
                    let stop = stream.pop().span;
                    Ok(Spanned::new(
                        Basic(AsteriskEqual),
                        Span::between(start, stop),
                    ))
                }
                _ => Ok(Spanned::new(
                    Basic(Asterisk),
                    Span::between(start, stream.peek_span()),
                )),
            }
        }
        '!' => {
            let start = stream.pop().span;
            match stream.peek() {
                '=' => Ok(Spanned::new(Basic(BangEqual), stream.pop().span)),
                _ => Ok(Spanned::new(Basic(Bang), stream.peek_span())),
            }
        }
        '[' => Ok(Spanned::new(Basic(LBrack), stream.pop().span)),
        ']' => Ok(Spanned::new(Basic(RBrack), stream.pop().span)),
        '=' => {
            let start = stream.pop().span;
            match stream.peek() {
                '=' => {
                    let stop = stream.pop().span;
                    Ok(Spanned::new(Basic(Equal2), Span::between(start, stop)))
                }
                _ => Ok(Spanned::new(
                    Basic(Equal),
                    Span::between(start, stream.peek_span()),
                )),
            }
        }
        '|' => {
            let start = stream.pop().span;
            match stream.peek() {
                '|' => {
                    let stop = stream.pop().span;
                    Ok(Spanned::new(Basic(Bar2), Span::between(start, stop)))
                }
                '=' => {
                    let stop = stream.pop().span;
                    Ok(Spanned::new(Basic(BarEqual), Span::between(start, stop)))
                }
                _ => Ok(Spanned::new(
                    Basic(Bar),
                    Span::between(start, stream.peek_span()),
                )),
            }
        }
        '+' => {
            let start = stream.pop().span;
            match stream.peek() {
                '=' => {
                    let stop = stream.pop().span;
                    Ok(Spanned::new(Basic(PlusEqual), Span::between(start, stop)))
                }
                _ => Ok(Spanned::new(
                    Basic(Plus),
                    Span::between(start, stream.peek_span()),
                )),
            }
        }
        '-' => {
            let start = stream.pop().span;
            match stream.peek() {
                '>' => {
                    let stop = stream.pop().span;
                    Ok(Spanned::new(Basic(LightRArrow), Span::between(start, stop)))
                }
                '=' => {
                    let stop = stream.pop().span;
                    Ok(Spanned::new(Basic(HyphenEqual), Span::between(start, stop)))
                }
                _ => Ok(Spanned::new(
                    Basic(Hyphen),
                    Span::between(start, stream.peek_span()),
                )),
            }
        }
        ':' => {
            let start = stream.pop().span;
            match stream.peek() {
                ':' => {
                    let stop = stream.pop().span;
                    Ok(Spanned::new(Basic(Colon2), Span::between(start, stop)))
                }
                otherwise => Err(Error::new(
                    stream.peek_span(),
                    format!(
                        "Expected ':' while attempting to tokenize {} but found {}",
                        Colon2, otherwise
                    ),
                )),
            }
        }
        '/' => {
            let start = stream.pop().span;
            match stream.peek() {
                '=' => Ok(Spanned::new(Basic(FSlashEqual), stream.pop().span)),
                _ => Ok(Spanned::new(Basic(FSlash), stream.pop().span)),
            }
        }
        '<' => {
            let start = stream.pop().span;
            match stream.peek() {
                '<' => match stream.peek() {
                    '=' => {
                        let stop = stream.pop().span;
                        Ok(Spanned::new(
                            Basic(LAngle2Equal),
                            Span::between(start, stop),
                        ))
                    }
                    _ => {
                        let stop = stream.pop().span;
                        Ok(Spanned::new(Basic(LAngle2), Span::between(start, stop)))
                    }
                },
                '=' => Ok(Spanned::new(
                    Basic(LAngleEqual),
                    Span::between(start, stream.pop().span),
                )),
                _ => Ok(Spanned::new(
                    Basic(LAngle),
                    Span::between(start, stream.peek_span()),
                )),
            }
        }
        '>' => {
            let start = stream.pop().span;
            match stream.peek() {
                '>' => match stream.peek() {
                    '=' => {
                        let stop = stream.pop().span;
                        Ok(Spanned::new(
                            Basic(RAngle2Equal),
                            Span::between(start, stop),
                        ))
                    }
                    _ => {
                        let stop = stream.pop().span;
                        Ok(Spanned::new(Basic(RAngle2), Span::between(start, stop)))
                    }
                },
                '=' => Ok(Spanned::new(
                    Basic(RAngleEqual),
                    Span::between(start, stream.pop().span),
                )),
                _ => Ok(Spanned::new(
                    Basic(RAngle),
                    Span::between(start, stream.peek_span()),
                )),
            }
        }
        otherwise => Err(Error::new(
            stream.peek_span(),
            format!(
                "Unexpected character '{}' while tokenizing source",
                otherwise
            ),
        )),
    }
}

pub fn tokenize(source: &str) -> LexResult<Stream<Token>> {
    let mut source = Stream::from(source);
    let mut tokens: Vec<Spanned<Token>> = Vec::new();

    loop {
        match source.peek() {
            '0'..='9' => tokens.push(tokenize_integer_literal(&mut source)?),
            'a'..='z' | 'A'..='Z' | '_' => tokens.push(tokenize_word(&mut source)?),
            '\'' => tokens.push(tokenize_char_literal(&mut source)?),
            '\"' => tokens.push(tokenize_string_literal(&mut source)?),
            '\0' => {
                tokens.push(Spanned::new(Token::Eof, source.end_span()));
                break Ok(tokens.into_iter().collect());
            }
            ' ' | '\n' => {
                source.pop();
                continue;
            }
            _ => tokens.push(tokenize_basic(&mut source)?),
        };
    }
}
