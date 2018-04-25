extern crate serde;
#[macro_use] extern crate serde_derive;
#[cfg(test)] #[macro_use] extern crate serde_json;
#[cfg(not(test))] extern crate serde_json;

/// The structure of the `Term` enum is symmetric to the representation of the
/// parsed JSON structure that we want to fullfil the contract of the parser.
///
/// There are enum variants that in terms of a *type system* don't make that much
/// sense, but it's simpler to have *one* place to create these values and not
/// have intermediate structs/representations in _this_ code when we can have
/// `serde` take care of all of that.
///
/// Weird cases include:
///
/// 1. The `Null` variant should *always* have `value: None` which will get
/// serialized to `null`.
///
/// 2. The `Hole` and `Wildcard` variants will just have `key` => `true`.
///
/// ---
///
/// You might be asking yourself, why not have the parser create `serde_json::Value`s
/// directly as we do in the tests? This *might* be more light-weight specifically
/// since it won't do any *heap* memory allocations as it borrows strings from the
/// parser instead of copy/cloning them... specifically for the variants that hold
/// strings.
#[derive(Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Term<'a> {
    Bool {
        value: bool
    },
    Float {
        value: f64
    },
    Hole {
        hole: bool
    },
    Id {
        id: &'a str
    },
    Int {
        value: i64
    },
    Null {
        value: Option<u8>
    },
    String {
        value: &'a str
    },
    Variable {
        variable: &'a str
    },
    Wildcard {
        wildcard: bool
    },
    Word {
        word: &'a str
    }
}

#[inline(always)]
pub fn parse<'a>(input: &'a str) -> Result<Vec<Term<'a>>, grammar::ParseError> {
    grammar::parse(input)
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
        let parsed = grammar::parse(s).unwrap();
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
                Term::Bool { value: false },
                Term::Float { value: 10.0 },
                Term::Hole { hole: true },
                Term::Id { id: "ay" },
                Term::Int { value: 123 },
                Term::Null { value: None },
                Term::String { value: "hi" },
                Term::Variable { variable: "sup" },
                Term::Word { word: " " },
                Term::Wildcard { wildcard: true },
                Term::Word { word: "banana" }
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
