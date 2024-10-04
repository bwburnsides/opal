use std::fmt::Display;

use crate::model::*;
use crate::stream::{PeekFor, Stream};

impl PeekFor<OpalBasic, ParseResult<Token>> for Stream<Token> {
    fn peek_for<S: Into<String> + Display>(
        &mut self,
        kind: OpalBasic,
        error: S,
    ) -> ParseResult<Token> {
        match self.peek() {
            Token::Basic(kind) => Ok(self.pop()),
            otherwise => Err(format!("{} but found {} instead", error, otherwise)),
        }
    }
}

impl PeekFor<OpalKeyword, ParseResult<Token>> for Stream<Token> {
    fn peek_for<S: Into<String> + Display>(
        &mut self,
        kind: OpalKeyword,
        error: S,
    ) -> ParseResult<Token> {
        match self.peek() {
            Token::Keyword(kind) => Ok(self.pop()),
            otherwise => Err(format!("{} but found {} instead", error, otherwise)),
        }
    }
}

impl PeekFor<OpalIdentifier, ParseResult<String>> for Stream<Token> {
    fn peek_for<S: Into<String> + Display>(
        &mut self,
        kind: OpalIdentifier,
        error: S,
    ) -> ParseResult<String> {
        match self.peek() {
            Token::Identifier(name) => {
                self.pop();
                Ok(name)
            }
            otherwise => Err(format!("{} but found {} instead", error, otherwise)),
        }
    }
}

type ParseResult<T> = Result<T, String>;

pub fn jewel(tokens: &mut Stream<Token>) -> ParseResult<Jewel> {
    // <jewel> |= <item> <jewel> $
    //         |  $

    match tokens.peek() {
        Token::Eof => return Ok(Vec::new()),
        otherwise => {
            let it = item(tokens)?;
            let mut items = jewel(tokens)?;
            items.insert(0, it);
            Ok(items)
        }
    }
}

fn item(tokens: &mut Stream<Token>) -> ParseResult<Item> {
    // <item> |= <function>
    //        |  <type-alias>
    //        |  <struct>
    //        |  <enum>
    //        |  <static>
    //        |  <const>

    use OpalKeyword::*;
    use Token::*;

    match tokens.peek() {
        Keyword(Fn) => function(tokens).map(Item::Function),
        Keyword(Type) => type_alias().map(Item::TypeAlias),
        Keyword(Struct) => struct_item(tokens).map(Item::Struct),
        Keyword(Enum) => enum_item(tokens).map(Item::Enum),
        Keyword(Static) => static_item().map(Item::Static),
        Keyword(Const) => const_item().map(Item::Constant),
        otherwise => Err(format!(
            "Expected to find item definition beginning with keyword {} but found {} instead",
            "`fn`, `type`, `struct`, `enum`, `static`, or `const`", otherwise,
        )),
    }
}

fn function(tokens: &mut Stream<Token>) -> ParseResult<FunctionItem> {
    // <function> |= FN IDENT LPAREN <param-list> RPAREN <opt-return> <block-expr>

    use OpalBasic::*;
    use OpalKeyword::*;
    use Token::*;

    let keyword = tokens.peek_for(
        Fn,
        format!(
            "Expected to find function definition beginning with {}",
            Keyword(Fn)
        ),
    )?;

    let name = tokens.peek_for(
        OpalIdentifier,
        format!(
            "Expected to find function definition's identifier after {}",
            Keyword(Fn)
        ),
    )?;

    tokens.peek_for(
        LParen,
        format!(
            "Expected to find {} following function identifier \"{}\"",
            Basic(LParen),
            name
        ),
    )?;

    let param_list = param_list(tokens)?;

    tokens.peek_for(
        RParen,
        format!(
            "Expected to find '{}' following function \"{}\"'s parameter list",
            Basic(RParen),
            name
        ),
    )?;

    let return_type = opt_return(tokens)?;

    let body = block_expression(tokens)?;

    Ok(FunctionItem::new(name, param_list, return_type, body))
}

fn enum_item(tokens: &mut Stream<Token>) -> ParseResult<EnumItem> {
    // <enum> |= ENUM IDENT LBRACE <enum-members> RBRACE

    use OpalBasic::*;
    use OpalKeyword::*;
    use Token::*;

    let keyword = match tokens.peek() {
        Keyword(Enum) => tokens.pop(),
        otherwise => {
            return Err(format!(
                "Expected to find enum definition beginning with {} but found {} instead",
                Keyword(Enum),
                otherwise
            ))
        }
    };

    let identifier = match tokens.peek() {
        Identifier(name) => {
            tokens.pop();
            name.clone()
        }
        otherwise => {
            return Err(format!(
                "Expected to find enum definition's identifier after {} but found {} instead",
                Keyword(Enum),
                otherwise
            ))
        }
    };

    match tokens.peek() {
        Basic(LBrace) => tokens.pop(),
        otherwise => {
            return Err(format!(
                "Expected to find {} following enum identifier {} but found {} instead",
                Basic(LBrace),
                identifier,
                otherwise
            ))
        }
    };

    let members = enum_members(tokens)?;

    match tokens.peek() {
        Basic(RBrace) => {
            tokens.pop();
            Ok(EnumItem::new(identifier, members))
        }
        otherwise => Err(format!(
            "Expected to find {} to conclude enum definition but found {} instead",
            Basic(RBrace),
            otherwise
        )),
    }
}

fn static_item() -> ParseResult<StaticItem> {
    todo!()
}

fn const_item() -> ParseResult<ConstItem> {
    todo!()
}

fn struct_item(tokens: &mut Stream<Token>) -> ParseResult<StructItem> {
    // <struct> |= STRUCT IDENT LBRACE <struct-members> RBRACE

    use OpalBasic::*;
    use OpalKeyword::*;
    use Token::*;

    let keyword = match tokens.peek() {
        Keyword(Struct) => tokens.pop(),
        otherwise => {
            return Err(format!(
                "Expected to find struct definition beginning with {} but found {} instead",
                Keyword(Struct),
                otherwise
            ))
        }
    };

    let identifier = tokens.peek_for(
        OpalIdentifier,
        "Expected to find struct definition's identifier",
    )?;

    let _ = tokens.peek_for(
        LBrace,
        format!(
            "Expected {} following struct identifier \"{}\"",
            LBrace, identifier
        ),
    )?;

    let fields = struct_fields(tokens)?;

    let rbrace = tokens.peek_for(
        RBrace,
        format!("Expected '{}' following struct definition", RBrace),
    )?;

    Ok(StructItem::new(identifier, fields))
}

fn type_alias() -> ParseResult<TypeAliasItem> {
    todo!()
}

fn expression(tokens: &mut Stream<Token>) -> ParseResult<Expression> {
    // <expression> |= LBRACE <statements> RBRACE
    //              |  IF <expression> <block-expression>
    //              |  IF <expression> <block-expression> ELSE <block-expression>
    //              |  IF <expression> <block-expression> ELSE <if-expression>
    //              |  WHEN <expression> <...>
    //              |  FOR <...>
    //              |  WHILE <...>
    //              |  CHAR-LIT
    //              |  STR-LIT
    //              |  INT-LIT
    //              |  TRUE
    //              |  FALSE
    //              |  COLON2 IDENT <...>   // PathExpression
    //              |  IDENT <...>          // PathExpression
    
}

fn block_expression(tokens: &mut Stream<Token>) -> ParseResult<BlockExpression> {
    // <block-expression> |= LBRACE <statements> RBRACE

    use OpalBasic::*;
    use OpalKeyword::*;
    use Token::*;

    let lbrace = tokens.peek_for(
        LBrace,
        format!("Expected to find {} to begin block expression", LBrace),
    )?;

    let statements = statements(tokens)?;

    let rbrace = tokens.peek_for(
        RBrace,
        format!("Expected to find {} to begin block expression", RBrace),
    )?;

    Ok(BlockExpression::from(statements))
}

fn statements(tokens: &mut Stream<Token>) -> ParseResult<Vec<Statement>> {
    // <statements> |= <statement> <statements>
    //              |  EPSILON

    use OpalBasic::*;
    use OpalKeyword::*;
    use Token::*;

    match tokens.peek() {
        Keyword(Let) => let_statement(tokens).map(Statement::Let)?,
        Keyword(Continue) => Statement::Continue,
        Keyword(Break) => Statement::Break,
        _ => todo!()
    };

    todo!()
}

fn let_statement(tokens: &mut Stream<Token>) -> ParseResult<LetStatement> {
    todo!()
}

fn type_repr(tokens: &mut Stream<Token>) -> ParseResult<TypeRepr> {
    todo!()
}

fn opt_return(tokens: &mut Stream<Token>) -> ParseResult<Option<TypeRepr>> {
    // <opt-return> |= LIGHT_RARROW <type-repr>
    //              |  EPSILON

    use OpalBasic::*;
    use OpalKeyword::*;
    use Token::*;

    match tokens.peek() {
        Basic(LightRArrow) => {
            tokens.pop();
            match type_repr(tokens) {
                Ok(type_repr) => Ok(Some(type_repr)),
                Err(err) => Err(err),
            }
        }
        epsilon => Ok(None),
    }
}

fn param_list(tokens: &mut Stream<Token>) -> ParseResult<Vec<Parameter>> {
    todo!()
}

fn struct_fields(tokens: &mut Stream<Token>) -> ParseResult<Vec<Field>> {
    todo!()
}

fn enum_members(tokens: &mut Stream<Token>) -> ParseResult<Vec<String>> {
    // <enum-members> |= IDENT <enum-members-tail>
    //                |  EPSILON

    use OpalBasic::*;
    use OpalKeyword::*;
    use Token::*;

    fn enum_members_tail(tokens: &mut Stream<Token>) -> ParseResult<Vec<String>> {
        // <enum-members-tail> |= COMMA <enum-members>
        //                     |  EPSILON

        match tokens.peek() {
            Basic(Comma) => {
                tokens.pop();
                enum_members(tokens)
            }
            epsilon => Ok(Vec::new()),
        }
    }

    let mut names = Vec::new();

    let name = match tokens.peek() {
        Identifier(name) => {
            tokens.pop();
            name.clone()
        }
        epsilon => return Ok(names),
    };

    names.push(name.clone());

    match enum_members_tail(tokens) {
        Ok(tail) => {
            names.extend(tail);
            Ok(names)
        }
        Err(err) => Err(err),
    }
}
