pub trait Precedence<P> {
    fn check_precedence(&self) -> P;
}
