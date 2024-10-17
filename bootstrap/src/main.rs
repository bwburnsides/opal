#![allow(dead_code)]
#![allow(unused_variables)]

mod check;
mod driver;
mod error;
mod model;
mod parse;
mod scope;
mod span;
mod stream;

fn main() {
    let source = std::fs::read_to_string("opal_tests/mod_test.opal").unwrap();
    let mut tokens = parse::tokenize(&source).unwrap();
    let ast = parse::geode(&mut tokens).unwrap();

    println!("{:#?}", ast);
}
