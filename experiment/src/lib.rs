#![allow(unused)]
use std::collections::HashMap;

// `Span` represents the a range of positions in a source file from .0 to .1 (TODO: Inclusive right?)
struct Span(usize, usize);

type SymbolIndex = usize;

// Identifies an AST node
type NodeId = usize;

// `Symbol` represents an interned string living in a datastructure at the given index.
struct Symbol(SymbolIndex);

struct Ident {
    name: Symbol,
    span: Span,
}

enum ItemKind {
    // ExternCrate, Use, Static, Const, Fn, Mod, TyAlias, Enum, Struct, ....
}

struct Item {
    id: NodeId,
    span: Span,
    ident: Ident,
    kind: ItemKind,
    // Rust also includes a stream of tokens that the node is made up of
}

struct Local;

enum StmtKind {
    Let(Local),
    Item(Item),
    Expr(Expr),
    Semi(Expr),
    Empty,
}

struct Stmt {
    id: NodeId,
    kind: StmtKind,
    span: Span,
}

enum ExprKind {
    Array, ConstBlock, Call, MethodCall, Tup, Binary, Unary, Lit, Cast, Type, Let, If, // ... A bunch of others
}

struct Expr {
    id: NodeId,
    kind: ExprKind,
    span: Span,
    // Tokens
}

// An index into the `hir-map` for a crate, identifying a particular definition.
// Should be considered an interned shorthand for a DefPath.
struct DefIndex(usize);

struct CrateNum(usize);

struct DefId {
    index: DefIndex,
    krate: CrateNum,
}

// "What kind of definition something is"
enum DefKind {
    Mod, Sturct, Union, Enum, Variant, Trait, TyAlias, Fn, Const, // ... A bunch of others
}

// Not represented directly in the AST; referred to by name through a `ty_path`
enum PrimTy {
    Int, Uint, Float, Str, Bool, Char
}

// "The resolution of a path or export"
// This is the output of name resolution
enum Resolution {
    Def(DefKind, DefId),
    PrimTy(PrimTy),
    SelfCtor(DefId),
}

enum HasGenericParams {
    Yes(Span), No
}

struct ModuleData {
    // A bunch of shit
}

struct Interned<T>(T);

struct Module(Interned<ModuleData>);

// A specific kind of scope. Each have different lookup rules and restrictions
enum RibKind {
    Normal, AssocItem, Item(HasGenericParams, DefKind), Module(Module)
}

// Basically a scope
struct Rib {
    bindings: HashMap<Ident, Resolution>,
    kind: RibKind
}

// The resolution keeps a separate stack of rubs as it traverses the AST for each namespace.
// When resolving, the name is looked up from inside out.

// Rust uses separate namespaces for types and variables


/*
# Type Alias
type Tokens = Vec[Token]

# Tuple Type
type Span(u32, u32)

# Sum Type
enum TokenKind {
    # Unit Variant
    Plus,

    # Sum Variant
    Name(String),

    # Product Variant
    Integer {
        value: u32,
        base: IntegerBase,
    }
}

# Product Type
struct Token {
    span: Span
    kind: TokenKind
}


fn main() {
    case expr {
        True -> {
            println("Got true!")
        },
        False -> println("Got false!")
    }

    if expr is True {
        println("Got true another way")
    }

    for idx in range(0, 5) {
        println(idx.to_string())
    }
}
*/

/*
if expr          --- Desugars -->>  if expr is true
case expr {...}  --- Lowers ---->>  *Decision Trees*
*/