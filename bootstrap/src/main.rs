mod error;
mod model;
mod parse;
mod pratt;
mod span;
mod stream;
mod tokenize;

fn main() {
    // match tokenize::tokenize("enum Color {Red, Green, Blue}") {
    //     Ok(mut tokens) => {
    //         let _ = parse::jewel(&mut tokens);
    //     }
    //     Err(err) => {
    //         println!("{:?}", err);
    //     }
    // };
}
