#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused)]

use std::io::Write;

use diagnostic::GraphvizRenderer;

mod semantic;
mod diagnostic;
mod driver;
mod error;
mod model;
mod parse;
mod scope;
mod span;
mod stream;
mod lower;

fn main() {
    let source = std::fs::read_to_string("opal_tests/enums.opal").unwrap();
    let mut tokens = parse::tokenize(&source).unwrap();
    let geode = parse::geode("DUMMY_NAME".to_owned(), &mut tokens).unwrap();
    // let graph = GraphvizRenderer::render(&geode);
    // let mut file = std::fs::File::create("test_output.gv").unwrap();
    // file.write(graph.as_bytes());
    println!("{:#?}", geode);
    // println!("{graph}");
}
