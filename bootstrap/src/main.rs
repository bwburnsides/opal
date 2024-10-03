#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::fmt::Display;

mod model;
mod parse;
mod stream;

use model::*;
use stream::Stream;

enum BuiltinType {
    Bool,
    U8,
}

fn main() {
    use OpalBasic::*;
    use OpalKeyword::*;
    use Token::*;

    let mut tokens = Stream::from([
        Keyword(Enum),
        Identifier(String::from("Foo")),
        Basic(LBrace),
        Basic(RBrace),
        Keyword(Enum),
        Identifier(String::from("Bar")),
        Basic(LBrace),
        Basic(RBrace),
    ]);

    match parse::jewel(&mut tokens) {
        Ok(item) => println!("{:?}", item),
        Err(err) => println!("{}", err),
    }
}
