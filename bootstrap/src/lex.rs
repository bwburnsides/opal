use combinators::Stream;
use std::collections::HashMap;
use std::hash::Hash;

use crate::model::token;
use crate::model::token::{Basic, Token};

#[derive(Hash, PartialEq, Eq, Clone)]
enum LexerInput {
    Just(char),        // Match exactly.
    Else, // Match anything if other transitions are invalid. Don't consume current token.
    Any(&'static str), // Match any character in set.
    Eat,  // Match anything if other transitions are invalid. Do consume current token.
}

struct Transition<State, Input, Output> {
    state: State,
    input: Input,
    target: State,
    output: Option<Output>,
}

impl<State, Input, Output> Transition<State, Input, Output> {
    pub const fn new(state: State, input: Input, target: State, output: Option<Output>) -> Self {
        Self {
            state: state,
            input: input,
            target: target,
            output: output,
        }
    }
}

type LexerTransition = Transition<LexerState, LexerInput, Token>;

#[derive(Hash, PartialEq, Eq, Clone)]
enum LexerState {
    Accepting,
    RefineHyphen,
    RefineEqual,
    RefineExclamation,
    RefineColon,
    RefineBar,
    RefinePlus,
    RefineAmpersand,
    RefineAsterisk,
    RefineFSlash,
    RefineLAngle,
    RefineLAngle2,
    RefineRAngle,
    RefineRAngle2,
    ExpectCharacter,
    CollectCharacters,
    RefineZeroDigit,
    CollectHexDigits,
    CollectBinDigits,
    ConsumeComment,
}

type StateMachineTable = HashMap<LexerState, HashMap<char, (LexerState, Option<Token>)>>;

struct Lexer {
    state: LexerState,
    table: StateMachineTable,
}

impl Lexer {
    fn new(table: StateMachineTable) -> Self {
        Lexer {
            state: LexerState::Accepting,
            table: table,
        }
    }

    fn process(&mut self, source: String) {
        let mut input = source.chars().peekable();

        let transitions = self.table.entry(self.state).or_default();
        // self.table.entry(key).or_default().entry(input.peek())

        let head = input.peek();
    }
}

fn build_table(transitions: [LexerTransition; 65]) -> StateMachineTable {
    // let mut table: StateMachineTable = HashMap::new();

    // for transition in transitions {
    //     let state_transitions = table.entry(transition.state).or_insert(HashMap::new());

    //     match transition.input {
    //         LexerInput::Just(ch) => state_transitions.insert(ch, (transition.target, transition.output)),

    //     };
    //     state_transitions.insert(transition.input, (transition.target, transition.output));
    // };

    // table

    todo!()
}

pub fn tokenize(_source: String) -> Stream<Token> {
    use token::Basic::*;
    use LexerInput::*;
    use LexerState::*;
    use Token::*;

    let timber_resources: HashMap<LexerState, HashMap<_, _>> = [
        (
            Accepting,
            [
                (Just(' '), (Accepting, None)),
                (Just('#'), (ConsumeComment, None)),
                (Just('('), (Accepting, Some(Basic(LParen)))),
                (Just(')'), (Accepting, Some(Basic(RParen)))),
                (Just('{'), (Accepting, Some(Basic(LBrace)))),
                (Just('}'), (Accepting, Some(Basic(RBrace)))),
                (Just('['), (Accepting, Some(Basic(LBrack)))),
                (Just(']'), (Accepting, Some(Basic(RBrack)))),
                (Just('-'), (RefineHyphen, None)),
                (Just('='), (RefineEqual, None)),
                (Just('\n'), (Accepting, Some(Basic(Newline)))),
                (Just(','), (Accepting, Some(Basic(Comma)))),
                (Just(';'), (Accepting, Some(Basic(Semicolon)))),
                (Just('?'), (Accepting, Some(Basic(Question)))),
                (Just('.'), (Accepting, Some(Basic(Period)))),
                (Just('!'), (RefineExclamation, None)),
                (Just(':'), (RefineColon, None)),
                (Just('|'), (RefineBar, None)),
                (Just('+'), (RefinePlus, None)),
                (Just('&'), (RefineAmpersand, None)),
                (Just('*'), (RefineAsterisk, None)),
                (Just('/'), (RefineFSlash, None)),
                (Just('<'), (RefineLAngle, None)),
                (Just('>'), (RefineRAngle, None)),
                (Just('\''), (ExpectCharacter, None)),
                (Just('"'), (CollectCharacters, None)),
                (Just('0'), (RefineZeroDigit, None)),
            ]
            .iter()
            .cloned()
            .collect(),
        ),
        // (
        //     ConsumeComment,
        //     [
        //         (Just('\n'), Accepting),
        //         (Eat,        ConsumeComment),
        //     ].iter().cloned().collect()
        // ),
        // (
        //     RefineHyphen,
        //     [

        //     ].iter().cloned().collect()
        // ),
    ]
    .iter()
    .cloned()
    .collect();

    const TRANSITIONS: [LexerTransition; 65] = [
        LexerTransition::new(Accepting, Just(' '), Accepting, None),
        LexerTransition::new(Accepting, Just('#'), ConsumeComment, None),
        LexerTransition::new(Accepting, Just('('), Accepting, Some(Basic(LParen))),
        LexerTransition::new(Accepting, Just(')'), Accepting, Some(Basic(RParen))),
        LexerTransition::new(Accepting, Just('{'), Accepting, Some(Basic(LBrace))),
        LexerTransition::new(Accepting, Just('}'), Accepting, Some(Basic(RBrace))),
        LexerTransition::new(Accepting, Just('['), Accepting, Some(Basic(LBrack))),
        LexerTransition::new(Accepting, Just(']'), Accepting, Some(Basic(RBrack))),
        LexerTransition::new(Accepting, Just('-'), RefineHyphen, None),
        LexerTransition::new(Accepting, Just('='), RefineEqual, None),
        LexerTransition::new(Accepting, Just('\n'), Accepting, Some(Basic(Newline))),
        LexerTransition::new(Accepting, Just(','), Accepting, Some(Basic(Comma))),
        LexerTransition::new(Accepting, Just(';'), Accepting, Some(Basic(Semicolon))),
        LexerTransition::new(Accepting, Just('?'), Accepting, Some(Basic(Question))),
        LexerTransition::new(Accepting, Just('.'), Accepting, Some(Basic(Period))),
        LexerTransition::new(Accepting, Just('!'), RefineExclamation, None),
        LexerTransition::new(Accepting, Just(':'), RefineColon, None),
        LexerTransition::new(Accepting, Just('|'), RefineBar, None),
        LexerTransition::new(Accepting, Just('+'), RefinePlus, None),
        LexerTransition::new(Accepting, Just('&'), RefineAmpersand, None),
        LexerTransition::new(Accepting, Just('*'), RefineAsterisk, None),
        LexerTransition::new(Accepting, Just('/'), RefineFSlash, None),
        LexerTransition::new(Accepting, Just('<'), RefineLAngle, None),
        LexerTransition::new(Accepting, Just('>'), RefineRAngle, None),
        LexerTransition::new(Accepting, Just('\''), ExpectCharacter, None),
        LexerTransition::new(Accepting, Just('"'), CollectCharacters, None),
        LexerTransition::new(Accepting, Just('0'), RefineZeroDigit, None),
        LexerTransition::new(ConsumeComment, Just('\n'), Accepting, None),
        LexerTransition::new(ConsumeComment, Eat, ConsumeComment, None),
        LexerTransition::new(RefineHyphen, Else, Accepting, Some(Basic(Hyphen))),
        LexerTransition::new(RefineHyphen, Just('='), Accepting, Some(Basic(HyphenEqual))),
        LexerTransition::new(RefineHyphen, Just('>'), Accepting, Some(Basic(RLightArrow))),
        LexerTransition::new(RefineEqual, Else, Accepting, Some(Basic(Equal))),
        LexerTransition::new(RefineEqual, Just('='), Accepting, Some(Basic(Equal2))),
        LexerTransition::new(RefineEqual, Just('>'), Accepting, Some(Basic(RHeavyArrow))),
        LexerTransition::new(RefineExclamation, Else, Accepting, Some(Basic(Exclamation))),
        LexerTransition::new(
            RefineExclamation,
            Just('='),
            Accepting,
            Some(Basic(ExclamationEqual)),
        ),
        LexerTransition::new(RefineColon, Else, Accepting, Some(Basic(Colon))),
        LexerTransition::new(RefineColon, Just(':'), Accepting, Some(Basic(Colon2))),
        LexerTransition::new(RefineBar, Else, Accepting, Some(Basic(Bar))),
        LexerTransition::new(RefineBar, Just('|'), Accepting, Some(Basic(Bar2))),
        LexerTransition::new(RefineBar, Just('='), Accepting, Some(Basic(BarEqual))),
        LexerTransition::new(RefinePlus, Else, Accepting, Some(Basic(Plus))),
        LexerTransition::new(RefinePlus, Just('='), Accepting, Some(Basic(PlusEqual))),
        LexerTransition::new(RefineAmpersand, Else, Accepting, Some(Basic(Ampersand))),
        LexerTransition::new(
            RefineAmpersand,
            Just('&'),
            Accepting,
            Some(Basic(Ampersand2)),
        ),
        LexerTransition::new(
            RefineAmpersand,
            Just('='),
            Accepting,
            Some(Basic(AmpersandEqual)),
        ),
        LexerTransition::new(RefineAsterisk, Else, Accepting, Some(Basic(Asterisk))),
        LexerTransition::new(
            RefineAsterisk,
            Just('='),
            Accepting,
            Some(Basic(AsteriskEqual)),
        ),
        LexerTransition::new(RefineFSlash, Else, Accepting, Some(Basic(FSlash))),
        LexerTransition::new(RefineFSlash, Just('='), Accepting, Some(Basic(FSlashEqual))),
        LexerTransition::new(RefineLAngle, Else, Accepting, Some(Basic(LAngle))),
        LexerTransition::new(RefineLAngle, Just('<'), RefineLAngle2, None),
        LexerTransition::new(RefineLAngle, Just('='), Accepting, Some(Basic(LAngleEqual))),
        LexerTransition::new(
            RefineLAngle2,
            Just('<'),
            Accepting,
            Some(Basic(LAngle2Equal)),
        ),
        LexerTransition::new(RefineLAngle2, Else, Accepting, Some(Basic(LAngle2))),
        LexerTransition::new(RefineRAngle, Else, Accepting, Some(Basic(RAngle))),
        LexerTransition::new(RefineRAngle, Just('>'), RefineRAngle2, None),
        LexerTransition::new(RefineRAngle, Just('='), Accepting, Some(Basic(LAngleEqual))),
        LexerTransition::new(
            RefineRAngle2,
            Just('>'),
            Accepting,
            Some(Basic(RAngle2Equal)),
        ),
        LexerTransition::new(RefineRAngle2, Else, Accepting, Some(Basic(RAngle2))),
        LexerTransition::new(RefineZeroDigit, Just('x'), CollectHexDigits, None),
        LexerTransition::new(RefineZeroDigit, Just('b'), CollectBinDigits, None),
        LexerTransition::new(RefineZeroDigit, Any("123456789"), Accepting, Some(Poison)),
        LexerTransition::new(
            RefineZeroDigit,
            Else,
            Accepting,
            Some(Literal(token::Literal::Integer(0))),
        ),
    ];

    let table = build_table(TRANSITIONS);
    Lexer::new(table);

    todo!()
}
