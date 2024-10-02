#[derive(Clone, Copy)]
pub struct Span(usize, usize);

impl Span {
    pub fn from_pair(first: Self, second: Self) -> Self {
        Span(first.0, second.1)
    }
}

#[derive(Clone, Copy)]
pub struct Spanned<T> {
    item: T,
    span: Span,
}

impl<T> Spanned<T> {
    pub fn new(item: T, span: Span) -> Self {
        Self {
            item: item,
            span: span,
        }
    }
}

#[derive(Clone)]
pub struct Stream<T> {
    items: Vec<Spanned<T>>,
    position: usize,
}

impl<T> Stream<T>
where
    T: Copy,
{
    pub fn new(iter: impl Iterator<Item = Spanned<T>>) -> Self {
        Self {
            items: iter.collect(),
            position: 0,
        }
    }

    pub fn peek(&self) -> Option<Spanned<T>> {
        match self.items.get(self.position) {
            None => None,
            Some(spanned) => Some(*spanned),
        }
    }

    pub fn advance(self) -> Self {
        self.advance_by(1)
    }

    pub fn advance_by(self, count: usize) -> Self {
        Self {
            items: self.items,
            position: self.position + count,
        }
    }
}

#[derive(Clone)]
pub struct ErrorReport<E> {
    errors: Vec<E>,
}

impl<E> ErrorReport<E> {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }
}

pub enum ParseResult<Input, Output, Error> {
    Match(Spanned<Output>, Stream<Input>),
    NoMatch,
    Error(Error),
}

pub type ParserReturn<P> =
    ParseResult<<P as Parser>::Input, <P as Parser>::Output, <P as Parser>::Error>;

pub trait Parser {
    type Input;
    type Output;
    type Error;

    fn run(self, input: Stream<Self::Input>) -> ParserReturn<Self>;

    fn and_then<Second>(self, second: Second) -> AndThen<Self, Second>
    where
        Self: Sized,
        Second: Parser<Input = Self::Input, Error = Self::Error>,
    {
        AndThen {
            first: self,
            second: second,
        }
    }
}

pub struct Just<I>(I);

impl<I> Parser for Just<I>
where
    I: PartialEq + Copy,
{
    type Input = I;
    type Output = I;
    type Error = ();

    // TODO: Should this return a Match with a None and empty ErrorReport? Or should it return a NoMatch?
    // TODO: Also should ParseResult::Match use an Input or Option<Input>?
    fn run(self, input: Stream<Self::Input>) -> ParserReturn<Self> {
        match input.peek() {
            None => ParseResult::NoMatch,
            Some(spanned) => ParseResult::Match(spanned, input.advance()),
        }
    }
}

pub fn just<I: PartialEq + Copy>(pattern: I) -> Just<I> {
    Just(pattern)
}

pub struct AndThen<First, Second> {
    first: First,
    second: Second,
}

impl<First, Second> Parser for AndThen<First, Second>
where
    First: Parser,
    First::Input: Clone,
    First::Error: Clone,
    Second: Parser<Input = First::Input, Error = First::Error>,
{
    type Input = First::Input;
    type Output = (Spanned<First::Output>, Spanned<Second::Output>);
    type Error = First::Error;

    fn run(self, input: Stream<Self::Input>) -> ParserReturn<Self> {
        match self.first.run(input.clone()) {
            ParseResult::Error(err) => ParseResult::Error(err),
            ParseResult::NoMatch => ParseResult::NoMatch,
            ParseResult::Match(first, rem) => match self.second.run(rem) {
                ParseResult::Error(err) => ParseResult::Error(err),
                ParseResult::NoMatch => ParseResult::NoMatch,
                ParseResult::Match(second, rem) => {
                    let first_span = first.span;
                    let second_span = second.span;
                    ParseResult::Match(
                        Spanned::new((first, second), Span::from_pair(first_span, second_span)),
                        rem,
                    )
                }
            },
        }
    }
}

pub struct Choice<P> {
    choices: Vec<P>,
}

impl<P> Parser for Choice<P>
where
    P: Parser,
{
    type Input = P::Input;
    type Output = P::Output;
    type Error = P::Error;

    fn run(self, input: Stream<Self::Input>) -> ParserReturn<Self> {
        todo!()
        //     for choice in self.choices {
        //         match choice.run(input) {
        //             ParseResult::Match(spanned, rem) => return ParseResult::Match(spanned, rem),
        //             ParseResult::NoMatch => (),
        //             ParseResult::Error(err) => return ParseResult::Error(err),
        //         }
        //     }

        //     ParseResult::NoMatch
    }
}
