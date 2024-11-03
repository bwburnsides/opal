fn main() {
    let token_iter = opalc_lexer::tokenize("foo bar baz!");

    for tok in token_iter {
        println!("{:?}", tok);
    }

    println!("Hello, world!");
}
