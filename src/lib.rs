#[derive(Debug, PartialEq, Eq)]
pub enum Term {
    Bool(bool),
    Null,
    Hole,
    Id(String),
    Int(isize),
    Variable(String),
    Whitespace,
    Wildcard,
    Word(String),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Terms(Vec<Term>);

/// Construct a `Terms` struct-- this macro implicitly
/// uses the `Term` enum variants, to make it simpler to
/// write.
#[allow(unused_macros)]
macro_rules! ts {
    ($($t:expr),*) => {{
        #[allow(unused_imports)]
        use Term::*;
        Terms(vec![$($t,)*])
    }};
}

mod grammar {
    use super::{Term, Terms};
    use std::str::FromStr;
    include!(concat!(env!("OUT_DIR"), "/living_room_grammar.rs"));
    #[allow(dead_code)]
    pub fn parse(s: &str) -> Result<Terms, ParseError> {
        fact_or_pattern(s)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    macro_rules! assert_ts {
        ($e: expr => [ $($t:expr),* ])  => {
            assert_eq!(
                ts![ $($t),* ],
                grammar::parse($e).unwrap()
            );
        }
    }

    macro_rules! test_ts {
        ($( $n:ident [ $($e: expr => [ $($t:expr),* ]),* ] ),*) => {
            $(
                #[test]
                fn $n() {
                    $(assert_ts!($e => [ $($t),* ]);)*
                }
            )*
        }
    }

    fn ws() -> Term {
        Term::Whitespace
    }

    fn word(s: &str) -> Term {
        Term::Word(s.to_owned())
    }

    fn id(s: &str) -> Term {
        Term::Id(s.to_owned())
    }

    fn var(s: &str) -> Term {
        Term::Variable(s.to_owned())
    }

    test_ts!(
        test_simple_word [
            "hi" => [ word("hi") ],
            "hi   you" => [ word("hi"), ws(), word("you") ]
        ],
        test_ids [
            "#hi" => [ id("hi") ]
        ],
        test_values [
            "0" => [ Int(0) ],
            "1123" => [ Int(1123) ],
            "-10" => [ Int(-10) ], "+1" => [ Int(1) ],
            "true" => [ Bool(true) ],
            "false" => [ Bool(false) ],
            "null" => [ Null ],
            "candy is null" => [ word("candy"), ws(), word("is"), ws(), Null ]
        ],
        test_vars [
            "gorog is at $x $y but _ sometimes $" => [
                word("gorog"), ws(), word("is"), ws(), word("at"), ws(),
                var("x"), ws(), var("y"), ws(), word("but"), ws(),
                Hole, ws(), word("sometimes"), ws(), Wildcard
            ]
        ]
    );

}
