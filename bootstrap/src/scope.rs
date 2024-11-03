use std::collections::HashMap;

use crate::model::{types::Type, Identifier, Symbol};
use crate::span::Spanned;

pub enum SymbolError {
    MultiplyDefined,
    Undefined,
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    symbols: HashMap<String, Spanned<Symbol>>,
    parent: Option<Box<SymbolTable>>,
}

impl SymbolTable {
    pub fn core_table() -> Self {
        Self {
            symbols: HashMap::new(),
            parent: None,
        }
    }

    pub fn std_table() -> Self {
        // This is a "prelude" of sorts that contains intrinsics via the core table and
        // standard library symbols introduced here.

        Self {
            symbols: HashMap::new(),
            parent: Some(Box::new(SymbolTable::core_table())),
        }
    }

    pub fn geode_table() -> Self {
        Self {
            symbols: HashMap::new(),
            parent: None,
        }
    }

    pub fn lookup(&self, name: &str) -> Result<&Spanned<Symbol>, SymbolError> {
        // Lookup the name in the local scope. If that succeeds, return the located symbol
        // If it fails, and the scope has a parent, look it up in the parent scope.
        // If it doesn't, the symbol is undefined.

        todo!()
        // TODO: fix this
        // self.local(name).or_else(|_err| {
        //     self.parent
        //         .clone()
        //         .map_or(Result::Err(SymbolError::Undefined), |p| p.lookup(name))
        // })
    }

    pub fn local(&self, name: &String) -> Result<&Spanned<Symbol>, SymbolError> {
        // Attempt to retrieve the symbol from the local scope.
        // If its present, return it. If its not, then the symbol is undefined.

        self.symbols.get(name).ok_or(SymbolError::Undefined)
    }

    pub fn insert(&mut self, name: String, symbol: Spanned<Symbol>) -> Result<(), SymbolError> {
        // Attempt to retrieve the symbol from the local scope. If this succeeds, the symbol
        // has becomed multiply defined, so convert the retrieved symbol into an error.
        // If this fails, insert the symbol.

        self.local(&name)
            .map_or(Ok(()), |_sym| Err(SymbolError::MultiplyDefined))
            .and_then(|_unit| {
                self.symbols.insert(name, symbol);
                Ok(())
            })
    }
}
