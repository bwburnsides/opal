#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Span {
    start: usize,
    stop: usize,
}

struct Spanned<T> {
    item: T,
    span: Span,
}

#[derive(Debug, PartialEq, Clone)]
enum OpalKeyword {
    Fn,
    Type,
    Struct,
    Enum,
    Static,
    Const,
    If,
}

impl std::fmt::Display for OpalKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use OpalKeyword::*;

        match self {
            Fn => write!(f, "fn"),
            Type => write!(f, "type"),
            Struct => write!(f, "struct"),
            Enum => write!(f, "enum"),
            Static => write!(f, "static"),
            Const => write!(f, "const"),
            If => write!(f, "if"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum OpalBasic {
    LBrace,
    RBrace,
    Comma,
}

impl std::fmt::Display for OpalBasic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use OpalBasic::*;

        match self {
            LBrace => write!(f, "{{"),
            RBrace => write!(f, "}}"),
            Comma => write!(f, ","),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Keyword(OpalKeyword),
    Identifier(String),
    Basic(OpalBasic),
    Eof,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Keyword(kw) => write!(f, "keyword `{}`", kw),
            Token::Identifier(name) => write!(f, "identifier \"{}\"", name),
            Token::Basic(basic) => write!(f, "token \"{}\"", basic),
            Token::Eof => write!(f, "end of file"),
        }
    }
}

impl EndMarked for Token {
    const END: EndMarker<Self> = &Token::Eof;
}

type EndMarker<T> = &'static T;

trait EndMarked
where
    Self: 'static,
{
    const END: EndMarker<Self>;
}

#[derive(Debug)]
struct Stream<T> {
    items: Vec<T>,
    position: usize,
}

impl<T: Clone + EndMarked + 'static + PartialEq> Stream<T> {
    fn new() -> Self {
        Self {
            items: Vec::new(),
            position: 0,
        }
    }

    fn peek(&self) -> &T {
        match self.items.get(self.position) {
            None => T::END,
            Some(item) => item,
        }
    }

    fn pop(self) -> (T, Self) {
        let peeked = self.peek();

        if peeked == T::END {
            (T::END.clone(), self)
        } else {
            (peeked.clone(), self.advance())
        }
    }

    fn advance(self) -> Self {
        Self {
            items: self.items,
            position: self.position + 1,
        }
    }
}

impl<T: Clone, const N: usize> From<[T; N]> for Stream<T> {
    fn from(items: [T; N]) -> Self {
        Self {
            items: Vec::from(items),
            position: 0,
        }
    }
}

type Jewel = Vec<Item>;

#[derive(Debug)]
struct FunctionItem {}

#[derive(Debug)]
struct TypeAliasItem {}

#[derive(Debug)]
struct StructItem {}

#[derive(Debug)]
struct EnumItem {
    name: String,
    members: Vec<String>,
}

impl EnumItem {
    fn new(name: String, members: Vec<String>) -> Self {
        Self { name, members }
    }
}

#[derive(Debug)]
struct StaticItem {}

#[derive(Debug)]
struct ConstItem {}

#[derive(Debug)]
enum Item {
    Function(FunctionItem),
    TypeAlias(TypeAliasItem),
    Struct(StructItem),
    Enum(EnumItem),
    Const(ConstItem),
    Static(StaticItem),
}

enum BuiltinType {
    Bool,
    U8,
}

enum TypeRepr {
    Builtin(BuiltinType),
    Identifier(String),
}

#[derive(Debug)]
enum ParseResult<T> {
    Match(T, Stream<Token>),
    Error(String),
}

impl<T> ParseResult<T> {
    fn map<U, F>(self, mapper: F) -> ParseResult<U>
    where
        F: Fn(T) -> U,
    {
        match self {
            Self::Match(item, remaining) => ParseResult::Match(mapper(item), remaining),
            Self::Error(err) => ParseResult::Error(err),
        }
    }
}

fn parse_function() -> ParseResult<FunctionItem> {
    todo!()
}

fn parse_type_alias() -> ParseResult<TypeAliasItem> {
    todo!()
}

fn parse_struct_members(tokens: Stream<Token>) -> ParseResult<Vec<(String, TypeRepr)>> {
    todo!()
}

fn parse_struct(tokens: Stream<Token>) -> ParseResult<StructItem> {
    // <struct> |= STRUCT IDENT LBRACE <struct-members> RBRACE

    use OpalBasic::*;
    use OpalKeyword::*;
    use Token::*;

    let (keyword, remaining) = match tokens.peek() {
        Keyword(Struct) => tokens.pop(),
        otherwise => {
            return ParseResult::Error(format!(
                "Expected to find struct definition beginning with {} but found {} instead",
                Keyword(Struct),
                otherwise
            ))
        }
    };

    let (identifier, remaining) = match remaining.peek() {
        Identifier(name) => (name.clone(), remaining.advance()),
        otherwise => {
            return ParseResult::Error(format!(
                "Expected to find struct definition's identifier after {} but found {} instead",
                Keyword(Struct),
                otherwise
            ))
        }
    };

    let remaining = match remaining.peek() {
        Basic(LBrace) => remaining.advance(),
        otherwise => {
            return ParseResult::Error(format!(
                "Expected to find {} following struct identifier {} but found {} instead",
                Basic(LBrace),
                identifier,
                otherwise
            ))
        }
    };

    let (members, remaining) = match parse_struct_members(remaining) {
        ParseResult::Match(members, remaining) => (members, remaining),
        ParseResult::Error(err) => return ParseResult::Error(err),
    };

    // match remaining.peek() {
    //     Basic(RBrace) => {
    //         ParseResult::Match(EnumItem::new(identifier, members), remaining.advance())
    //     }
    //     otherwise => ParseResult::Error(format!(
    //         "Expected to find {} to conclude enum definition but found {} instead",
    //         Basic(RBrace),
    //         otherwise
    //     )),
    // }

    todo!()
}

fn parse_enum_members(tokens: Stream<Token>) -> ParseResult<Vec<String>> {
    // <enum-members> |= IDENT <enum-members-tail>
    //                |  EPSILON

    use OpalBasic::*;
    use OpalKeyword::*;
    use Token::*;

    fn parse_enum_members_tail(tokens: Stream<Token>) -> ParseResult<Vec<String>> {
        // <enum-members-tail> |= COMMA <enum-members>
        //                     |  EPSILON

        match tokens.peek() {
            Basic(Comma) => parse_enum_members(tokens.advance()),
            epsilon => ParseResult::Match(Vec::new(), tokens),
        }
    }

    let mut names = Vec::new();

    let (name, remaining) = match tokens.peek() {
        Identifier(name) => (name.clone(), tokens.advance()),
        epsilon => return ParseResult::Match(names, tokens),
    };

    names.push(name.clone());

    match parse_enum_members_tail(remaining) {
        ParseResult::Match(tail, remaining) => {
            names.extend(tail);
            ParseResult::Match(names, remaining)
        }
        ParseResult::Error(err) => ParseResult::Error(err),
    }
}

fn parse_enum(tokens: Stream<Token>) -> ParseResult<EnumItem> {
    // <enum> |= ENUM IDENT LBRACE <enum-members> RBRACE

    use OpalBasic::*;
    use OpalKeyword::*;
    use Token::*;

    let (keyword, remaining) = match tokens.peek() {
        Keyword(Enum) => tokens.pop(),
        otherwise => {
            return ParseResult::Error(format!(
                "Expected to find enum definition beginning with {} but found {} instead",
                Keyword(Enum),
                otherwise
            ))
        }
    };

    let (identifier, remaining) = match remaining.peek() {
        Identifier(name) => (name.clone(), remaining.advance()),
        otherwise => {
            return ParseResult::Error(format!(
                "Expected to find enum definition's identifier after {} but found {} instead",
                Keyword(Enum),
                otherwise
            ))
        }
    };

    let remaining = match remaining.peek() {
        Basic(LBrace) => remaining.advance(),
        otherwise => {
            return ParseResult::Error(format!(
                "Expected to find {} following enum identifier {} but found {} instead",
                Basic(LBrace),
                identifier,
                otherwise
            ))
        }
    };

    let (members, remaining) = match parse_enum_members(remaining) {
        ParseResult::Match(members, remaining) => (members, remaining),
        ParseResult::Error(err) => return ParseResult::Error(err),
    };

    match remaining.peek() {
        Basic(RBrace) => {
            ParseResult::Match(EnumItem::new(identifier, members), remaining.advance())
        }
        otherwise => ParseResult::Error(format!(
            "Expected to find {} to conclude enum definition but found {} instead",
            Basic(RBrace),
            otherwise
        )),
    }
}

fn parse_static() -> ParseResult<StaticItem> {
    todo!()
}

fn parse_const() -> ParseResult<ConstItem> {
    todo!()
}

fn parse_item(tokens: Stream<Token>) -> ParseResult<Item> {
    // <item> |= <function>
    //        |  <type-alias>
    //        |  <struct>
    //        |  <enum>
    //        |  <static>
    //        |  <const>

    use OpalKeyword::*;
    use Token::*;

    match tokens.peek() {
        Keyword(Fn) => parse_function().map(Item::Function),
        Keyword(Type) => parse_type_alias().map(Item::TypeAlias),
        Keyword(Struct) => parse_struct(tokens).map(Item::Struct),
        Keyword(Enum) => parse_enum(tokens).map(Item::Enum),
        Keyword(Static) => parse_static().map(Item::Static),
        Keyword(Const) => parse_const().map(Item::Const),
        otherwise => ParseResult::Error(format!(
            "Expected to find item definition beginning with keyword {} but found {} instead",
            "`fn`, `type`, `struct`, `enum`, `static`, or `const`", otherwise,
        )),
    }
}

fn parse_jewel(tokens: Stream<Token>) -> ParseResult<Jewel> {
    //  <jewel> |= <item>* $

    use OpalKeyword::*;
    use Token::*;

    let mut items = Vec::new();
    let mut remaining = tokens;

    loop {
        match remaining.peek() {
            Token::Eof => return ParseResult::Match(items, remaining),
            otherwise => match parse_item(remaining) {
                ParseResult::Match(item, rem) => {
                    items.push(item);
                    remaining = rem;
                }
                ParseResult::Error(err) => return ParseResult::Error(err),
            },
        }
    }
}

fn main() {
    use OpalBasic::*;
    use OpalKeyword::*;
    use Token::*;

    let tokens = Stream::from([
        Keyword(If),
        Identifier(String::from("Color")),
        Basic(LBrace),
        // Basic(RBrace),
    ]);

    match parse_jewel(tokens) {
        ParseResult::Match(item, _) => println!("{:?}", item),
        ParseResult::Error(err) => println!("{}", err),
    }
}
