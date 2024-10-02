#[derive(PartialEq, Clone)]
enum OpalKeyword {
    Fn,
    Type,
    Struct,
    Enum,
    Static,
    Const,
}

#[derive(PartialEq, Clone)]
enum OpalBasic {
    LBrace,
    RBrace,
    Comma,
}

#[derive(PartialEq, Clone)]
enum Token {
    Keyword(OpalKeyword),
    Identifier(String),
    Basic(OpalBasic),
}

struct Stream<T> {
    items: Vec<T>,
    position: usize,
}

impl<T: Clone> Stream<T> {
    fn new() -> Self {
        Self {
            items: Vec::new(),
            position: 0,
        }
    }

    fn peek(&self) -> Option<&T> {
        self.items.get(self.position)
    }

    fn pop(self) -> (Option<T>, Self) {
        match self.peek() {
            None => (None, self),
            Some(item) => (Some(item.clone()), self.advance()),
        }
    }

    fn advance(self) -> Self {
        Self {
            items: self.items,
            position: self.position + 1,
        }
    }
}

type Jewel = Vec<Item>;

struct FunctionItem {}
struct TypeAliasItem {}
struct StructItem {}
struct EnumItem {}
struct StaticItem {}
struct ConstItem {}

enum Item {
    Function(FunctionItem),
    TypeAlias(TypeAliasItem),
    Struct(StructItem),
    Enum(EnumItem),
    Const(ConstItem),
    Static(StaticItem),
}

enum ParseResult<T> {
    Match(T, Stream<Token>),
    Error,
}

impl<T> ParseResult<T> {
    fn map<U, F>(self, mapper: F) -> ParseResult<U>
    where
        F: Fn(T) -> U,
    {
        todo!()
    }
}

fn parse_function() -> ParseResult<FunctionItem> {
    todo!()
}

fn parse_type_alias() -> ParseResult<TypeAliasItem> {
    todo!()
}

fn parse_struct() -> ParseResult<StructItem> {
    todo!()
}

fn parse_enum(tokens: Stream<Token>) -> ParseResult<EnumItem> {
    use OpalBasic::*;
    use OpalKeyword::*;
    use Token::*;

    let (keyword, remaining) = match tokens.peek() {
        None => return ParseResult::Error,
        Some(Keyword(Enum)) => tokens.pop(),
        Some(other_token) => return ParseResult::Error,
    };

    let (identifier, remaining) = match remaining.peek() {
        None => return ParseResult::Error,
        Some(Identifier(_)) => remaining.pop(),
        Some(other_token) => return ParseResult::Error,
    };

    todo!()
}

fn parse_static() -> ParseResult<StaticItem> {
    todo!()
}

fn parse_const() -> ParseResult<ConstItem> {
    todo!()
}

fn parse_item(tokens: Stream<Token>) -> ParseResult<Item> {
    use OpalKeyword::*;
    use Token::*;

    match tokens.peek() {
        None => ParseResult::Error,
        Some(token) => match token {
            Keyword(Fn) => parse_function().map(Item::Function),
            Keyword(Type) => parse_type_alias().map(Item::TypeAlias),
            Keyword(Struct) => parse_struct().map(Item::Struct),
            Keyword(Enum) => parse_enum(tokens).map(Item::Enum),
            Keyword(Static) => parse_static().map(Item::Static),
            Keyword(Const) => parse_const().map(Item::Const),
            _ => ParseResult::Error,
        },
    }
}

fn parse_jewel(tokens: Stream<Token>) -> ParseResult<Jewel> {
    let mut items = Vec::new();
    let mut remaining = tokens;

    loop {
        match remaining.peek() {
            None => return ParseResult::Match(items, remaining),
            Some(_) => match parse_item(remaining) {
                ParseResult::Match(item, rem) => {
                    items.push(item);
                    remaining = rem;
                }
                ParseResult::Error => return ParseResult::Error,
            },
        }
    }
}

fn main() {
    let tokens = Stream::<Token>::new();
    parse_jewel(tokens);
}
