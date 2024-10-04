use crate::stream::*;
use crate::token::*;

type LexResult<T> = Result<T, String>;

impl EndMarked for char {
    const END: char = '\0';
}

fn consume_hexadecimal_characters(stream: &mut Stream<char>) -> LexResult<OpalLiteral> {
    use Token::*;

    let mut chars: Vec<char> = Vec::new();

    match stream.peek() {
        '0'..='9' => chars.push(stream.pop()),
        'A'..='F' => chars.push(stream.pop()),
        'a'..='f' => chars.push(stream.pop()),
        _ => return Err("Failed to tokenize hexadecimal literal".to_string()),
    }

    loop {
        match stream.peek() {
            '0'..='9' => chars.push(stream.pop()),
            'A'..='F' => chars.push(stream.pop()),
            'a'..='f' => chars.push(stream.pop()),
            _ => {
                let maybe_int =
                    u32::from_str_radix(chars.into_iter().collect::<String>().as_str(), 16);
                match maybe_int {
                    Ok(int) => return Ok(OpalLiteral::Integer(int)),
                    Err(_) => return Err("Failed to tokenize hexadecimal literal".to_string()),
                };
            }
        }
    }
}

fn consume_binary_characters(stream: &mut Stream<char>) -> LexResult<OpalLiteral> {
    use Token::*;

    let mut chars: Vec<char> = Vec::new();

    match stream.peek() {
        '0'..='1' => chars.push(stream.pop()),
        _ => return Err("Failed to tokenize binary literal".to_string()),
    }

    loop {
        match stream.peek() {
            '0'..='1' => chars.push(stream.pop()),
            _ => {
                let maybe_int =
                    u32::from_str_radix(chars.into_iter().collect::<String>().as_str(), 2);
                match maybe_int {
                    Ok(int) => return Ok(OpalLiteral::Integer(int)),
                    Err(_) => return Err("Failed to tokenize binary literal".to_string()),
                };
            }
        }
    }
}

fn consume_decimal_characters(stream: &mut Stream<char>) -> LexResult<OpalLiteral> {
    use Token::*;

    let mut chars: Vec<char> = Vec::new();

    match stream.peek() {
        '1'..='9' => chars.push(stream.pop()),
        _ => return Err("Failed to tokenize decimal literal".to_string()),
    }

    loop {
        match stream.peek() {
            '0'..='9' => chars.push(stream.pop()),
            _ => {
                let maybe_int =
                    u32::from_str_radix(chars.into_iter().collect::<String>().as_str(), 10);
                match maybe_int {
                    Ok(int) => return Ok(OpalLiteral::Integer(int)),
                    Err(_) => return Err("Failed to tokenize decimal literal".to_string()),
                };
            }
        }
    }
}

fn tokenize_integer_literal(stream: &mut Stream<char>) -> LexResult<Token> {
    use OpalLiteral::*;
    use Token::*;

    match stream.peek() {
        '0' => {
            stream.pop();

            match stream.peek() {
                'x' => {
                    stream.pop();
                    Ok(Literal(consume_hexadecimal_characters(stream)?))
                }
                'b' => {
                    stream.pop();
                    Ok(Literal(consume_binary_characters(stream)?))
                }
                '0'..='9' => Err("Invalid integer literal".to_string()),
                _ => Ok(Literal(Integer(0))),
            }
        }
        '1'..='9' => Ok(Literal(consume_decimal_characters(stream)?)),
        _ => Err("Invalid integer literal".to_string()),
    }
}

fn tokenize_word(stream: &mut Stream<char>) -> LexResult<Token> {
    use OpalKeyword::*;
    use Token::*;

    let mut chars = Vec::new();

    match stream.peek() {
        'A'..='Z' | 'a'..='z' | '_' => chars.push(stream.pop()),
        otherwise => {
            return Err("Unexpected character while tokenizing identifier or keyword".to_string())
        }
    };

    let str: String = loop {
        match stream.peek() {
            'A'..='Z' | 'a'..='z' | '_' | '0'..='9' => chars.push(stream.pop()),
            _ => break chars.into_iter().collect(),
        }
    };

    match OpalKeyword::try_from(str.clone()) {
        Ok(kw) => Ok(Keyword(kw)),
        Err(_) => Ok(Identifier(str)),
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
                _ => Err("Illegal escape sequence".to_string()),
            }
        }
        _ => Err("Illegal escape sequence".to_string()),
    }
}

fn tokenize_char_literal(stream: &mut Stream<char>) -> LexResult<Token> {
    use OpalLiteral::*;
    use Token::*;

    match stream.peek() {
        '\'' => {
            stream.pop();

            let ch = match stream.peek() {
                '\\' => tokenize_escape_sequence(stream)?,
                '\'' => return Err("Illegal empty char literal".to_string()),
                otherwise => otherwise,
            };

            stream.pop();

            match stream.peek() {
                '\'' => {
                    stream.pop();
                    Ok(Literal(Character(ch)))
                }
                _ => Err("Expected single quote terminating character literal'".to_string()),
            }
        }
        _ => Err("Invalid character literal".to_string()),
    }
}

fn tokenize_string_literal(stream: &mut Stream<char>) -> LexResult<Token> {
    use OpalLiteral::*;
    use Token::*;

    match stream.peek() {
        '"' => {
            stream.pop();
            let mut chars = Vec::new();

            loop {
                let ch = match stream.peek() {
                    '\\' => {
                        stream.pop();
                        match stream.peek() {
                            '0' => '\0',
                            'n' => '\n',
                            't' => '\t',
                            '\\' => '\\',
                            '\'' => '\'',
                            '\"' => '\"',
                            _ => return Err("Illegal escape sequence".to_string()),
                        }
                    }
                    '"' => {
                        stream.pop();
                        return Ok(Literal(String(chars.into_iter().collect())));
                    }
                    otherwise => otherwise,
                };

                stream.pop();
                chars.push(ch);
            }
        }
        _ => Err("Invalid string literal".to_string()),
    }
}

fn tokenize_basic(stream: &mut Stream<char>) -> LexResult<Token> {
    use OpalBasic::*;
    use Token::*;

    let token = match stream.peek() {
        '{' => Basic(LBrace),
        '}' => Basic(RBrace),
        ',' => Basic(Comma),
        '(' => Basic(LParen),
        ')' => Basic(RParen),
        '-' => {
            stream.pop();
            match stream.peek() {
                '>' => Basic(LightRArrow),
                _ => {
                    return Err(
                        "Unexpected character while expecting '>' to complete '->'".to_string()
                    )
                }
            }
        }
        _ => return Err("Unexpected character".to_string()),
    };

    stream.pop();
    Ok(token)
}

pub fn tokenize(source: &str) -> LexResult<Stream<Token>> {
    let mut source = source.chars().collect::<Stream<_>>();
    let mut tokens = Vec::new();

    loop {
        match source.peek() {
            '0'..='9' => tokens.push(tokenize_integer_literal(&mut source)?),
            'a'..='z' => tokens.push(tokenize_word(&mut source)?),
            'A'..='Z' => tokens.push(tokenize_word(&mut source)?),
            '_' => tokens.push(tokenize_word(&mut source)?),
            '\'' => tokens.push(tokenize_char_literal(&mut source)?),
            '\"' => tokens.push(tokenize_string_literal(&mut source)?),
            '\0' => {
                tokens.push(Token::Eof);
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
