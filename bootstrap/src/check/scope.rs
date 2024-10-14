use crate::model::{Symbol, Type};
use std::collections::HashMap;

struct Scope {
    parent: Option<Box<Scope>>,
    symbols: HashMap<String, Symbol>,
}

impl Scope {
    pub fn global() -> Self {
        let symbols = HashMap::from([
            ("Unit".to_owned(), Symbol::Type(Type::Unit)),
            ("u8".to_owned(), Symbol::Type(Type::U8)),
            ("i8".to_owned(), Symbol::Type(Type::I8)),
            ("u16".to_owned(), Symbol::Type(Type::U16)),
            ("i16".to_owned(), Symbol::Type(Type::I16)),
            ("u32".to_owned(), Symbol::Type(Type::U32)),
            ("i32".to_owned(), Symbol::Type(Type::I32)),
            ("bool".to_owned(), Symbol::Type(Type::Bool)),
            ("char".to_owned(), Symbol::Type(Type::Char)),
            ("str".to_owned(), Symbol::Type(Type::Str)),
        ]);

        Self {
            parent: Option::None,
            symbols,
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        match self.symbols.get(name) {
            None => {
                if let Some(parent) = &self.parent {
                    parent.lookup(name)
                } else {
                    None
                }
            }
            Some(sym) => Some(sym),
        }
    }

    pub fn local(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }
}
