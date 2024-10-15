enum EmptyEnum {}
enum UnitVariant {
    Foo,
    Bar,
    Baz,
}
enum TupleVariant {
    Foo(u8, UnitVariant),
    Bar(bool, char, EmptyEnum),
}
enum StructVariant {
    SomeBadExample {foo: Foo, bar: Bar, baz: Baz}
}