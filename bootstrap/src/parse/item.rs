use crate::model::*;
use crate::parse;
use crate::parse::*;
use crate::span::Span;
use crate::stream::Stream;

pub fn item(tokens: &mut Stream<Token>) -> ParseResult<Item> {
    use KeywordToken::*;
    use Token::*;

    match tokens.peek() {
        Keyword(Fn) => function_item(tokens).map(Item::Function),
        Keyword(Type) => type_alias_item(tokens).map(Item::TypeAlias),
        Keyword(Struct) => struct_item(tokens).map(Item::Struct),
        Keyword(Enum) => enum_item(tokens).map(Item::Enum),
        Keyword(Const) => const_item(tokens).map(Item::Const),
        Keyword(Static) => static_item(tokens).map(Item::Static),
        _ => Err(Error::new(
            tokens.peek_span(),
            format!("Expected to find item"),
        )),
    }
}

fn function_item(tokens: &mut Stream<Token>) -> ParseResult<FunctionItem> {
    use BasicToken::*;
    use KeywordToken::*;

    let start = tokens.peek_for(
        Fn,
        format!("Expected to find function item beginning with {Fn}"),
    )?;

    let function_name = tokens.peek_for(
        IdentifierToken,
        format!("Expected to find function item identifier following {Fn}"),
    )?;

    tokens.peek_for(
        LParen,
        format!(
            "Expected to find {LParen} to begin function item {}'s parameter list",
            function_name.item
        ),
    )?;

    let mut parameters = Vec::new();
    let mut expect_rparen = false;

    loop {
        match tokens.peek_for(RParen, String::from("")) {
            Ok(_) => break,
            _ => {
                if expect_rparen {
                    return Err(
                        Error::new(
                            tokens.peek_span(),
                            format!("Expected to find {RParen} to conclude function item {}'s parameter list", function_name.item)
                        )
                    );
                }
            }
        }

        let param = parameter(tokens)?;
        parameters.push(param);

        match tokens.peek_for(Comma, String::from("")) {
            Ok(_) => { /* */ }
            Err(_) => expect_rparen = true,
        }
    }

    let return_type = match tokens.peek() {
        Token::Basic(LightRArrow) => {
            tokens.pop();
            Some(type_repr(tokens)?)
        }
        _ => None,
    };

    let body = block_expression(tokens)?;

    Ok(FunctionItem::new(
        function_name,
        parameters,
        return_type,
        body.item,
        Span::between(start.span, body.span),
    ))
}

fn type_alias_item(tokens: &mut Stream<Token>) -> ParseResult<TypeAliasItem> {
    use BasicToken::*;
    use KeywordToken::*;

    let start = tokens.peek_for(
        Type,
        format!("Expected to find type alias item beginning with {Type}"),
    )?;

    let name = tokens.peek_for(
        IdentifierToken,
        format!("Expected to find type alias identifier following {Type}"),
    )?;

    tokens.peek_for(
        Equal,
        format!(
            "Expected to find {Equal} following type alias identifier {}",
            name.item
        ),
    )?;

    let ty = type_repr(tokens)?;

    let end = tokens.peek_for(
        Semicolon,
        format!(
            "Expected to find {Semicolon} to conclude type alias {} item",
            name.item,
        ),
    )?;

    Ok(TypeAliasItem::new(
        name,
        ty,
        Span::between(start.span, end.span),
    ))
}

fn struct_item(tokens: &mut Stream<Token>) -> ParseResult<StructItem> {
    use BasicToken::*;
    use KeywordToken::*;

    let start = tokens.peek_for(
        Struct,
        format!("Expected to find struct item beginning with {Struct}"),
    )?;

    let name = tokens.peek_for(
        IdentifierToken,
        format!("Expected to find struct item identifier following {Struct}"),
    )?;

    tokens.peek_for(
        LBrace,
        format!(
            "Expected to find {LBrace} following struct item identifier {}",
            name.item
        ),
    )?;

    let mut fields = Vec::new();

    loop {
        if let Token::Identifier(_) = tokens.peek() {
            let fd = field(tokens)?;
            fields.push(fd);
    
            match tokens.peek_for(Comma, String::from("")) {
                Ok(_) => { /* */ }
                Err(_) => break,
            }
        } else {
            break
        }
    }

    let end = tokens.peek_for(
        RBrace,
        format!(
            "Expected to find {RBrace} to conclude struct item `{}`",
            name.item
        ),
    )?;

    Ok(StructItem::new(
        name,
        fields,
        Span::between(start.span, end.span),
    ))
}

fn enum_item(tokens: &mut Stream<Token>) -> ParseResult<EnumItem> {
    use BasicToken::*;
    use KeywordToken::*;

    let start = tokens.peek_for(
        Enum,
        format!("Expected to find enum item beginning with {Enum}"),
    )?;

    let name = tokens.peek_for(
        IdentifierToken,
        format!("Expected to find enum item identifier following {Enum}"),
    )?;

    tokens.peek_for(
        LBrace,
        format!(
            "Expected to find {LBrace} following enum item identifier {}",
            name.item
        ),
    )?;

    let mut variants = Vec::new();

    loop {
        if let Token::Identifier(_) = tokens.peek() {
            let vt = variant(tokens)?;
            variants.push(vt);
    
            match tokens.peek_for(Comma, String::from("")) {
                Ok(_) => { /* */ }
                Err(_) => break,
            }
        } else {
            break
        }
    }

    let end = tokens.peek_for(
        RBrace,
        format!(
            "Expected to find {RBrace} to conclude enum item `{}`",
            name.item
        ),
    )?;

    Ok(EnumItem::new(
        name,
        variants,
        Span::between(start.span, end.span),
    ))
}

fn const_item(tokens: &mut Stream<Token>) -> ParseResult<ConstItem> {
    use BasicToken::*;
    use KeywordToken::*;

    let start = tokens.peek_for(
        Const,
        format!("Expected to find const item beginning with {Const}"),
    )?;

    let name = tokens.peek_for(
        IdentifierToken,
        format!("Expected to find const item identifier following {Type}"),
    )?;

    tokens.peek_for(
        Colon,
        format!(
            "Expected to find {Colon} following const item identifier {}",
            name.item
        ),
    )?;

    let ty = type_repr(tokens)?;

    tokens.peek_for(
        Equal,
        format!(
            "Expected to find {Equal} following const item {}'s type annotation",
            name.item
        ),
    )?;

    let value = expression(tokens)?;

    let end = tokens.peek_for(
        Semicolon,
        format!(
            "Expected to find {Semicolon} to conclude const item {}",
            name.item
        ),
    )?;

    Ok(ConstItem::new(
        name,
        ty,
        value,
        Span::between(start.span, end.span),
    ))
}

fn static_item(tokens: &mut Stream<Token>) -> ParseResult<StaticItem> {
    use BasicToken::*;
    use KeywordToken::*;

    let start = tokens.peek_for(
        Static,
        format!("Expected to find static item beginning with {Const}"),
    )?;

    let name = tokens.peek_for(
        IdentifierToken,
        format!("Expected to find static item identifier following {Type}"),
    )?;

    tokens.peek_for(
        Colon,
        format!(
            "Expected to find {Colon} following static item identifier {}",
            name.item
        ),
    )?;

    let ty = type_repr(tokens)?;

    tokens.peek_for(
        Equal,
        format!(
            "Expected to find {Equal} following static item {}'s type annotation",
            name.item
        ),
    )?;

    let value = expression(tokens)?;

    let end = tokens.peek_for(
        Semicolon,
        format!(
            "Expected to find {Semicolon} to conclude static item {}",
            name.item
        ),
    )?;

    Ok(StaticItem::new(
        name,
        ty,
        value,
        Span::between(start.span, end.span),
    ))
}

pub fn type_repr(tokens: &mut Stream<Token>) -> ParseResult<TypeRepr> {
    use BasicToken::*;
    use KeywordToken::*;
    use Token::*;

    match tokens.peek() {
        Keyword(U8) => Ok(TypeRepr::new(TypeReprKind::U8, tokens.pop().span)),
        Keyword(I8) => Ok(TypeRepr::new(TypeReprKind::I8, tokens.pop().span)),
        Keyword(U16) => Ok(TypeRepr::new(TypeReprKind::U16, tokens.pop().span)),
        Keyword(I16) => Ok(TypeRepr::new(TypeReprKind::I16, tokens.pop().span)),
        Keyword(U32) => Ok(TypeRepr::new(TypeReprKind::U32, tokens.pop().span)),
        Keyword(I32) => Ok(TypeRepr::new(TypeReprKind::I32, tokens.pop().span)),
        Keyword(Bool) => Ok(TypeRepr::new(TypeReprKind::Bool, tokens.pop().span)),
        Keyword(Char) => Ok(TypeRepr::new(TypeReprKind::Char, tokens.pop().span)),
        Keyword(Str) => Ok(TypeRepr::new(TypeReprKind::Str, tokens.pop().span)),
        Keyword(Unit) => Ok(TypeRepr::new(TypeReprKind::Unit, tokens.pop().span)),
        Basic(LBrack) => {
            let start = tokens.pop();
            let element_type = type_repr(tokens)?;
            tokens.peek_for(
                Semicolon,
                format!("Expected to find {Semicolon} following array type literal's element type"),
            )?;
            let size = tokens.peek_for(
                IntegerLiteralToken,
                format!("Expected to find array type literal's span"),
            )?;
            let end = tokens.peek_for(
                RBrack,
                format!("Expected to find {RBrack} to conclude array type literal"),
            )?;

            Ok(TypeRepr::new(
                TypeReprKind::Array(Box::new(element_type), size),
                Span::between(start.span, end.span),
            ))
        }
        Basic(Ampersand) => {
            let start = tokens.pop();
            let mutability = match tokens.peek_for(Mut, String::from("")) {
                Ok(_) => Mutability::Mutable,
                Err(_) => Mutability::Immutable,
            };
            let ty = type_repr(tokens)?;
            let ty_span = ty.span.clone();

            Ok(TypeRepr::new(
                TypeReprKind::Reference(mutability, Box::new(ty)),
                Span::between(start.span, ty_span),
            ))
        }
        Basic(LParen) => {
            let start = tokens.pop();
            let ty = type_repr(tokens)?;
            let end = tokens.peek_for(
                RParen,
                format!("Expected to find {RParen} to conclude parenthesized type literal"),
            )?;

            Ok(TypeRepr::new(
                TypeReprKind::Parenthesized(Box::new(ty)),
                Span::between(start.span, end.span),
            ))
        }
        Basic(Colon2) | Identifier(_) => {
            let spanned_path = parse::path(tokens)?;
            Ok(TypeRepr::new(
                TypeReprKind::Path(spanned_path.item),
                spanned_path.span,
            ))
        }
        otherwise => Err(Error::new(
            tokens.peek_span(),
            format!("Expected to find type literal, but found {otherwise} instead"),
        )),
    }
}

fn parameter(tokens: &mut Stream<Token>) -> ParseResult<Parameter> {
    use BasicToken::*;
    use KeywordToken::*;

    let (mutability, maybe_start) = match tokens.peek_for(Mut, String::from("")) {
        Ok(tok) => (Mutability::Mutable, Some(tok.span)),
        Err(_) => (Mutability::Immutable, None),
    };

    let name = tokens.peek_for(IdentifierToken, format!("Expected to find parameter name"))?;
    let name_span = name.span.clone();

    tokens.peek_for(
        Colon,
        format!(
            "Expected to find {Colon} following parameter named {}",
            name.item
        ),
    )?;

    let ty = type_repr(tokens)?;
    let ty_span = ty.span.clone();

    Ok(Parameter::new(
        mutability,
        name,
        ty,
        Span::between(maybe_start.unwrap_or(name_span), ty_span),
    ))
}

fn field(tokens: &mut Stream<Token>) -> ParseResult<Field> {
    use BasicToken::Colon;

    let name = tokens.peek_for(IdentifierToken, format!("Expected field name"))?;
    let name_span = name.span.clone();

    tokens.peek_for(
        Colon,
        format!("Expected to find {Colon} following {}", name.item),
    )?;

    let ty = type_repr(tokens)?;
    let ty_span = ty.span.clone();

    Ok(Field::new(name, ty, Span::between(name_span, ty_span)))
}

fn variant(tokens: &mut Stream<Token>) -> ParseResult<Variant> {
    use BasicToken::*;
    use Token::*;

    let name = tokens.peek_for(
        IdentifierToken,
        format!("Expected to find variant identifier"),
    )?;

    match tokens.peek() {
        Basic(LParen) => {
            tokens.pop();
            let mut expect_rparen = false;
            let mut elements = Vec::new();

            loop {
                match tokens.peek_for(RParen, String::from("")) {
                    Ok(_) => break,
                    _ => {
                        if expect_rparen {
                            return Err(Error::new(
                                tokens.peek_span(),
                                format!(
                                    "Expected to find {RParen} to conclude tuple variant {}",
                                    name.item
                                ),
                            ));
                        }
                    }
                }

                let ty = type_repr(tokens)?;
                elements.push(ty);

                match tokens.peek_for(Comma, String::from("")) {
                    Ok(_) => { /* */ }
                    Err(_) => expect_rparen = true,
                }
            }

            Ok(Variant::Tuple(name, elements))
        }
        Basic(LBrace) => {
            tokens.pop();
            let mut expect_rbrace = false;
            let mut fields = Vec::new();

            loop {
                match tokens.peek_for(RBrace, String::from("")) {
                    Ok(_) => break,
                    _ => {
                        if expect_rbrace {
                            return Err(Error::new(
                                tokens.peek_span(),
                                format!(
                                    "Expected to find {RBrace} to conclude struct variant {}",
                                    name.item
                                ),
                            ));
                        }
                    }
                }

                let fd = field(tokens)?;
                fields.push(fd);

                match tokens.peek_for(Comma, String::from("")) {
                    Ok(_) => { /* */ }
                    Err(_) => expect_rbrace = true,
                }
            }

            Ok(Variant::Struct(name, fields))
        }
        _ => Ok(Variant::Unit(name)),
    }
}

pub fn peek_type_repr(tokens: &Stream<Token>) -> bool {
    use BasicToken::*;
    use KeywordToken::*;
    use Token::*;

    match tokens.peek() {
        Keyword(U8) => true,
        Keyword(I8) => true,
        Keyword(U16) => true,
        Keyword(I16) => true,
        Keyword(U32) => true,
        Keyword(I32) => true,
        Keyword(Bool) => true,
        Keyword(Char) => true,
        Keyword(Str) => true,
        Keyword(Unit) => true,
        Basic(LBrack) => true,
        Basic(Ampersand) => true,
        Basic(LParen) => true,
        Basic(Colon2) => true,
        Identifier(_) => true,
        _ => false,
    }
}
