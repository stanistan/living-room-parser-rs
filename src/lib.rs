extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

mod term_json;
use term_json::ReprJSON;

#[derive(Debug, PartialEq)]
pub enum Term<'a> {
    Bool(bool),
    Float(f64),
    Hole,
    Id(&'a str),
    Int(i64),
    Null,
    String(&'a str),
    Variable(&'a str),
    Whitespace,
    Wildcard,
    Word(&'a str),
}

impl <'a> Term<'a> {
    pub fn repr_json(&'a self) -> ReprJSON<'a> {
        use Term::*;
        match self {
            Bool(ref b) => ReprJSON::bool_value(*b),
            Float(ref f) => ReprJSON::float_value(*f),
            Hole => ReprJSON::hole(),
            Id(ref s) => ReprJSON::id(s),
            Int(ref i) => ReprJSON::int_value(*i),
            Term::Null => ReprJSON::null_value(),
            String(ref s) => ReprJSON::string_value(s),
            Variable(ref v) => ReprJSON::variable(v),
            Term::Whitespace => ReprJSON::word(" "),
            Term::Wildcard => ReprJSON::wildcard(),
            Word(ref w) => ReprJSON::word(w),
        }
    }
}

use serde::ser::{Serialize, Serializer};
impl <'a> Serialize for Term<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        self.repr_json().serialize(serializer)
    }
}

mod grammar {
    use super::Term;
    use std::str::FromStr;
    include!(concat!(env!("OUT_DIR"), "/living_room_grammar.rs"));
}

#[cfg(test)]
mod tests {

    use super::*;

    fn parse_to_serde_value(s: &str) -> serde_json::Value {
        let terms = grammar::parse(s).unwrap();
        let parsed: Vec<_> = terms.iter().map(|s| s.repr_json()).collect();
        let parsed_as_json = serde_json::to_string(&parsed).unwrap();
        serde_json::from_str(&parsed_as_json).unwrap()
    }

    fn ws() -> serde_json::Value {
        word(" ")
    }

    fn word(s: &str) -> serde_json::Value {
        json!({ "word": s })
    }

    fn id(s: &str) -> serde_json::Value {
        json!({ "id": s })
    }

    fn var(s: &str) -> serde_json::Value {
        json!({ "variable": s })
    }

    fn value<T: serde::ser::Serialize>(v: T) -> serde_json::Value {
        json!({ "value": v })
    }

    fn null() -> serde_json::Value {
        json!({ "value": null })
    }

    fn wildcard() -> serde_json::Value {
        json!({ "wildcard": true })
    }

    fn hole() -> serde_json::Value {
        json!({ "hole": true })
    }

    /// Generates groups of named tests that validate an input string
    /// against the generated json (and not an intermediate type).
    ///
    /// The expression gets parsed by the parser, then serialized, and
    /// unserialized into JSON so we can do the comparison against
    /// the final output.
    macro_rules! test_ts {
        ($( $n:ident [ $($e: expr => [ $($t:expr),* ]),* ] ),*) => {
            $(
                #[test]
                fn $n() {
                    $({
                        let unparsed = parse_to_serde_value($e);
                        assert_eq!(json!([ $($t),*]), unparsed);
                    })*
                }
            )*
        }
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
            "0" => [ value(0) ],
            "1123" => [ value(1123) ],
            "-10" => [ value(-10) ], "+1" => [ value(1) ],
            "true" => [ value(true) ],
            "false" => [ value(false) ],
            "null" => [ null() ],
            "candy is null" => [ word("candy"), ws(), word("is"), ws(), null() ]
        ],
        test_vars [
            "gorog is at $x $y but _ sometimes $ 1" => [
                word("gorog"), ws(), word("is"), ws(), word("at"), ws(),
                var("x"), ws(), var("y"), ws(), word("but"), ws(),
                hole(), ws(), word("sometimes"), ws(), wildcard(), ws(), value(1)
            ]
        ],
        test_oddities [
            "a_" => [ word("a"), hole()],
            ",,," => [ word(",,,") ],
            "a,y" => [ word("a,y") ],
            "hi. you" => [ word("hi."), ws(), word("you") ],
            "hi1" => [ word("hi"), value(1) ],
            "hi.1" => [ word("hi"), value(0.1) ],
            "#" => [ id("") ]
        ],
        test_coords [
            "($a, $b)" => [ word("("), var("a"), word(","), ws(), var("b"), word(")") ]
        ],
        test_nums [
            "1 10 1.2" => [ value(1), ws(), value(10), ws(), value(1.2) ],
            "0.1" => [ value(0.1) ],
            ".1" => [ value(0.1) ]
        ],
        test_strings [
            "\"aay\"" => [ value("aay") ],
            // "w\"" => [ word("w\"") ], FIXME SHOULD PANIC?
            "w\"\"" => [ word("w"), value("") ]
        ]
    );


    /// Make sure we're serializing values to JSON the correct way.
    #[test]
    fn test_serialize_value() {

        macro_rules! assert_json {
            ([$($e:expr),*] => $t:tt) => {
                assert_eq!(
                    format!("{}", json!($t)),
                    serde_json::to_string(&vec![ $($e),* ]).unwrap()
                );
            }
        }

        assert_json!(
            [
                Term::Bool(false),
                Term::Float(10.0),
                Term::Hole,
                Term::Id("ay"),
                Term::Int(123),
                Term::Null,
                Term::String("hi"),
                Term::Variable("sup"),
                Term::Whitespace,
                Term::Wildcard,
                Term::Word("banana")
            ]
            =>
            [
                { "value": false },
                { "value": 10.0 },
                { "hole": true },
                { "id": "ay" },
                { "value": 123 },
                { "value": null },
                { "value": "hi" },
                { "variable": "sup" },
                { "word": " " },
                { "wildcard": true },
                { "word": "banana" }
            ]
        );
    }

}
