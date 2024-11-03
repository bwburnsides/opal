use std::iter;

use crate::model::*;
use crate::parse;
use crate::parse::*;
use crate::span::Span;
use crate::stream::Stream;

pub fn item(tokens: &mut Stream<Token>) -> ParseResult<Item> {
    use ItemKind::*;
    use KeywordToken as Kw;
    use Token::*;

    match tokens.peek() {
        Keyword(Kw::Mod) => {
            mod_item(tokens).map(|spanned| Item::new(Mod(spanned.item), spanned.span))
        }
        Keyword(Kw::Use) => {
            use_item(tokens).map(|spanned| Item::new(Use(spanned.item), spanned.span))
        }
        Keyword(Kw::Fn) => {
            function_item(tokens).map(|spanned| Item::new(Function(spanned.item), spanned.span))
        }
        Keyword(Kw::Type) => {
            type_alias_item(tokens).map(|spanned| Item::new(TypeAlias(spanned.item), spanned.span))
        }
        Keyword(Kw::Struct) => {
            struct_item(tokens).map(|spanned| Item::new(Struct(spanned.item), spanned.span))
        }
        Keyword(Kw::Enum) => {
            enum_item(tokens).map(|spanned| Item::new(Enum(spanned.item), spanned.span))
        }
        Keyword(Kw::Const) => {
            const_item(tokens).map(|spanned| Item::new(Const(spanned.item), spanned.span))
        }
        Keyword(Kw::Static) => {
            static_item(tokens).map(|spanned| Item::new(Static(spanned.item), spanned.span))
        }
        _ => Err(Error::new(
            tokens.peek_span(),
            "Expected to find item".to_string(),
        )),
    }
}

fn mod_item(tokens: &mut Stream<Token>) -> ParseResult<Spanned<ModItem>> {
    use BasicToken::*;
    use KeywordToken::*;

    let start = tokens.peek_for(
        Mod,
        format!("Expected to find mod item beginning with {Mod}"),
    )?;
    let name = tokens.peek_for(
        IdentifierToken,
        format!("Expected module name to follow {Mod}"),
    )?;

    match tokens.peek_for(Semicolon, String::from("")) {
        Ok(spanned) => Ok(Spanned::new(
            ModItem::new(name, None),
            Span::between(start.span, spanned.span),
        )),
        Err(_) => {
            tokens.peek_for(
                LBrace,
                format!("Expected {LBrace} following module name {}", name.item),
            )?;

            let mut items = Vec::new();
            loop {
                match tokens.peek_for(RBrace, String::from("")) {
                    Ok(spanned) => {
                        break Ok(Spanned::new(
                            ModItem::new(name, Some(items)),
                            Span::between(start.span, spanned.span),
                        ))
                    }
                    Err(_) => items.push(item(tokens)?),
                }
            }
        }
    }
}

fn use_item(tokens: &mut Stream<Token>) -> ParseResult<Spanned<UseItem>> {
    todo!()

    // use BasicToken::*;
    // use KeywordToken::*;

    // let start = tokens.peek_for(
    //     Use,
    //     format!("Expected to find use item beginning with {Use}"),
    // )?;

    // let tree = use_tree(tokens)?;

    // let end = tokens.peek_for(
    //     Semicolon,
    //     format!("Expected to find {Semicolon} to conclude use item"),
    // )?;

    // Ok(Spanned::new(tree, Span::between(start.span, end.span)))
}

fn function_item(tokens: &mut Stream<Token>) -> ParseResult<Spanned<FunctionItem>> {
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

    Ok(Spanned::new(
        FunctionItem::new(
            function_name,
            parameters,
            return_type,
            Some(body.item), // TODO: Make body optional
        ),
        Span::between(start.span, body.span),
    ))
}

fn type_alias_item(tokens: &mut Stream<Token>) -> ParseResult<Spanned<TypeAliasItem>> {
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

    Ok(Spanned::new(
        TypeAliasItem::new(name, ty),
        Span::between(start.span, end.span),
    ))
}

fn struct_item(tokens: &mut Stream<Token>) -> ParseResult<Spanned<StructItem>> {
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

    while let Token::Identifier(_) = tokens.peek() {
        let fd = field(tokens)?;

        fields.push(fd);

        match tokens.peek_for(Comma, String::from("")) {
            Ok(_) => { /* */ }
            Err(_) => break,
        }
    }

    let end = tokens.peek_for(
        RBrace,
        format!(
            "Expected to find {RBrace} to conclude struct item `{}`",
            name.item
        ),
    )?;

    Ok(Spanned::new(
        StructItem::new(name, fields),
        Span::between(start.span, end.span),
    ))
}

fn enum_item(tokens: &mut Stream<Token>) -> ParseResult<Spanned<EnumItem>> {
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

    while let Token::Identifier(_) = tokens.peek() {
        let vt = variant(tokens)?;
        variants.push(vt);

        match tokens.peek_for(Comma, String::from("")) {
            Ok(_) => { /* */ }
            Err(_) => break,
        }
    }

    let end = tokens.peek_for(
        RBrace,
        format!(
            "Expected to find {RBrace} to conclude enum item `{}`",
            name.item
        ),
    )?;

    Ok(Spanned::new(
        EnumItem::new(name, variants),
        Span::between(start.span, end.span),
    ))
}

fn const_item(tokens: &mut Stream<Token>) -> ParseResult<Spanned<ConstItem>> {
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

    Ok(Spanned::new(
        ConstItem::new(name, ty, value),
        Span::between(start.span, end.span),
    ))
}

fn static_item(tokens: &mut Stream<Token>) -> ParseResult<Spanned<StaticItem>> {
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

    Ok(Spanned::new(
        StaticItem::new(name, ty, value),
        Span::between(start.span, end.span),
    ))
}

// fn use_tree(tokens: &mut Stream<Token>) -> ParseResult<UseTree> {
//     use BasicToken::*;
//     use KeywordToken::*;
//     use Token::*;

//     // UseTree |= Path COLON2 ASTERISK
//     //         |  Path COLON2 LBRACE UseTree (COMMA UseTree)* COMMA? RBRACE
//     //         |  Path (AS IDENT)?
//     //
//     // With some left factoring...
//     //
//     // UseTree |= Path UseTreeTail
//     //
//     // UseTreeTail |= COLON2 ASTERISK
//     //             |  COLON2 LBRACE UseTree (COMMA UseTree)* COMMA? RBRACE
//     //             |  AS IDENT
//     //             |  EPSILON
//     //

//     let mut segments = Vec::new();
//     segments.push(tokens.peek_for(
//         IdentifierToken,
//         format!("Expected identifier while parsing use path"),
//     )?);

//     if let Basic(Colon2) = tokens.peek() {
//         tokens.pop();

//         loop {
//             match tokens.peek() {
//                 Identifier(name) => {
//                     let popped = tokens.pop();
//                     segments.push(ast::Identifier::new(name, popped.span));
//                 }
//                 Basic(Asterisk) => {
//                     tokens.pop();
//                     break Ok(UseTree::Wildcard(UsePath::new(
//                         segments
//                             .pop()
//                             .expect("Use tree parser should always produce at least one name..."),
//                         segments,
//                     )));
//                 }
//                 Basic(LBrace) => {
//                     tokens.pop();
//                     let mut trees = vec![use_tree(tokens)?];
//                     loop {
//                         if let Basic(Comma) = tokens.peek() {
//                             tokens.pop();
//                             trees.push(use_tree(tokens)?);
//                         } else {
//                             break;
//                         }
//                     }
//                     let _ = tokens.peek_for(Comma, String::from(""));
//                     tokens.peek_for(RBrace, format!("Expected {RBrace} to conclude use tree"))?;
//                     break Ok(UseTree::Children(
//                         UsePath::new(
//                             segments.pop().expect(
//                                 "Use tree parser should always produce at least one name...",
//                             ),
//                             segments,
//                         ),
//                         trees,
//                     ));
//                 },
//                 otherwise => break Err(
//                     Error::new(
//                         tokens.peek_span(),
//                         format!(
//                             "Expected identifier, {Asterisk}, or {LBrace} as part of use tree, but found {otherwise} instead"
//                         )
//                     )
//                 )
//             }

//             match tokens.peek() {
//                 Basic(Colon2) => {
//                     tokens.pop();
//                 }
//                 Keyword(As) => {
//                     tokens.pop();
//                     let name = tokens.peek_for(
//                         IdentifierToken,
//                         format!("Expected identifier following {As}"),
//                     )?;
//                     break Ok(UseTree::Rebind(
//                         UsePath::new(segments.pop().unwrap(), segments),
//                         name,
//                     ));
//                 }
//                 _epsilon => {
//                     break Ok(UseTree::Import(UsePath::new(
//                         segments.pop().unwrap(),
//                         segments,
//                     )))
//                 }
//             }
//         }
//     } else {
//         match tokens.peek() {
//             Keyword(As) => {
//                 tokens.pop();
//                 let name = tokens.peek_for(
//                     IdentifierToken,
//                     format!("Expected identifier following {As}"),
//                 )?;
//                 Ok(UseTree::Rebind(
//                     UsePath::new(segments.pop().unwrap(), segments),
//                     name,
//                 ))
//             }
//             _epsilon => Ok(UseTree::Import(UsePath::new(
//                 segments.pop().unwrap(),
//                 segments,
//             ))),
//         }
//     }
// }

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
                "Expected to find array type literal's span".to_string(),
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
            let ty_span = ty.span;

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

    let name = tokens.peek_for(
        IdentifierToken,
        "Expected to find parameter name".to_string(),
    )?;
    let name_span = name.span;

    tokens.peek_for(
        Colon,
        format!(
            "Expected to find {Colon} following parameter named {}",
            name.item
        ),
    )?;

    let ty = type_repr(tokens)?;
    let ty_span = ty.span;

    Ok(Parameter::new(
        mutability,
        name,
        ty,
        Span::between(maybe_start.unwrap_or(name_span), ty_span),
    ))
}

fn field(tokens: &mut Stream<Token>) -> ParseResult<Field> {
    use BasicToken::Colon;

    let name = tokens.peek_for(IdentifierToken, "Expected field name".to_string())?;
    let name_span = name.span;

    tokens.peek_for(
        Colon,
        format!("Expected to find {Colon} following {}", name.item),
    )?;

    let ty = type_repr(tokens)?;
    let ty_span = ty.span;

    Ok(Field::new(name, ty, Span::between(name_span, ty_span)))
}

fn variant(tokens: &mut Stream<Token>) -> ParseResult<Variant> {
    use BasicToken::*;
    use Token::*;

    let name = tokens.peek_for(
        IdentifierToken,
        "Expected to find variant identifier".to_string(),
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

    matches!(
        tokens.peek(),
        Keyword(U8)
            | Keyword(I8)
            | Keyword(U16)
            | Keyword(I16)
            | Keyword(U32)
            | Keyword(I32)
            | Keyword(Bool)
            | Keyword(Char)
            | Keyword(Str)
            | Keyword(Unit)
            | Basic(LBrack)
            | Basic(Ampersand)
            | Basic(LParen)
            | Basic(Colon2)
            | Identifier(_)
    )
}
