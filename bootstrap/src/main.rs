#![allow(dead_code)]
#![allow(unused_variables)]

mod error;
mod model;
mod parse;
mod span;
mod stream;

fn main() {
    let mut tokens = parse::tokenize("foo[[1,2]]").unwrap();

    let result = parse::expression(&mut tokens);

    match result {
        Ok(expr) => println!("{expr:#?}"),
        Err(err) => println!("{err:#?}"),
    }
}
