#![allow(dead_code)]
#![allow(unused_variables)]

mod check;
mod error;
mod model;
mod parse;
mod span;
mod stream;

type Foo = u8;
type Bar = u8;

fn main() {
    let source = std::fs::read_to_string("opal_tests/enums.opal").unwrap();
    let mut tokens = parse::tokenize(&source).unwrap();
    let ast = parse::module(&mut tokens).unwrap();

    println!("{:#?}", ast);
}
