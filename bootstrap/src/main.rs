#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::fmt::Display;

mod model;
mod parse;
mod stream;
mod tokenize;

use model::*;
use stream::Stream;

enum BuiltinType {
    Bool,
    U8,
}

fn main() {
    use OpalBasic::*;
    use OpalKeyword::*;
    use OpalLiteral::*;
    use Token::*;

    let source = "enum Color {
        Red,
        Green,
        Blue,
    }";

    let mut tokens = tokenize::tokenize(source).unwrap();

    match parse::jewel(&mut tokens) {
        Ok(item) => println!("{:?}", item),
        Err(err) => println!("{}", err),
    }
}
