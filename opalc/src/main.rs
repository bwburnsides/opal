mod lexer;
mod ast;
mod span;

fn main() {
    let token_iter = lexer::tokenize("foo bar baz!");

    for tok in token_iter {
        println!("{:?}", tok);
    }

    println!("Hello, world!");
}
